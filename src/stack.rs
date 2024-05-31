use crate::frames::Frame;
use crate::MethodInfo;

pub struct Stack {
    pub frames: Vec<Frame>,
}

impl Stack {
    fn new_frame(self) {
        unimplemented!("new frame is created")
    }

    pub fn current_frame(self) -> Frame {
        // self.frames.last().unwrap()
        unimplemented!("missing lifetime")
    }

    fn current_method(self) -> MethodInfo {
        self.current_frame().method
    }
}
