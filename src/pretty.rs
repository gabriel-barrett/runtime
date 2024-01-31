use crate::expr::{Atom, Definition, Expression, Operation};
use crate::lexer::{Keyword, Token};
use std::fmt;

pub struct Pretty<T> {
    inner: T,
}

impl Expression {
    #[allow(dead_code)]
    pub fn pretty(&self) -> Pretty<&Self> {
        Pretty { inner: self }
    }
}

impl Definition {
    pub fn pretty(&self) -> Pretty<&Self> {
        Pretty { inner: self }
    }
}

impl Operation {
    #[allow(dead_code)]
    pub fn pretty(&self) -> Pretty<&Self> {
        Pretty { inner: self }
    }
}

impl Token {
    pub fn pretty(&self) -> Pretty<&Self> {
        Pretty { inner: self }
    }
}

fn fmt_newline_ident(f: &mut fmt::Formatter<'_>, ident: usize) -> fmt::Result {
    writeln!(f)?;
    (0..ident).try_for_each(|_| write!(f, "  "))
}

fn op_to_str(x: &Operation) -> String {
    match x {
        Operation::Add => "+".to_string(),
        Operation::Sub => "-".to_string(),
        Operation::Mul => "*".to_string(),
        Operation::Div => "/".to_string(),
        Operation::Mod => "%".to_string(),
        Operation::Eq => "==".to_string(),
        Operation::Lt => "<".to_string(),
        Operation::Le => "<=".to_string(),
        Operation::Gt => ">".to_string(),
        Operation::Ge => ">=".to_string(),
        Operation::And => "&&".to_string(),
        Operation::Or => "||".to_string(),
        Operation::Xor => "^".to_string(),
        Operation::Sr => ">>".to_string(),
        Operation::Sl => "<<".to_string(),
    }
}

fn atom_to_str(x: &Atom) -> String {
    match x {
        Atom::Var(x) => x.to_string(),
        Atom::Lit(x) => x.to_string(),
    }
}

fn fmt_expr(
    f: &mut fmt::Formatter<'_>,
    expr: &Expression,
    ident: usize,
    newline: bool,
) -> fmt::Result {
    if newline {
        fmt_newline_ident(f, ident)?;
    }
    match expr {
        Expression::Unit(x) => write!(f, "{}", atom_to_str(x)),
        Expression::Let(x, val, body) => {
            write!(f, "let {} = ", x)?;
            fmt_expr(f, val, ident + 1, false)?;
            write!(f, ";")?;
            fmt_expr(f, body, ident, true)
        }
        Expression::Apply(func, xs) => {
            let mut args = Vec::with_capacity(xs.len() + 1);
            args.push(func.clone());
            xs.iter().map(atom_to_str).for_each(|s| args.push(s));
            fmt_app(f, &"apply".into(), &args)
        }
        Expression::Call(func, xs) => {
            let xs = xs.iter().map(atom_to_str).collect::<Vec<_>>();
            fmt_app(f, func, &xs)
        }
        Expression::Match(x, matches, default) => {
            write!(f, "match {} {{", atom_to_str(x))?;
            for (num, exp) in matches {
                fmt_newline_ident(f, ident + 1)?;
                write!(f, "{} => {{", num)?;
                fmt_expr(f, exp, ident + 2, true)?;
                fmt_newline_ident(f, ident + 1)?;
                write!(f, "}}")?;
            }
            if let Some(expr) = default {
                fmt_newline_ident(f, ident + 1)?;
                write!(f, "_ => {{")?;
                fmt_expr(f, expr, ident + 2, true)?;
                fmt_newline_ident(f, ident + 1)?;
                write!(f, "}}")?;
            }
            fmt_newline_ident(f, ident)?;
            write!(f, "}}")
        }
        Expression::Operate(op, x, y) => write!(
            f,
            "({} {} {})",
            op_to_str(op),
            atom_to_str(x),
            atom_to_str(y)
        ),
    }
}

fn fmt_def(f: &mut fmt::Formatter<'_>, def: &Definition) -> fmt::Result {
    write!(f, "fn ")?;
    fmt_app(f, &def.name, &def.args)?;
    write!(f, " {{")?;
    fmt_expr(f, &def.body, 1, true)?;
    write!(f, "\n}}")
}

fn fmt_app(f: &mut fmt::Formatter<'_>, head: &String, args: &[String]) -> fmt::Result {
    write!(f, "({}", head)?;
    for arg in args {
        write!(f, " {arg}")?;
    }
    write!(f, ")")
}

impl fmt::Debug for Pretty<&Expression> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt_expr(f, self.inner, 0, false)
    }
}

impl fmt::Debug for Pretty<&Definition> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt_def(f, self.inner)
    }
}

impl fmt::Debug for Pretty<&Operation> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", op_to_str(self.inner))
    }
}

impl fmt::Debug for Pretty<&Token> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.inner {
            Token::Keyword(Keyword::Fn) => write!(f, "fn"),
            Token::Keyword(Keyword::Let) => write!(f, "fn"),
            Token::Keyword(Keyword::Match) => write!(f, "match"),
            Token::Keyword(Keyword::Apply) => write!(f, "apply"),
            Token::Identifier(name) => write!(f, "{}", name),
            Token::Symbol(c) => write!(f, "{}", c.to_char()),
            Token::Number(num) => write!(f, "{}", num),
            Token::Paren(c) => write!(f, "{}", c.to_char()),
        }
    }
}
