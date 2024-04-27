use core::cmp::Ordering;

use loongarch::{register::{tcfg, cpucfg::Cpucfg}, time};
use crate::sync::UPSafeCell;
use crate::task::{wakeup_task, TaskControlBlock};
use alloc::collections::BinaryHeap;
use alloc::sync::Arc;
use lazy_static::*;

pub fn init_trigger(ticks_per_sec: usize) {
    let stable_counter_freq = Cpucfg::get_sc_freq();
    let trigger_freq = stable_counter_freq / ticks_per_sec;
    let mut tcfg = tcfg::read();
    tcfg.init_trigger(trigger_freq);
}

// pub fn get_time_s() -> usize {
//     let stable_counter_freq = Cpucfg::get_sc_freq();
//     get_time() / stable_counter_freq
// }

pub fn get_time_ms() -> usize {
    let stable_counter_freq = Cpucfg::get_sc_freq();
    get_time() * 1_000 / stable_counter_freq
}

// pub fn get_time_us() -> usize {
//     let stable_counter_freq = Cpucfg::get_sc_freq();
//     get_time() * 1_000_000 / stable_counter_freq
// }

pub fn get_time() -> usize {
    time::get_time()
}
pub struct TimerCondVar {
    pub expire_ms: usize,
    pub task: Arc<TaskControlBlock>,
}

impl PartialEq for TimerCondVar {
    fn eq(&self, other: &Self) -> bool {
        self.expire_ms == other.expire_ms
    }
}
impl Eq for TimerCondVar {}
impl PartialOrd for TimerCondVar {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let a = -(self.expire_ms as isize);
        let b = -(other.expire_ms as isize);
        Some(a.cmp(&b))
    }
}

impl Ord for TimerCondVar {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

lazy_static! {
    static ref TIMERS: UPSafeCell<BinaryHeap<TimerCondVar>> =
        unsafe { UPSafeCell::new(BinaryHeap::<TimerCondVar>::new()) };
}

pub fn add_timer(expire_ms: usize, task: Arc<TaskControlBlock>) {
    let mut timers = TIMERS.exclusive_access();
    timers.push(TimerCondVar { expire_ms, task });
}

pub fn remove_timer(task: Arc<TaskControlBlock>) {
    let mut timers = TIMERS.exclusive_access();
    let mut temp = BinaryHeap::<TimerCondVar>::new();
    for condvar in timers.drain() {
        if Arc::as_ptr(&task) != Arc::as_ptr(&condvar.task) {
            temp.push(condvar);
        }
    }
    timers.clear();
    timers.append(&mut temp);
}

pub fn check_timer() {
    let current_ms = get_time_ms();
    let mut timers = TIMERS.exclusive_access();
    while let Some(timer) = timers.peek() {
        if timer.expire_ms <= current_ms {
            wakeup_task(Arc::clone(&timer.task));
            timers.pop();
        } else {
            break;
        }
    }
}
