use std::str::Chars;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Keyword {
    Fn,
    Let,
    Match,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Symbol {
    LAngle,
    RAngle,
    Semicolon,
    Equal,
    Plus,
    Minus,
    Star,
    Slash,
    Percentage,
    And,
    Pipe,
    Caret,
    Underscore,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bracket {
    LBrace,
    RBrace,
    LParen,
    RParen,
}

impl Symbol {
    pub fn to_char(self) -> char {
        match self {
            Symbol::LAngle => '<',
            Symbol::RAngle => '>',
            Symbol::Semicolon => ';',
            Symbol::Equal => '=',
            Symbol::Plus => '+',
            Symbol::Minus => '-',
            Symbol::Star => '*',
            Symbol::Slash => '/',
            Symbol::Percentage => '%',
            Symbol::And => '&',
            Symbol::Pipe => '|',
            Symbol::Caret => '^',
            Symbol::Underscore => '_',
        }
    }
}

impl Bracket {
    pub fn to_char(self) -> char {
        match self {
            Bracket::LBrace => '{',
            Bracket::RBrace => '}',
            Bracket::LParen => '(',
            Bracket::RParen => ')',
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token {
    Keyword(Keyword),
    Identifier(String),
    Number(usize),
    Symbol(Symbol),
    Paren(Bracket),
}

#[derive(Debug)]
pub enum LexerError {
    NumErr,
    UnknownToken(char),
}

pub struct Scanner<'a> {
    start: &'a str,
    chars: Chars<'a>,
}

impl<'a> Scanner<'a> {
    pub fn new(source_code: &'a str) -> Self {
        Self {
            start: source_code,
            chars: source_code.chars(),
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.clone().next()
    }

    fn is_init(&self) -> bool {
        self.chars.as_str() == self.start
    }

    fn consume_scan(&mut self) -> Option<&str> {
        let span_length = self.start.len() - self.chars.as_str().len();
        if span_length == 0 {
            return None;
        }
        let res = &self.start[..span_length];
        self.start = self.chars.as_str();
        Some(res)
    }

    fn scan_char_if(&mut self, pred: fn(char) -> bool) -> bool {
        match self.peek() {
            Some(c) if pred(c) => {
                self.chars.next();
                true
            }
            _ => false,
        }
    }

    fn scan_identifier(&mut self) {
        if self.scan_char_if(char::is_alphabetic) {
            while self.scan_char_if(char::is_alphanumeric) {}
        }
    }

    fn scan_number(&mut self) {
        if self.scan_char_if(char::is_numeric) {
            while self.scan_char_if(char::is_alphanumeric) {}
        }
    }

    pub fn skip_whitespace(&mut self) {
        while self.scan_char_if(char::is_whitespace) {}
        self.start = self.chars.as_str();
    }

    pub fn lex_identifier(&mut self) -> Option<Token> {
        assert!(self.is_init());
        self.scan_identifier();
        let res = self.consume_scan()?;
        match res {
            "fn" => Some(Token::Keyword(Keyword::Fn)),
            "let" => Some(Token::Keyword(Keyword::Let)),
            "match" => Some(Token::Keyword(Keyword::Match)),
            _ => Some(Token::Identifier(res.into())),
        }
    }

    pub fn lex_number(&mut self) -> Option<Result<Token, LexerError>> {
        assert!(self.is_init());
        self.scan_number();
        let res = self.consume_scan()?;
        let num = res
            .parse()
            .map(Token::Number)
            .map_err(|_| LexerError::NumErr);
        Some(num)
    }

    pub fn lex_paren(&mut self) -> Option<Token> {
        let c = self.peek()?;
        use self::Bracket::*;
        use self::Token::Paren;
        let token = match c {
            '{' => Paren(LBrace),
            '}' => Paren(RBrace),
            '(' => Paren(LParen),
            ')' => Paren(RParen),
            _ => return None,
        };
        self.chars.next();
        self.start = self.chars.as_str();
        Some(token)
    }

    pub fn lex_symbol(&mut self) -> Option<Token> {
        let c = self.peek()?;
        use self::Symbol::*;
        use self::Token::Symbol;
        let token = match c {
            '<' => Symbol(LAngle),
            '>' => Symbol(RAngle),
            ';' => Symbol(Semicolon),
            '=' => Symbol(Equal),
            '+' => Symbol(Plus),
            '-' => Symbol(Minus),
            '*' => Symbol(Star),
            '/' => Symbol(Slash),
            '%' => Symbol(Percentage),
            '&' => Symbol(And),
            '|' => Symbol(Pipe),
            '^' => Symbol(Caret),
            '_' => Symbol(Underscore),
            _ => return None,
        };
        self.chars.next();
        self.start = self.chars.as_str();
        Some(token)
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Result<Token, LexerError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();
        if let Some(t) = self.lex_symbol() {
            return Some(Ok(t));
        }
        if let Some(t) = self.lex_paren() {
            return Some(Ok(t));
        }
        if let Some(t) = self.lex_identifier() {
            return Some(Ok(t));
        }
        if let Some(m) = self.lex_number() {
            return Some(m);
        }
        if let Some(c) = self.peek() {
            return Some(Err(LexerError::UnknownToken(c)));
        }
        None
    }
}
