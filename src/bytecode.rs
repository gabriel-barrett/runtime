use crate::{
    expr::{Atom, Expression, Operation},
    module::Module,
};

type REG = u32;
type NUM = u64;

pub enum Bytecode {
    OP(Operation, REG, REG, REG),
    OPI1(Operation, REG, NUM, REG),
    OPI2(Operation, REG, REG, NUM),
    CALL(REG),
    CALLI(NUM),
    ARG(NUM, REG),
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
    mut reg_map: impl FnMut(&String) -> REG,
) {
    use Bytecode::*;
    match expr {
        Expression::Unit(Atom::Lit(s)) => code.push(RETURNI(*s)),
        Expression::Unit(Atom::Var(s)) => {
            let r = reg_map(s);
            code.push(RETURN(r))
        }
        _ => todo!(),
    }
}
