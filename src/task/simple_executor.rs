use super::Task;
use alloc::collections::VecDeque;
use core::task::{Waker, RawWaker, RawWakerVTable, Context, Poll};

pub struct SimpleExecutor {
    task_queue: VecDeque<Task>,
}

impl SimpleExecutor {
    pub fn new() -> SimpleExecutor {
        SimpleExecutor {
            task_queue: VecDeque::new(),    // a bidirection queue, we use it as a FIFO
        }
    }

    pub fn spawn(&mut self, task: Task) {
        self.task_queue.push_back(task)
    }

    pub fn run(&mut self) {
        while let Some(mut task) = self.task_queue.pop_front() {    // pop from the front
            let waker = dummy_waker();                  // return a raw waker
            let mut context = Context::from_waker(&waker);  // creatign a context by wrapping a waker

            match task.poll(&mut context) {
                Poll::Ready(()) => {}
                Poll::Pending => self.task_queue.push_back(task), // push from the back
            }
        }
    }
}

fn dummy_raw_waker() -> RawWaker {
    fn no_op(_: *const ()) {}               // do nothing
    fn clone(_: *const ()) -> RawWaker {    // call dummy_raw_waker again
        dummy_raw_waker()
    }

    let vtable = &RawWakerVTable::new(clone, no_op, no_op, no_op);  // create a minimal RawWakerVTable
    RawWaker::new(0 as *const (), vtable)   // pass a raw pointer and return a RawWaker
}

fn dummy_waker() -> Waker {
    unsafe { 
        Waker::from_raw(dummy_raw_waker()) // create a new Waker from a raw waker
    }
}