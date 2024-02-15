use crate::expr::{Atom, Definition, Expression, Operation};
use crate::lexer::{Bracket, Keyword, Symbol, Token};
use std::iter::Peekable;

pub struct Scanner<Iter: Iterator> {
    iter: Peekable<Iter>,
}

#[derive(Debug)]
pub enum ParserError {
    Expected(String),
    UnknownOperation(String),
}

fn expected<S: Into<String>>(msg: S) -> ParserError {
    ParserError::Expected(msg.into())
}

impl<Iter: Iterator<Item = Token>> Scanner<Iter> {
    pub fn new(iter: Iter) -> Self {
        Self {
            iter: iter.peekable(),
        }
    }

    fn consume_token(&mut self) -> Option<Token> {
        self.iter.next()
    }

    fn peek(&mut self) -> Option<&Token> {
        self.iter.peek()
    }

    fn expect_token(&mut self, t: &Token) -> Result<(), ParserError> {
        let s = self
            .consume_token()
            .ok_or(expected(format!("{:?}", t.pretty())))?;
        if *t != s {
            return Err(expected(format!("{:?}", t.pretty())));
        }
        Ok(())
    }

    fn parse_symbol(&mut self) -> Option<Vec<Symbol>> {
        let mut symbols = vec![];
        while let Some(Token::Symbol(s)) = self.peek() {
            symbols.push(*s);
            self.consume_token();
        }
        if symbols.is_empty() {
            return None;
        }
        Some(symbols)
    }

    fn parse_atom(&mut self) -> Option<Atom> {
        let x = match self.peek()? {
            Token::Identifier(v) => Atom::Var(v.clone()),
            Token::Number(n) => Atom::Lit(*n),
            _ => return None,
        };
        self.consume_token();
        Some(x)
    }

    fn parse_op(&mut self) -> Option<Result<Operation, ParserError>> {
        use Operation::*;
        let symbols = self.parse_symbol()?;
        let string: String = symbols.into_iter().map(|s| s.to_char()).collect();
        match string.as_str() {
            "+" => Some(Ok(Add)),
            "-" => Some(Ok(Sub)),
            "*" => Some(Ok(Mul)),
            "/" => Some(Ok(Div)),
            "%" => Some(Ok(Mod)),
            "==" => Some(Ok(Eq)),
            "<" => Some(Ok(Lt)),
            "<=" => Some(Ok(Le)),
            ">" => Some(Ok(Gt)),
            ">=" => Some(Ok(Ge)),
            "&&" => Some(Ok(And)),
            "||" => Some(Ok(Or)),
            "^" => Some(Ok(Xor)),
            ">>" => Some(Ok(Sr)),
            "<<" => Some(Ok(Sl)),
            _ => Some(Err(ParserError::UnknownOperation(string))),
        }
    }

    fn parse_def(&mut self) -> Option<Result<Definition, ParserError>> {
        if Token::Keyword(Keyword::Fn) != *self.peek()? {
            return None;
        }
        self.consume_token();

        Some(self.parse_def_inner())
    }

    fn parse_def_inner(&mut self) -> Result<Definition, ParserError> {
        let (name, params) = self.parse_function_header()?;
        self.expect_token(&Token::Paren(Bracket::LBrace))?;
        let body = self.parse_expr()?;
        self.expect_token(&Token::Paren(Bracket::RBrace))?;
        Ok(Definition { name, params, body })
    }

    fn parse_expr(&mut self) -> Result<Expression, ParserError> {
        let expr = match self.peek().ok_or(expected("an expression"))? {
            Token::Keyword(Keyword::Let) => return self.parse_let(),
            Token::Keyword(Keyword::Match) => return self.parse_match(),
            Token::Paren(Bracket::LParen) => return self.parse_apply(),
            Token::Identifier(var) => Expression::Unit(Atom::Var(var.clone())),
            Token::Number(num) => Expression::Unit(Atom::Lit(*num)),
            _ => return Err(expected("an expression")),
        };
        self.consume_token();
        Ok(expr)
    }

    fn parse_function_header(&mut self) -> Result<(String, Vec<String>), ParserError> {
        self.expect_token(&Token::Paren(Bracket::LParen))?;
        let c = self.consume_token().ok_or(expected("function name"))?;
        let Token::Identifier(name) = c else {
            return Err(expected("function name"));
        };
        let mut args = vec![];
        while let Some(Token::Identifier(arg)) = self.peek() {
            args.push(arg.clone());
            self.consume_token();
        }
        self.expect_token(&Token::Paren(Bracket::RParen))?;
        Ok((name.clone(), args))
    }

