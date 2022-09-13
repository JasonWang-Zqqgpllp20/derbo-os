use crate::{print, println};
use alloc::vec::Vec;

pub fn execute(command: &str) {
    match command {
        "hello" => cmd_hello(),
        "clear" => cmd_clear(),
        // TODO: append other handler functions here
        _ => (),
    }
}

pub fn execute_para(command: &str, para: Vec<char>) {
    match command {
        "echo" => cmd_echo(para),
        // TODO: append other handler functions here
        _ => (),
    }
}

pub fn cmd_hello() {
    println!("Hello, world!");
}

pub fn cmd_clear() {
    let c = 0x02 as char;
    print!("{}", c);
}

pub fn cmd_echo(para: Vec<char>) {
    for c in para.iter() {
        print!("{}", *c);
    }
    println!("");
}