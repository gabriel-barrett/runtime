use crate::expr::{Atom, Definition, Expression};
use std::collections::{HashMap, HashSet};

static ARGS_MAX_SIZE: usize = 8;

pub struct Module {
    toplevel: HashMap<String, Definition>,
}

impl Module {
    pub fn new(iter: impl Iterator<Item = Definition>) -> Self {
        let toplevel = iter.map(|def| (def.name.clone(), def)).collect();
        check(&toplevel);
        Self { toplevel }
    }

    pub fn toplevel(&self) -> &HashMap<String, Definition> {
        &self.toplevel
    }
}

fn check(top: &HashMap<String, Definition>) {
    top.iter().for_each(|(name, def)| check_def(name, def, top));
}

fn check_def(name: &str, def: &Definition, top: &HashMap<String, Definition>) {
    assert_eq!(name, def.name.as_str());
    assert!(
        def.args.len() <= ARGS_MAX_SIZE,
        "function `{name}` has more than {ARGS_MAX_SIZE} arguments"
    );
    let vars = &mut def.args.clone();
    check_expr(&def.body, vars, top);
}

fn check_expr(expr: &Expression, vars: &mut Vec<String>, top: &HashMap<String, Definition>) {
    match expr {
        Expression::Unit(Atom::Var(x)) => {
            assert!(vars.contains(x), "unbound variable `{x}`");
        }
        Expression::Unit(Atom::Lit(_)) => {}
        Expression::Let(name, val, body) => {
            let vars_init_len = vars.len();
            check_expr(val, vars, top);
            vars.truncate(vars_init_len);
            vars.push(name.clone());
            check_expr(body, vars, top);
        }
        Expression::Apply(closure, args) => {
            assert!(
                // the closure is also part of the argument of apply so
                // `args` should be strictly less than `ARGS_MAX_SIZE`
                args.len() < ARGS_MAX_SIZE,
                "application has more than {ARGS_MAX_SIZE} arguments"
            );
            assert!(vars.contains(closure), "unbound variable `{closure}`");
            for arg in args {
                if let Atom::Var(var) = arg {
                    assert!(vars.contains(var), "unbound variable `{var}`");
                }
            }
        }
        Expression::Call(func, args) => {
            assert!(
                args.len() <= ARGS_MAX_SIZE,
                "application has more than {ARGS_MAX_SIZE} arguments"
            );
            assert!(top.contains_key(func), "unbound function `{func}`");
            for arg in args {
                if let Atom::Var(var) = arg {
                    assert!(vars.contains(var), "unbound variable `{var}`");
                }
            }
        }
        Expression::Match(atom, matches, default) => {
            if let Atom::Var(var) = atom {
                assert!(vars.contains(var), "unbound variable `{var}`");
            }
            let mut unique_pat = HashSet::new();
            for (pat, exp) in matches {
                assert!(unique_pat.insert(pat), "repeated pattern in match");
                check_expr(exp, vars, top);
            }
            if let Some(exp) = default {
                check_expr(exp, vars, top)
            }
        }
        Expression::Operate(_, x, y) => {
            if let Atom::Var(var) = x {
                assert!(vars.contains(var), "unbound variable `{var}`");
            }
            if let Atom::Var(var) = y {
                assert!(vars.contains(var), "unbound variable `{var}`");
            }
        }
    }
}
