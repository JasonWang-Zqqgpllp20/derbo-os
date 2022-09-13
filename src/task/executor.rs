use super::{Task, TaskId};
use alloc::{collections::BTreeMap, sync::Arc, task::Wake};
use core::task::{Waker, Context, Poll};
use crossbeam_queue::ArrayQueue;

pub struct Executor {
    tasks: BTreeMap<TaskId, Task>,
    task_queue: Arc<ArrayQueue<TaskId>>,
    waker_cache: BTreeMap<TaskId, Waker>, 
}

impl Executor {
    pub fn new() -> Self {
        Executor {
            tasks: BTreeMap::new(),         // contain actual Task instances with a BTreeMap
            task_queue: Arc::new(ArrayQueue::new(1000)), // offer reference counting which works by heap
            waker_cache: BTreeMap::new(),   // store non-repeated Wakers by TaskId index
        }
    }

    pub fn spawn(&mut self, task: Task) {
        let task_id = task.id;
        if self.tasks.insert(task.id, task).is_some() {
            panic!("task with same ID already in tasks");
        }
        self.task_queue.push(task_id).expect("queue full");
    }

    fn run_ready_tasks(&mut self) -> bool {
        let Self {      // destructure to avoid borrow checker errors
            tasks,
            task_queue,
            waker_cache,
        } = self;

        while let Ok(task_id) = task_queue.pop() {
            let task = match tasks.get_mut(&task_id) {
                Some(task) => task,
                None => continue,   // task no longer exists
            };
            let waker = waker_cache
                .entry(task_id)
                .or_insert_with(|| TaskWaker::new(task_id, task_queue.clone()));    // Arc count + 1
            let mut context = Context::from_waker(waker);
            match task.poll(&mut context) {
                Poll::Ready(()) => {
                    tasks.remove(&task_id);
                    waker_cache.remove(&task_id);
                    return true;
                }
                Poll::Pending => {}
            }
        }
        return false;
    }

    pub fn run(&mut self) -> ! {
        loop {
            self.run_ready_tasks();
            self.sleep_if_idle();
        }
    }
    pub fn run_once(&mut self) {
        loop {
            if self.run_ready_tasks() == true {
                break;
            }
            self.sleep_if_idle();
        }
    }

    fn sleep_if_idle(&self) {   // the queue is no longer empty
        use x86_64::instructions::interrupts::{self, enable_and_hlt};

        interrupts::disable();
        if self.task_queue.is_empty() {
            enable_and_hlt();       // make enable interrupts and hlt to a single atomic operation
        } else {
            interrupts::enable();
        }
    }
}

struct TaskWaker {
    task_id: TaskId,
    task_queue: Arc<ArrayQueue<TaskId>>,
}

impl TaskWaker {
    fn new(task_id: TaskId, task_queue: Arc<ArrayQueue<TaskId>>) -> Waker {
        Waker::from(Arc::new(TaskWaker {
            task_id,
            task_queue,
        }))
    }

    fn wake_task(&self) {   // ArrayQueue only needs '&' to modify rather than '&mut'
        self.task_queue.push(self.task_id).expect("task_queue full");
    }
}

impl Wake for TaskWaker {
    /* since wakers are commonly shared between the executor and asynchronous tasks
       the trait methods require that the Self instance is wrapped in the Arc type */
    fn wake(self: Arc<Self>) {  // take ownership of the Arc and require an increase of the reference count
        self.wake_task();
    }

    fn wake_by_ref(self: &Arc<Self>) {  // only requier a reference to Arc, better performence
        self.wake_task();
    }
}