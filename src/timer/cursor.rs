use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;
use crate::{print, println};
use core::{pin::Pin, task::{Poll, Context}};
use futures_util::stream::{Stream, StreamExt};
use futures_util::task::AtomicWaker;

static TIMER_FIFO: OnceCell<ArrayQueue<u64>> = OnceCell::uninit();   // use a OnceCell to wrap it to initialize at compile time rather than using ArrayQeueu::new()
static WAKER: AtomicWaker = AtomicWaker::new();

pub struct TimerFifoStream {
    _private: (),
}

impl TimerFifoStream {
    pub fn new() -> Self {
        TIMER_FIFO.try_init_once(|| ArrayQueue::new(100))
            .expect("ScancodeStream::new should only be called once");
        TimerFifoStream { _private: () }
    }
}

impl Stream for TimerFifoStream {
    type Item = u64;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<u64>> {
        let queue = TIMER_FIFO.try_get().expect("not initialized");

        if let Ok(fifodata) = queue.pop() { // avoid the performance overhead if it's successful to pop
            return Poll::Ready(Some(fifodata));
        }
        WAKER.register(&cx.waker());        // register a Waker
        match queue.pop() {                 // pop again
            Ok(fifodata) =>{
                WAKER.take();               // Waker no longer needed
                Poll::Ready(Some(fifodata))
            }
            Err(crossbeam_queue::PopError) => Poll::Pending,
        }
    }
}

/* called by the keyboard interrupt handler, must not block or allocate */
pub(crate) fn timerfifo_push(fifodata: u64) { // shouldn't be callable from main.rs make it only available to lib.rs
    if let Ok(queue) = TIMER_FIFO.try_get() {   // get a reference to the initialized queue
        if let Err(_) = queue.push(fifodata) {
            println!("WARNING: timerfifo full; dropping timer input");
        } else {
            WAKER.wake();   // call wake() if the push to the scancode queue succeeds to notify the executor
        }
    } else {
        // println!("WARNING: timerfifo uninitialized");
    }
}

pub async fn print_timerfifo() {
    let mut fifodata = TimerFifoStream::new();

    while let Some(data) = fifodata.next().await {
        match data {
            0x01 => {                   // use an ascii of 0x01 to tell the WRITER to blink the cursor
                let c = 0x01 as char;
                print!("{}", c);
            },
            // _ => ()
            _ => println!("{}", data)
        }
    }
}