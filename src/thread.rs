use crate::frames::Frame;
use crate::MethodInfo;
use crate::stack::Stack;

pub type Method = MethodInfo;

pub struct Thread {
    current_method: Method,
    pc: usize,
    stack: Stack
}

impl Thread {
    fn current_frame(self) -> Frame {
        self.stack.current_frame()
    }

    fn run_method(self, method: Method) {
        if self.stack.has_space_for(method) {

        } else {
            // throw Stack Overflow Error
        }
    }
}
