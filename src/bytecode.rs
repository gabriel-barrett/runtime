use crate::{
    expr::{Atom, Expression, Operation},
    module::Module,
};

type REG = u32;
type NUM = u64;

use Bytecode::*;
pub enum Bytecode {
    OP(Operation, REG, REG, REG),
    OPI1(Operation, REG, NUM, REG),
    OPI2(Operation, REG, REG, NUM),
    CALL(REG),
    CALLI(NUM),
    ARG(NUM, REG),
    ARGI(NUM, NUM),
    RETURN(REG),
    RETURNI(NUM),
    BRANCH(NUM),
    LOAD(NUM),
    STORE(NUM),
}

pub struct Program {
    main: usize,
    code: Vec<Bytecode>,
}

fn compile_module(module: &Module) -> Vec<Bytecode> {
    todo!()
}

fn compile_expression(
    expr: &Expression,
    code: &mut Vec<Bytecode>,
    rec_name: &str,
    rec_pos: usize,
    reg_map: &mut impl FnMut(&String) -> REG,
    top_map: &mut impl FnMut(&String) -> usize,
) {
    match expr {
        Expression::Unit(Atom::Lit(s)) => code.push(RETURNI(*s)),
        Expression::Unit(Atom::Var(s)) => {
            let r = reg_map(s);
            code.push(RETURN(r))
        }
        Expression::Call(name, atoms) => {
            let func = if name == rec_name {
                rec_pos as NUM
            } else {
                top_map(name) as NUM
            };
            push_arguments(atoms, code, reg_map);
            code.push(CALLI(func))
        }
        _ => todo!(),
    }
}

fn push_arguments(
    args: &[Atom],
    code: &mut Vec<Bytecode>,
    reg_map: &mut impl FnMut(&String) -> REG,
) {
    for (i, arg) in args.iter().enumerate() {
        match arg {
            Atom::Var(s) => {
                let pos = i as NUM;
                let reg = reg_map(s);
                code.push(ARG(pos, reg))
            }
            Atom::Lit(x) => {
                let pos = i as NUM;
                let val = *x as NUM;
                code.push(ARGI(pos, val))
            }
        }
    }
}
