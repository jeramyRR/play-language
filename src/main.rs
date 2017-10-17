#[macro_use]
extern crate log;

mod lexer;
mod parser;

use lexer::Token;

fn main() {

    let string = "(add 2 (subtract 4 2))";
    let tokens: Vec<Token> = lexer::tokenizer(string);

    println!("Tokens: {:?}", tokens);
}




