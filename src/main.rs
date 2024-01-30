mod expr;
mod lexer;

use lexer::Scanner;
fn main() {
    let scan = Scanner::new("let fn (otherstring ){10929} >");
    let tokens = scan.map(|m| m.unwrap()).collect::<Vec<_>>();
    println!("{:?}", tokens);
}
