#[macro_use]
extern crate log;

mod lexer;

fn main() {

    let string = "(add 2 (subtract 4 2))";
    let tokens: Vec<lexer::Token> = lexer::tokenizer(string);

    println!("Tokens: {:?}", tokens);
}




