#![allow(dead_code)]
#![allow(unused_imports)]

use std::collections::HashMap;

use crate::expr::{Atom, Definition, Expression};
use crate::module::Module;

use lurk::lem::{Block, Ctrl, Func, Op, Var};

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
        // TODO
        output_size: 0,
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

fn compile_expression(body: &Expression, index_map: &HashMap<&String, usize>) -> Block {
    todo!()
}
