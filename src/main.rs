mod expr;
mod lexer;
mod parser;
mod pretty;

use lexer::Scanner as LexerScanner;
use parser::Scanner as ParserScanner;

static SOURCE: &str = "
fn (id x) {
  x
}

fn (flip f x y) {
  (apply f y x)
}

fn (polynomial x) {
  let x2 = (* x x);
  let x3 = (* x x2);
  (+ x2 x3)
}

fn (not x) {
  match x {
    0 => {
      1
    }
    1 => {
      0
    }
    _ => {
      x
    }
  }
}
";

fn main() {
    let lexer_scanner = LexerScanner::new(SOURCE);
    let tokens = lexer_scanner.map(|m| m.unwrap());
    let parser_scanner = ParserScanner::new(tokens);
    let definitions = parser_scanner.map(|m| m.unwrap());
    definitions.for_each(|def| println!("{:?}", def.pretty()));
}
