use crate::bytecode::Operation;
use crate::frames::OperandStack;
use crate::thread::Thread;

pub struct Execution {
    bytecode: Vec<u8>,
    op_counter: usize,
    thread: Thread
}

impl Execution {
    fn current_operand(self) -> Operation {
        self.bytecode[self.op_counter]
    }

    fn execute(self, operand_stack: OperandStack) {
        match self.current_operand() {
            AALOAD => {
                self.thread.current_method()
            }
        }
    }
}
