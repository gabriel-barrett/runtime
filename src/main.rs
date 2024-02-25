mod bytecode;
mod expr;
mod interpreter;
mod lexer;
mod module;
mod parser;
mod pretty;

use interpreter::State;
use lexer::Scanner as LexerScanner;
use module::Module;
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

fn (nil n c) {
  n
}

fn (cons x xs n c) {
  (apply c x xs)
}

fn (buildList n) {
  match n {
    0 => {
      (papp nil)
    }
    _ => {
      let m = (- n 1);
      let tail = (buildList m);
      (papp cons n tail)
    }
  }
}

fn (sumList xs) {
  let sum = (papp sumListAux);
  (apply xs 0 sum)
}

fn (sumListAux x ys) {
  let y = (sumList ys);
  (+ x y)
}

fn (main) {
  let xs = (buildList 100);
  (sumList xs)
}
";

fn main() {
    let lexer_scanner = LexerScanner::new(SOURCE);
    let tokens = lexer_scanner.map(|m| m.unwrap());
    let parser_scanner = ParserScanner::new(tokens);
    let definitions = parser_scanner.map(|m| m.unwrap());
    let module = Module::new(definitions);
    let top = module.toplevel().iter();
    top.for_each(|(_, def)| println!("{:?}\n", def.pretty()));

    let mut state = State::new();
    let val = state.run(&module);
    println!("main: {:?}", val);
}
