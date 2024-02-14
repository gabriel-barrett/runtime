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

    pub fn get(&self, name: &str) -> Option<&Definition> {
        self.toplevel.get(name)
    }
}

// Expressions must be SSA, applications and function arguments cannot be greater than size `ARGS_MAX_SIZE`

fn check(top: &HashMap<String, Definition>) {
    top.iter().for_each(|(name, def)| check_def(name, def, top));
}

fn check_def(name: &str, def: &Definition, top: &HashMap<String, Definition>) {
    assert_eq!(name, def.name.as_str());
    assert!(
        def.params.len() <= ARGS_MAX_SIZE,
        "Function `{name}` has more than {ARGS_MAX_SIZE} arguments"
    );
    let vars = &mut HashSet::new();
    for arg in def.params.iter() {
        insert_unique(arg, vars);
    }
    check_expr(&def.body, vars, top);
}

fn is_bound(x: &String, vars: &HashSet<String>) {
    assert!(vars.contains(x), "Unbound variable `{x}`");
}

fn insert_unique(x: &String, vars: &mut HashSet<String>) {
    assert!(
        vars.insert(x.clone()),
        "Variable `{x}` has already been defined. Functions are supposed to be in SSA"
    );
}

fn check_expr(expr: &Expression, vars: &mut HashSet<String>, top: &HashMap<String, Definition>) {
    match expr {
        Expression::Unit(Atom::Var(x)) => is_bound(x, vars),
        Expression::Unit(Atom::Lit(_)) => {}
        Expression::Let(name, val, body) => {
            check_expr(val, vars, top);
            insert_unique(name, vars);
            check_expr(body, vars, top);
        }
        Expression::Apply(closure, args) => {
            assert!(
                // the closure is also part of the argument of apply so
                // `args` should be strictly less than `ARGS_MAX_SIZE`
                args.len() < ARGS_MAX_SIZE,
                "Application has more than {ARGS_MAX_SIZE} arguments"
            );
            is_bound(closure, vars);
            for arg in args {
                if let Atom::Var(var) = arg {
                    is_bound(var, vars);
                }
            }
        }
        Expression::Call(func, args) => {
            assert!(
                args.len() <= ARGS_MAX_SIZE,
                "Application has more than {ARGS_MAX_SIZE} arguments"
            );
            let func = top
                .get(func)
                .unwrap_or_else(|| panic!("Unbound function `{func}`"));
            assert_eq!(func.params.len(), args.len(), "Wrong number of arguments");
            for arg in args {
                if let Atom::Var(var) = arg {
                    is_bound(var, vars);
                }
            }
        }
        Expression::Papp(func, args) => {
            assert!(
                args.len() < ARGS_MAX_SIZE,
                "Partial application has more than {ARGS_MAX_SIZE} arguments"
            );
            let func = top
                .get(func)
                .unwrap_or_else(|| panic!("Unbound function `{func}`"));
            assert!(
                func.params.len() > args.len(),
                "Partial application exceeds function arity"
            );
            for arg in args {
                if let Atom::Var(var) = arg {
                    is_bound(var, vars);
                }
            }
        }
        Expression::Match(atom, matches, default) => {
            if let Atom::Var(var) = atom {
                is_bound(var, vars)
            }
            let mut unique_pat = HashSet::new();
            for (pat, exp) in matches {
                assert!(unique_pat.insert(pat), "Repeated pattern in match");
                check_expr(exp, vars, top);
            }
            if let Some(exp) = default {
                check_expr(exp, vars, top)
            }
        }
        Expression::Operate(_, x, y) => {
            if let Atom::Var(var) = x {
                is_bound(var, vars);
            }
            if let Atom::Var(var) = y {
                is_bound(var, vars);
            }
        }
    }
}
