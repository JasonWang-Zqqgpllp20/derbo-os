use alloc::vec::Vec;
use alloc::string::String;

pub fn sleep005s(times: u64) {
    use crate::timer::sleep;
    use crate::task::Task;
    use crate::task::executor::Executor;

    let mut executor = Executor::new();
    executor.spawn(Task::new(sleep::sleep_timerfifo_005s(times)));
    executor.run_once();
}
pub fn sleep1s(times: u64) {
    use crate::timer::sleep;
    use crate::task::Task;
    use crate::task::executor::Executor;

    let mut executor = Executor::new();
    executor.spawn(Task::new(sleep::sleep_timerfifo_1s(times)));
    executor.run_once();
}

pub fn char2int(char_vec: Vec<char>) -> Result<u64, ()> {
    let mut sum: u64 = 0;
    let len = char_vec.len();

    for c in char_vec.iter() {
        if *c < 48 as char || *c > 57 as char {     // not number 0~9.
            return Err(())
        }
    }

    fn n_e(n: u64) -> u64 {     // calculate n^10
        let mut ret = 1;
        for _ in 0..n {
            ret *= 10;
        }
        return ret;
    }

    for (i, c) in char_vec.iter().enumerate() {
        sum += (*c as u64 - 48) * n_e( (len-i-1) as u64);
    }

    return Ok(sum);
}

pub fn char_vec_cmp(v1: &Vec<char>, v2: &Vec<char>) -> bool {
    if v1.len() != v2.len() {
        return false;
    }

    for (i, _) in v1.iter().enumerate() {
        if v1[i] != v2[i] {
            return false;
        }
    }

    return true;
}

pub fn str2char(s: &str) -> Vec<char> {
    let v1 = s.as_bytes().to_vec();
    let mut v2: Vec<char> = Vec::new();
    for c in v1 {
        v2.push(c as char);
    }
    v2
}

pub fn code2stringvec(char_vec: Vec<Vec<char>>) -> Vec<String> {
    
    let mut stringvec = Vec::new();
    for line in char_vec {
        let s = line.iter().collect::<String>();
        stringvec.push(s);
    }

    stringvec
}