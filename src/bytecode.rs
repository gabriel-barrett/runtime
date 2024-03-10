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
    ALLOC(REG, REG),
    ALLOCI(REG, NUM),
    BRANCH(NUM),
    LOAD(REG, NUM, REG),
    STORE(REG, NUM, REG),
    STOREI(REG, NUM, NUM),
}

pub struct Program {
    main: usize,
    code: Vec<Bytecode>,
}

fn compile_module(module: &Module) -> Vec<Vec<Bytecode>> {
    let mut programs = vec![];
    for (pos, (name, def)) in module.toplevel().iter().enumerate() {
        let expr = &def.body;
        let mut code = vec![];
        let reg_map = &mut |_name: &String| todo!();
        let top_map = &mut |_name: &String| todo!();
        let uniq = &mut 0;
        compile_expression(expr, &mut code, name, pos, uniq, reg_map, top_map);
        programs.push(code);
    }
    programs
}

fn compile_expression(
    expr: &Expression,
    code: &mut Vec<Bytecode>,
    rec_name: &str,
    rec_pos: usize,
    uniq: &mut usize,
    reg_map: &mut impl FnMut(&String) -> REG,
    top_map: &mut impl FnMut(&String) -> usize,
) {
    match expr {
        Expression::Unit(Atom::Lit(s)) => code.push(RETURNI(*s)),
        Expression::Unit(Atom::Var(s)) => {
            let r = reg_map(s);
            code.push(RETURN(r));
        }
        Expression::Call(name, atoms) => {
            let func = if name == rec_name {
                rec_pos as NUM
            } else {
                top_map(name) as NUM
            };
            push_arguments(atoms, code, reg_map);
            code.push(CALLI(func));
        }
        Expression::Papp(name, args) => {
            let func = if name == rec_name {
                rec_pos as NUM
            } else {
                top_map(name) as NUM
            };
            let n = args.len() as NUM + 1;
            let ptr = new_register(uniq, reg_map);
            code.push(ALLOCI(ptr, n));
            code.push(STOREI(ptr, 0, func));
            for (i, arg) in args.iter().enumerate() {
                let pos = i as NUM + 1;
                match arg {
                    Atom::Var(s) => {
                        let r = reg_map(s);
                        code.push(STORE(ptr, pos, r));
                    }
                    Atom::Lit(s) => code.push(STOREI(ptr, pos, *s)),
                };
            }
            code.push(RETURN(ptr));
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

fn new_register(uniq: &mut usize, reg_map: &mut impl FnMut(&String) -> REG) -> REG {
    let s = format!("x#{uniq}");
    *uniq += 1;
    reg_map(&s)
}
