use crate::bytecode::*;
use crate::MethodInfo;

enum LocalVariable {
    Boolean(bool),
    Byte(i8),
    Char(u32),
    Short(i16),
    Int(i32),
    Float(f32),
    Reference(usize),
    ReturnAddress(usize),
    // use pair to hold long or double
}

struct Operand {}

pub struct OperandStack {
    // long and double contribute two units to the depth. the rest only one
    values: Vec<LocalVariable>,
}

impl OperandStack {
    pub(crate) fn pop(&self) -> Operand {
        todo!()
    }
}

impl OperandStack {
    fn new(method: &Method) -> Self {
        // the length of the operands stack is determined by the bytecode
        todo!()
    }

    fn push(self, variable: LocalVariable) {
        // loads value or constant into the stack. it can also push
        // here we also prepare parameters for called methods and receive method results
    }

    fn pop_int(self) -> LocalVariable::Int {
        unimplemented!()
    }
}

type MethodCallParameters = ();
type Method = MethodInfo;

pub struct Frame {
    variables: Vec<LocalVariable>,
    operand_stack: OperandStack,
    run_time_constant_pool: (),
    // run-time constant pool of the class of the current method
    pub method: Method,
}


impl Frame {
    fn fill_variables(method: &Method, parameters: MethodCallParameters) -> Vec<LocalVariable> {
        // on method, the variable 0 is reference to this
        todo!()
    }
}

impl Frame {
    fn new(method: Method, parameters: MethodCallParameters) -> Self {
        let variables: Vec<LocalVariable> = Frame::fill_variables(&method, parameters);
        let operand_stack = OperandStack::new(&method);
        Self {
            variables,
            operand_stack,
            run_time_constant_pool: (),
            method,
        }
    }

    fn execute(self, operation: Operation) {
        match operation {
            IADD => self.execute_iadd(),
            _ => {}
        }
    }

    fn execute_iadd(self) {
        let i1pop = self.operand_stack.pop_int();
        let i2pop = self.operand_stack.pop_int();
        if let LocalVariable::Int(i1) = i1pop {
            if let LocalVariable::Int(i2) = i2pop {
                self.operand_stack.push(LocalVariable::Int(i1 + i2));
            }
        }
    }
}
