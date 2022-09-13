use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;
use crate::{print, println};
use core::{pin::Pin, task::{Poll, Context}};
use futures_util::stream::{Stream, StreamExt};
use futures_util::task::AtomicWaker;
use alloc::string::String;

static SCANCODE_QUEUE: OnceCell<ArrayQueue<(String, String)>> = OnceCell::uninit();   // use a OnceCell to wrap it to initialize at compile time rather than using ArrayQeueu::new()
static WAKER: AtomicWaker = AtomicWaker::new();

pub static mut TASK_SHOULD_RUNNING_2: bool = true;

use crate::buffer::vga_buffer::IF_ESC;
use super::terminal2::PRINTING_2;

pub struct ScancodeStream {
    _private: (),
}

impl ScancodeStream {
    pub fn new() -> Self {
        SCANCODE_QUEUE.try_init_once(|| ArrayQueue::new(1000))
            .expect("ScancodeStream::new should only be called once");
        ScancodeStream { _private: () }
    }
}

impl Stream for ScancodeStream {
    type Item = (String, String);

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<(String, String)>> {
        let queue = SCANCODE_QUEUE.try_get().expect("not initialized");

        if let Ok(scancode) = queue.pop() { // avoid the performance overhead if it's successful to pop
            return Poll::Ready(Some(scancode));
        }
        WAKER.register(&cx.waker());        // register a Waker
        match queue.pop() {                 // pop again
            Ok(scancode) =>{
                WAKER.take();               // Waker no longer needed
                Poll::Ready(Some(scancode))
            }
            Err(crossbeam_queue::PopError) => Poll::Pending,
        }
    }
}

/* called by the keyboard interrupt handler, must not block or allocate */
pub(crate) fn add_command(scancode: (String, String)) { // shouldn't be callable from main.rs make it only available to lib.rs
    if let Ok(queue) = SCANCODE_QUEUE.try_get() {   // get a reference to the initialized queue
        if let Err(_) = queue.push(scancode) {
            println!("WARNING: scancode queue full; dropping keyboard input");
        } else {
            WAKER.wake();   // call wake() if the push to the scancode queue succeeds to notify the executor
        }
    } else {
        println!("WARNING: scancode queue uninitialized");
    }
}

pub async fn run_command() {
    use crate::timer::sleep;
    use crate::api;

    let mut scancodes = ScancodeStream::new();

    while let Some(scancode) = scancodes.next().await {
        unsafe {
            let command = scancode.0;
            let parameter = scancode.1;

            if command == "sleep" {
                if TASK_SHOULD_RUNNING_2 == false {
                    continue;
                }

                let time = api::str2char(&parameter);
                let _time = api::char2int(time).unwrap();
                for _ in 0..20 {
                    sleep::sleep_timerfifo_005s(1).await;
                }

                if IF_ESC == true { 
                    IF_ESC = false;
                    PRINTING_2 = true;
                    println!("Interrupt by keypress 'Esc'");
                    PRINTING_2 = false;
                    TASK_SHOULD_RUNNING_2 = false;
                }
            } else if command == "print" {
                if TASK_SHOULD_RUNNING_2 == false {
                    continue;
                }
                
                PRINTING_2 = true;
                print!("{}", parameter);
                PRINTING_2 = false;
            } else if command == "println" {
                if TASK_SHOULD_RUNNING_2 == false {
                    continue;
                }
                
                PRINTING_2 = true;
                println!("{}", parameter);
                PRINTING_2 = false;
            } else if command == "over" {
                TASK_SHOULD_RUNNING_2 = true;
                PRINTING_2 = true;
                print!("{}", parameter);
                PRINTING_2 = false;
            }
        }
    }
}