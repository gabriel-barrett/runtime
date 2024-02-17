#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;

use crate::expr::{Atom, Definition, Expression};
use crate::module::Module;

use lurk::lem::{Block, Ctrl, Func, Lit, Op, Var};

pub struct Coroutine {
    func: Func,
    query_index: usize,
    rc: usize,
    leaf: bool,
}

pub struct CoroutineModule {
    toplevel: HashMap<usize, Coroutine>,
}

impl CoroutineModule {
    pub fn from_module(module: &Module) -> Self {
        compile_module(module)
    }
}

fn compile_module(module: &Module) -> CoroutineModule {
    let mut index_map = HashMap::new();
    for (i, name) in module.toplevel().keys().enumerate() {
        index_map.insert(name, i);
    }
    let mut toplevel = HashMap::new();
    for (name, def) in module.toplevel().iter() {
        let coroutine = compile_definition(def, name, &index_map);
        let index = index_map.get(name).unwrap();
        toplevel.insert(*index, coroutine);
    }
    CoroutineModule { toplevel }
}

fn compile_definition(
    def: &Definition,
    name: &String,
    index_map: &HashMap<&String, usize>,
) -> Coroutine {
    let query_index = *index_map.get(name).unwrap();
    let name = name.clone();
    let body = compile_expression(&def.body, index_map);
    let slots_count = body.count_slots();
    let input_params = def.params.iter().map(|x| Var::new(x)).collect();
    let func = Func {
        name,
        input_params,
        output_size: 1,
        body,
        slots_count,
    };
    Coroutine {
        func,
        query_index,
        rc: 1,
        leaf: false,
    }
}

fn compile_expression(expr: &Expression, index_map: &HashMap<&String, usize>) -> Block {
    let mut ops = vec![];
    let mut rest_expr = expr;
    while let Expression::Let(name, val, body) = rest_expr {
        rest_expr = body;
        let var = Var::new(name);
        let op = match val.as_ref() {
            Expression::Unit(Atom::Var(..)) => {
                panic!("TODO: rename operation in LEM")
            }
            Expression::Unit(Atom::Lit(x)) => Op::Lit(var, Lit::Num(*x as u128)),
            Expression::Call(..) => {
                panic!("TODO: coroutine call in LEM")
            }
            Expression::Papp(..) => {
                panic!("TODO: partial application")
            }
            Expression::Apply(..) => {
                panic!("TODO: apply coroutine")
            }
            Expression::Operate(_, x, y) => {
                todo!()
            }
            _ => {
                panic!("TODO: LEM does not yet support inner blocks")
            }
        };
        ops.push(op);
    }
    let ctrl = match rest_expr {
        Expression::Let(..) => unreachable!(),
        Expression::Unit(Atom::Var(x)) => Ctrl::Return(vec![Var::new(x)]),
        Expression::Unit(Atom::Lit(_)) => {
            todo!()
        }
        Expression::Apply(closure, args) => {
            todo!()
        }
        Expression::Call(func, args) => {
            todo!()
        }
        Expression::Papp(func, args) => {
            todo!()
        }
        Expression::Match(atom, matches, default) => {
            todo!()
        }
        Expression::Operate(_, x, y) => {
            todo!()
        }
    };
    Block { ops, ctrl }
}
