use core::{future::Future, pin::Pin};
use core::task::{Context, Poll};
use core::sync::atomic::{AtomicU64, Ordering};
use alloc::boxed::Box;

pub mod simple_executor;
pub mod executor;
pub mod async_task;

pub struct Task {
    id: TaskId,
    future: Pin<Box<dyn Future<Output = ()>>>,
    // tasks don't return any result, they are just executed for its side effects, so it returns ()
    // use 'dyn' is because the methods on the future are dynamically dispatched ang there will be different tasks
    // Pin<Box> type avoids the moving of the value in specific status and prevents the creation of &mut reference to it
}

impl Task {
    pub fn new(future: impl Future<Output = ()> + 'static) -> Task {
        Task {
            id: TaskId::new(),
            future: Box::pin(future),   // pin the future in memory through the Box::pin
        }
    }   // the Task can live for an arbitrary time, so the future needs to be valid for that time

    // add a poll method of the Future trait to be called on a Pin<&mut T> type
    fn poll(&mut self, context: &mut Context) -> Poll<()>{  // private, should only be called by executor
        self.future.as_mut().poll(context)  // use Pin::as_mut method to convert the self.future of Pin<Box<T>>
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct TaskId(u64);

impl TaskId {
    fn new() -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);  // static, ensure each ID is assigned only once
                                                        // Atomic, be safely shared between threads
        TaskId(NEXT_ID.fetch_add(1, Ordering::Relaxed)) // atomically increase the value and return
        // Ordering defines how much the compiler is allowed to reorder the fetch_add operation in sstream
    }
}