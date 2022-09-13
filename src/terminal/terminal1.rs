use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;
use crate::{print, println};
use core::{pin::Pin, task::{Poll, Context}};
use futures_util::stream::{Stream, StreamExt};
use futures_util::task::AtomicWaker;
use pc_keyboard::{layouts, DecodedKey, KeyCode, HandleControl, Keyboard, ScancodeSet1};
use crate::terminal::controller::TerminalController;
use crate::terminal::SwitchState;
use crate::file::{FileNode, FileType};
use crate::file::file_system::FileSystem;
use alloc::vec::Vec;
use super::task1;
use alloc::format;
use alloc::string::String;

static SCANCODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();   // use a OnceCell to wrap it to initialize at compile time rather than using ArrayQeueu::new()
static WAKER: AtomicWaker = AtomicWaker::new();
use crate::buffer::keyboard::{SWITCH, EDIT_CHAR, EDIT_ENTER};
use crate::buffer::vga_buffer::{IF_ESC, EDIT_UP, EDIT_DOWN};
pub static mut PRINTING_1: bool = false;
pub static mut TASKING_1: bool = false;
pub static mut SHIFT_OFFSET_1: usize = 0;
pub static mut EDITING_1: bool = false;

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
    use super::command;
    use crate::timer::sleep;
    use alloc::vec;
    use crate::compiler::compile;

    let mut scancodes = ScancodeStream::new();
    let mut keyboard = Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore);

    let mut terminal_ctl1 = TerminalController::new();
    terminal_ctl1.init();

    let mut file_system = FileSystem::new();
    file_system.init();

    while let Some(scancode) = scancodes.next().await {     // a while loop which make CPU work all the time
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let Some(key) = keyboard.process_keyevent(key_event) {
                match key {
                    DecodedKey::Unicode(character) => { // normal characters
                        match character {
                            '\n' => {
                                unsafe {
                                    TASKING_1 = true;
                                    if SWITCH == SwitchState::Terminal1 { // try to run command in terminal1
                                        match terminal_ctl1.retrieve() {
                                            Ok( (comm, para) ) => {                   // found a command
                                                if comm == "sleep" { // write here because we use 'await'
                                                    use crate::api::char2int;

                                                    let res = char2int(para);
                                                    match res {
                                                        Ok(n) => {
                                                            for i in 0..n {
                                                                // sleep::sleep_timerfifo_1s(1).await;
                                                                // PRINTING_1 = true;
                                                                // println!("{}s", i+1);
                                                                // PRINTING_1 = false;
                                                                // if IF_ESC == true { 
                                                                //     IF_ESC = false;
                                                                //     println!("Interrupt by keypress 'Esc'");
                                                                //     break
                                                                // }
                                                                let time = format!("{}", i+1);
                                                                task1::add_command((String::from("sleep"), time.clone()));
                                                                task1::add_command((String::from("println"), time));
                                                            }
                                                        },
                                                        Err(_) => {
                                                            PRINTING_1 = true;
                                                            println!("Parameter must be number");
                                                            PRINTING_1 = false;
                                                        }
                                                    }
                                                } else if comm == "cd" {
                                                    if para.len() == 0 {
                                                        println!("There must be a parameter");
                                                    } else {
                                                        use crate::api::char_vec_cmp;

                                                        if char_vec_cmp(&para, &vec!['.', '.']) {
                                                            file_system.outof_forlder();
                                                        } else {
                                                            file_system.into_folder(para);
                                                        }
                                                    }
                                                } else if comm == "ls" {
                                                    file_system.list();
                                                } else if comm == "edit" {
                                                    if para.len() == 0 {
                                                        println!("There must be a parameter");
                                                    } else {
                                                        match file_system.read_file(para.clone(), true) {
                                                            Ok(mut content) => {
                                                                EDITING_1 = true;
                                                                SHIFT_OFFSET_1 = content.len();
                                                                print!("{}", 0x03 as char);

                                                                let mut vec_cur = content.len();
                                                                loop {
                                                                    sleep::sleep_timerfifo_005s(1).await;
                                                                    if EDIT_UP == true {
                                                                        if vec_cur != 0 {
                                                                            vec_cur -= 1;
                                                                            EDIT_UP = false;
                                                                        }
                                                                    }

                                                                    if EDIT_DOWN == true {
                                                                        if vec_cur >= content.len() {
                                                                            content.push(Vec::new());
                                                                        }
                                                                        vec_cur += 1;
                                                                        EDIT_DOWN = false;
                                                                    }

                                                                    if EDIT_ENTER == true {
                                                                        content.insert(vec_cur+1, Vec::new());
                                                                        vec_cur += 1;
                                                                        EDIT_ENTER = false;
                                                                    }

                                                                    for (i, b) in EDIT_CHAR.iter().enumerate() {
                                                                        if *b == true {
                                                                            let c = (i as u8 + 32 ) as char;

                                                                            if vec_cur == content.len() {
                                                                                content.push(Vec::new());
                                                                            }

                                                                            if c as u8 != 0x7f {            // normal characters
                                                                                content[vec_cur].push(c);
                                                                            } else {                        // backspace
                                                                                if content[vec_cur].len() == 0 
                                                                                    && vec_cur != 0 {
                                                                                    content.remove(vec_cur);
                                                                                    vec_cur -= 1;
                                                                                }
                                                                                content[vec_cur].pop();
                                                                            }
                                                                        }                                                                        
                                                                    }
                                                                    for i in 0..EDIT_CHAR.len() {
                                                                        EDIT_CHAR[i] = false;
                                                                    }

                                                                    if IF_ESC == true {
                                                                        IF_ESC = false;
                                                                        EDITING_1 = false;
                                                                        // todo: move the cursor to the first empty line on the screen
                                                                        break
                                                                    }
                                                                }
                                                                file_system.edit_file(para, content).unwrap();
                                                                print!("{}", 0x02 as char);
                                                            },
                                                            Err(_) => {}
                                                        }
                                                    }
                                                } else if comm == "run" {
                                                    if para.len() == 0 {
                                                        println!("There must be a parameter");
                                                    } else {
                                                        match file_system.read_file(para.clone(), false) {
                                                            Ok(content) => {
                                                                compile::compile_run(content);
                                                            },
                                                            Err(_) => {}
                                                        }
                                                    }
                                                } else if comm == "mk" {
                                                    if para.len() == 0 {
                                                        println!("There must be a parameter");
                                                    } else {
                                                        match file_system.retrieve(para.clone()) {
                                                            Ok((_, _)) => {
                                                                println!("The file exsits");
                                                            },
                                                            Err(_) => {
                                                                let file = FileNode::new(para.clone(), FileType::Document);
                                                                file_system.add_file(file).unwrap();
                                                            }
                                                        }
                                                    }
                                                } else if comm == "mkdir" {                                              
                                                    if para.len() == 0 {
                                                        println!("There must be a parameter");
                                                    } else {
                                                        match file_system.retrieve(para.clone()) {
                                                            Ok((_, _)) => {
                                                                println!("The file exsits");
                                                            },
                                                            Err(_) => {
                                                                let file = FileNode::new(para.clone(), FileType::Folder);
                                                                file_system.add_file(file).unwrap();
                                                            }
                                                        }
                                                    }
                                                } else if comm == "rm" {                                              
                                                    if para.len() == 0 {
                                                        println!("There must be a parameter");
                                                    } else {
                                                        match file_system.retrieve(para.clone()) {
                                                            Ok((file, _)) => {
                                                                file_system.remove_file(file).unwrap();
                                                            },
                                                            Err(_) => {
                                                                print!("No file called ");
                                                                for c in para {
                                                                    print!("{}", c);
                                                                }
                                                                println!("");
                                                            }
                                                        }
                                                    }
                                                } else {                        // normal command
                                                    if para.len() == 0 {        // command without parameter
                                                        command::execute(comm);
                                                    } else {
                                                        command::execute_para(comm, para);
                                                    }
                                                }
                                            },
                                            Err(_) => {
                                                PRINTING_1 = true;
                                                task1::add_command((String::from("println"), String::from("Invalid command")));
                                                PRINTING_1 = false;
                                            }
                                        }
                                        PRINTING_1 = true;
                                        // file_system.print_path();
                                        task1::add_command((String::from("over"), file_system.get_path()));
                                        PRINTING_1 = false;
                                        terminal_ctl1.clear();
                                    }
                                    TASKING_1 = false;
                                }
                            },
                            '\t' => {},
                            _ => {
                                let ch = character as u8;
                                if ch == 0x08 {                 // backspace
                                    unsafe {
                                        if SWITCH == SwitchState::Terminal1 {
                                            terminal_ctl1.backspace();
                                        }
                                    }
                                } else if ch == 0x1b {                     // Esc
                                    
                                } else {                        // normal characters
                                    unsafe {
                                        if SWITCH == SwitchState::Terminal1 {
                                            terminal_ctl1.pushchar(character);
                                        }
                                    }
                                }
                            }
                        }
                    },
                    DecodedKey::RawKey(key) => {        // other keys on the keyboard
                        match key {
                            KeyCode::F1 => {},    // switch terminal in right cycle?
                            KeyCode::F2 => {},    // switch terminal in left cycle?
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}