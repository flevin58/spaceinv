use raylib_ffi as ray;
use std::cell::RefCell;

pub struct Context {
    pub rl: RefCell<RaylibHandle>,
    pub thread: RefCell<RaylibThread>,
}

impl Context {
    pub fn new(rl: RaylibHandle, thread: RaylibThread) -> Self {
        Self {
            rl: RefCell::new(rl),
            thread: RefCell::new(thread),
        }
    }
}
