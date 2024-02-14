use crate::{
    expr::{Atom, Expression, Operation},
    module::Module,
};

use std::collections::HashMap;

type Ptr = u32;
#[derive(Clone, Copy)]
enum Value {
    Num(usize),
    Papp(Ptr),
}

impl Value {
    fn expect_num(self) -> usize {
        match self {
            Value::Num(x) => x,
            _ => panic!("Expected number"),
        }
    }

    fn expect_papp(self) -> Ptr {
        match self {
            Value::Papp(x) => x,
            _ => panic!("Expected partial application"),
        }
    }
}

struct Frame(HashMap<String, Value>);

impl Frame {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn insert(&mut self, reg: String, val: Value) {
        self.0.insert(reg, val);
    }

    fn get(&self, reg: &str) -> Option<&Value> {
        self.0.get(reg)
    }
}

static INIT_HEAP_SIZE: usize = 1 << 24;
static INIT_STACK_SIZE: usize = 1 << 18;

fn alloc_heap() -> Vec<Value> {
    Vec::with_capacity(INIT_HEAP_SIZE)
}

fn alloc_stack() -> Vec<Frame> {
    Vec::with_capacity(INIT_STACK_SIZE)
}

fn push_within_capacity<T>(mem: &mut Vec<T>, val: T, msg: &str) {
    if mem.len() == mem.capacity() {
        panic!("{}", msg)
    }
    mem.push(val)
}

struct State {
    heap: Vec<Value>,
    stack: Vec<Frame>,
    frame: Frame,
}

impl Operation {
    fn run(&self, x: usize, y: usize) -> usize {
        match self {
            Operation::Add => x + y,
            Operation::Sub => x - y,
            Operation::Mul => x * y,
            Operation::Div => x / y,
            Operation::Mod => x % y,
            Operation::Eq => (x == y) as usize,
            Operation::Lt => (x < y) as usize,
            Operation::Le => (x <= y) as usize,
            Operation::Gt => (x > y) as usize,
            Operation::Ge => (x >= y) as usize,
            Operation::And => x & y,
            Operation::Or => x | y,
            Operation::Xor => x ^ y,
            Operation::Sr => x >> y,
            Operation::Sl => x << y,
        }
    }
}

impl State {
    fn alloc_on_stack(&mut self, reg: Frame) {
        push_within_capacity(&mut self.stack, reg, "Stack has overflown")
    }

    fn alloc_on_heap(&mut self, val: Value) {
        push_within_capacity(&mut self.heap, val, "Memory has run out")
    }

    fn retrieve_atom(&self, atom: &Atom) -> Value {
        match atom {
            Atom::Var(x) => *self.frame.get(x).unwrap(),
            Atom::Lit(x) => Value::Num(*x),
        }
    }

    fn call(&mut self, func: &str, args: &[Atom], module: &Module) -> Value {
        let func = module.toplevel().get(func).unwrap();
        let mut frame = Frame::new();

        for (param, atom) in func.params.iter().zip(args.iter()) {
            let value = self.retrieve_atom(atom);
            frame.insert(param.to_owned(), value);
        }

        std::mem::swap(&mut frame, &mut self.frame);
        self.alloc_on_stack(frame);

        let val = self.eval(&func.body, module);
        let frame = self.stack.pop().unwrap();
        self.frame = frame;
        val
    }

    fn eval(&mut self, expr: &Expression, module: &Module) -> Value {
        match expr {
            Expression::Unit(atom) => self.retrieve_atom(atom),
            Expression::Let(x, v, b) => {
                let v_val = self.eval(v, module);
                self.frame.insert(x.to_owned(), v_val);
                self.eval(b, module)
            }
            Expression::Apply(f, args) => {
                todo!()
            }
            Expression::Call(f, args) => self.call(f, args, module),
            Expression::Match(atom, cases, def) => {
                let x = self.retrieve_atom(atom).expect_num();
                let branch = cases
                    .iter()
                    .find_map(|(y, branch)| if x == *y { Some(branch) } else { None })
                    .or(def.as_deref())
                    .expect("Match failed");
                self.eval(branch, module)
            }
            Expression::Operate(op, x, y) => {
                let x_num = self.retrieve_atom(x).expect_num();
                let y_num = self.retrieve_atom(y).expect_num();
                Value::Num(op.run(x_num, y_num))
            }
        }
    }
}
