use crate::vm::VM;
use std::thread;

#[derive(Default)]
pub struct Scheduler {}

impl Scheduler {
    pub fn new() -> Scheduler {
        Scheduler {}
    }

    pub fn get_thread(vm: VM) {}
}
