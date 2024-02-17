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
        let coroutine = compile_definition(def, &index_map);
        let index = index_map.get(name).unwrap();
        toplevel.insert(*index, coroutine);
    }
    CoroutineModule { toplevel }
}

fn compile_definition(_def: &Definition, _index_map: &HashMap<&String, usize>) -> Coroutine {
    todo!()
}
