use crate::{
    expr::{Atom, Expression, Operation},
    module::Module,
};

use std::cmp::Ordering;
use std::collections::HashMap;

type Ptr = u32;
#[derive(Clone, Copy)]
enum Value {
    Num(usize),
    Ptr(Ptr),
}

enum HeapCell {
    Papp(String, Vec<Value>),
}

impl Value {
    fn expect_num(self) -> usize {
        match self {
            Value::Num(x) => x,
            _ => panic!("Expected number"),
        }
    }

    fn expect_ptr(self) -> Ptr {
        match self {
            Value::Ptr(x) => x,
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

    fn get(&self, reg: &str) -> Value {
        *self.0.get(reg).unwrap()
    }
}

static INIT_HEAP_SIZE: usize = 1 << 24;
static INIT_STACK_SIZE: usize = 1 << 18;

fn alloc_heap() -> Vec<HeapCell> {
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
    heap: Vec<HeapCell>,
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

    fn alloc_on_heap(&mut self, obj: HeapCell) -> Ptr {
        let ptr = self.heap.len();
        push_within_capacity(&mut self.heap, obj, "Memory has run out");
        ptr as Ptr
    }

    fn retrieve_ptr(&self, ptr: Ptr) -> &HeapCell {
        &self.heap[ptr as usize]
    }

    fn retrieve_atom(&self, atom: &Atom) -> Value {
        match atom {
            Atom::Var(x) => self.frame.get(x),
            Atom::Lit(x) => Value::Num(*x),
        }
    }

    fn call(&mut self, func: &str, args: &[Value], module: &Module) -> Value {
        let func = module.get(func).unwrap();
        let mut frame = Frame::new();
        for (param, value) in func.params.iter().zip(args.iter()) {
            frame.insert(param.to_owned(), *value);
        }

        std::mem::swap(&mut frame, &mut self.frame);
        self.alloc_on_stack(frame);

        let val = self.eval(&func.body, module);
        let frame = self.stack.pop().unwrap();
        self.frame = frame;
        val
    }

    fn apply(&mut self, ptr: Ptr, more_args: &[Value], module: &Module) -> Value {
        let HeapCell::Papp(func, init_args) = self.retrieve_ptr(ptr);
        let args = {
            let mut args = init_args.clone();
            args.extend_from_slice(more_args);
            args
        };
        let def = module.get(func).unwrap();
        match args.len().cmp(&def.params.len()) {
            Ordering::Less => {
                let papp = HeapCell::Papp(func.to_owned(), args);
                let ptr = self.alloc_on_heap(papp);
                Value::Ptr(ptr)
            }
            Ordering::Equal => self.call(&func.clone(), &args, module),
            Ordering::Greater => {
                let call_args = &args[0..def.params.len()];
                let val = self.call(&func.clone(), call_args, module);
                let ptr = val.expect_ptr();
                let rest = &args[def.params.len()..];
                self.apply(ptr, rest, module)
            }
        }
    }

    fn eval(&mut self, expr: &Expression, module: &Module) -> Value {
        match expr {
            Expression::Unit(atom) => self.retrieve_atom(atom),
            Expression::Let(x, v, b) => {
                let v_val = self.eval(v, module);
                self.frame.insert(x.to_owned(), v_val);
                self.eval(b, module)
            }
            Expression::Call(f, args) => {
                let args = args
                    .iter()
                    .map(|arg| self.retrieve_atom(arg))
                    .collect::<Vec<_>>();
                self.call(f, &args, module)
            }
            Expression::Apply(f, args) => {
                let ptr = self.frame.get(f).expect_ptr();
                let args = args
                    .iter()
                    .map(|arg| self.retrieve_atom(arg))
                    .collect::<Vec<_>>();
                self.apply(ptr, &args, module)
            }
            Expression::Papp(f, args) => {
                let args = args.iter().map(|arg| self.retrieve_atom(arg)).collect();
                let papp = HeapCell::Papp(f.to_owned(), args);
                let ptr = self.alloc_on_heap(papp);
                Value::Ptr(ptr)
            }
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
