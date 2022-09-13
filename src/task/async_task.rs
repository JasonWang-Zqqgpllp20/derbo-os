use crate::{print, println};
use crate::interrupts::TIMER_COUNT;

pub async fn async_task1() {
    let mut s: i64 = 0;
    add_more(&mut s).await;
    unsafe { println!("{}", TIMER_COUNT); }
}

pub async fn add_more(a: &mut i64) {
    for _ in 0..10000000 {
        *a += 1
    }
}

pub async fn async_task2() {
    let mut s: i64 = 0;
    for i in 0..10000 {
        add(&mut s).await;
        println!("{}", i);
    }
    unsafe { println!("{}", TIMER_COUNT); }
}

pub async fn add(a: &mut i64) {
    for _ in 0..1000 {
        *a += 1
    }
}