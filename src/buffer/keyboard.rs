use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;
use crate::{print, println};
use core::{pin::Pin, task::{Poll, Context}};
use futures_util::stream::{Stream, StreamExt};
use futures_util::task::AtomicWaker;
use pc_keyboard::{layouts, DecodedKey, KeyCode, HandleControl, Keyboard, ScancodeSet1};
use crate::terminal::controller::TerminalController;
use crate::terminal::SwitchState;

use crate::terminal::terminal1::EDITING_1;

static SCANCODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();   // use a OnceCell to wrap it to initialize at compile time rather than using ArrayQeueu::new()
pub static mut SWITCH: SwitchState = SwitchState::Terminal1;
pub static mut IF_TAB: bool = false;
pub static mut EDIT_CHAR: [bool; 96] = [false; 96];
pub static mut EDIT_ENTER: bool = false;

static WAKER: AtomicWaker = AtomicWaker::new();

pub struct ScancodeStream {
    _private: (),
}

impl ScancodeStream {
    pub fn new() -> Self {
        SCANCODE_QUEUE.try_init_once(|| ArrayQueue::new(100))
            .expect("ScancodeStream::new should only be called once");
        ScancodeStream { _private: () }
    }
}

impl Stream for ScancodeStream {
    type Item = u8;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<u8>> {
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
pub(crate) fn add_scancode(scancode: u8) { // shouldn't be callable from main.rs make it only available to lib.rs
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

pub async fn print_keypresses() {
    let mut scancodes = ScancodeStream::new();
    let mut keyboard = Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore);

    let mut terminal_ctl1 = TerminalController::new();
    terminal_ctl1.init();
    let mut terminal_ctl2 = TerminalController::new();
    terminal_ctl2.init();

    while let Some(scancode) = scancodes.next().await {     // a while loop which make CPU work all the time
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let Some(key) = keyboard.process_keyevent(key_event) {
                match key {
                    DecodedKey::Unicode(character) => { // normal characters
                        match character {
                            '\n' => {
                                unsafe {
                                    if EDITING_1 {
                                        EDIT_ENTER = true;
                                    }
                                }
                                println!("");
                            },
                            '\t' => {
                                unsafe {
                                    IF_TAB = true;
                                    if SWITCH == SwitchState::Terminal1 {
                                        SWITCH = SwitchState::Terminal2;
                                    } else {
                                        SWITCH = SwitchState::Terminal1;
                                    }
                                }
                                print!("{}", character);
                                unsafe {
                                    IF_TAB = false;
                                }
                            },
                            _ => {
                                unsafe {
                                    if EDITING_1 == true {
                                        let c = character as i32 - 32;
                                        // println!("\n{}", c);
                                        if c == -24 {                   // 'backspace' use a index of 95, which is the position of 'delete' in ascii.
                                            EDIT_CHAR[95] = true;
                                        } else if c>=0 && c<=94 {
                                            EDIT_CHAR[c as usize] = true;
                                        }
                                    }
                                }
                                let ch = character as u8;
                                if ch == 0x08 {                 // backspace
                                    print!("{}", character);
                                } else {                        // normal characters
                                    print!("{}", character);
                                }
                            }
                        }
                    },
                    DecodedKey::RawKey(key) => {        // other keys on the keyboard
                        match key {
                            KeyCode::F1 => {},    // switch terminal in right cycle?
                            KeyCode::F2 => {},    // switch terminal in left cycle?
                            KeyCode::Escape => {},
                            KeyCode::ArrowUp => {
                                print!("{}", 0x04 as char);
                            },
                            KeyCode::ArrowDown => {
                                print!("{}", 0x05 as char);
                            },
                            _ => {}
                        }
                    }
                } // https://docs.rs/pc-keyboard/0.5.1/pc_keyboard/enum.KeyCode.html
            }
        }
    }
}