    fn parse_apply(&mut self) -> Result<Expression, ParserError> {
        self.expect_token(&Token::Paren(Bracket::LParen))?;
        if let Some(op) = self.parse_op() {
            let op = op?;
            let x = self.parse_atom().ok_or(expected("an atom"))?;
            let y = self.parse_atom().ok_or(expected("an atom"))?;
            self.expect_token(&Token::Paren(Bracket::RParen))?;
            return Ok(Expression::Operate(op, x, y));
        }
        let token = self.consume_token();
        if let Some(Token::Identifier(func)) = token {
            let mut args = vec![];
            while let Some(arg) = self.parse_atom() {
                args.push(arg);
            }
            self.expect_token(&Token::Paren(Bracket::RParen))?;
            Ok(Expression::Call(func.clone(), args))
        } else if let Some(Token::Keyword(Keyword::Apply)) = token {
            let Some(Token::Identifier(func)) = self.consume_token() else {
                return Err(expected("a function to apply to"));
            };
            let mut args = vec![];
            while let Some(arg) = self.parse_atom() {
                args.push(arg);
            }
            self.expect_token(&Token::Paren(Bracket::RParen))?;
            Ok(Expression::Apply(func, args))
        } else if let Some(Token::Keyword(Keyword::Papp)) = token {
            let Some(Token::Identifier(func)) = self.consume_token() else {
                return Err(expected("a function to partially apply to"));
            };
            let mut args = vec![];
            while let Some(arg) = self.parse_atom() {
                args.push(arg);
            }
            self.expect_token(&Token::Paren(Bracket::RParen))?;
            Ok(Expression::Papp(func, args))
        } else {
            Err(expected("a function or operator"))
        }
    }

    fn parse_let(&mut self) -> Result<Expression, ParserError> {
        self.expect_token(&Token::Keyword(Keyword::Let))?;
        let Some(Token::Identifier(name)) = self.consume_token() else {
            return Err(expected("an identifier"));
        };
        self.expect_token(&Token::Symbol(Symbol::Equal))?;
        let val = self.parse_expr()?;
        self.expect_token(&Token::Symbol(Symbol::Semicolon))?;
        let body = self.parse_expr()?;
        Ok(Expression::Let(name.clone(), val.into(), body.into()))
    }

    fn parse_match(&mut self) -> Result<Expression, ParserError> {
        self.expect_token(&Token::Keyword(Keyword::Match))?;
        let atom = self.parse_atom().ok_or(expected("an atom"))?;
        let mut matches = vec![];
        let mut default = None;
        self.expect_token(&Token::Paren(Bracket::LBrace))?;
        while let Some(Token::Number(n)) = self.peek() {
            let n = *n;
            self.consume_token();
            let symbols = self.parse_symbol().ok_or(expected("=>"))?;
            let string: String = symbols.into_iter().map(|s| s.to_char()).collect();
            if string.as_str() != "=>" {
                return Err(expected("=>"));
            }
            self.expect_token(&Token::Paren(Bracket::LBrace))?;
            let expr = self.parse_expr()?;
            self.expect_token(&Token::Paren(Bracket::RBrace))?;
            matches.push((n, expr));
        }
        if let Some(Token::Symbol(Symbol::Underscore)) = self.peek() {
            self.consume_token();
            let symbols = self.parse_symbol().ok_or(expected("=>"))?;
            let string: String = symbols.into_iter().map(|s| s.to_char()).collect();
            if string.as_str() != "=>" {
                return Err(expected("=>"));
            }
            self.expect_token(&Token::Paren(Bracket::LBrace))?;
            let expr = self.parse_expr()?;
            self.expect_token(&Token::Paren(Bracket::RBrace))?;
            default = Some(expr.into());
        }
        self.expect_token(&Token::Paren(Bracket::RBrace))?;
        Ok(Expression::Match(atom, matches, default))
    }
}

impl<Iter: Iterator<Item = Token>> Iterator for Scanner<Iter> {
    type Item = Result<Definition, ParserError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.parse_def()
    }
}
