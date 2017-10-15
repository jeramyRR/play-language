#[macro_use]
extern crate log;

fn main() {

    let string = "(add 2 (subtract 4 2))";
    let tokens: Vec<Token> = tokenizer(string);

    println!("Tokens: {:?}", tokens);
}

const PAREN: &'static str = "paren";
const NUMBER: &'static str = "number";
const STRING: &'static str = "string";
const NAME: &'static str = "name";

#[derive(Debug)]
struct Token {
    ttype: String,
    value: String
}

/// Take a string of code and tokenize it
/// (add 2 (subtract 4 2)) => [ { type: 'paren', value: '('}, ...]
fn tokenizer(input: &str) -> Vec<Token> {
    let mut current: usize = 0; // current location of cursor

    let mut tokens: Vec<Token> = Vec::new(); // array for tokens

    let input_chars: Vec<char> = input.chars().collect();

    trace!("input length is: {}", input.len());
    while current < input.len() {
        let mut cur_char: char = input_chars[current];

        match cur_char {
            '(' | ')' => {
                add_new_token_from_char(PAREN, cur_char, &mut tokens);
                current = current + 1;
                continue;
            }
            '0' ... '9' => {
                let mut num_value: String = String::new();

                while cur_char.is_numeric() {
                    num_value.push(cur_char);
                    current = current + 1;
                    cur_char = input_chars[current];
                }

                add_new_token(NUMBER, num_value, &mut tokens);
                continue;
            }
            '"' => {
                let mut str_value = String::new();

                current = current + 1;
                cur_char = input_chars[current];

                while cur_char != '"' {
                    str_value.push(cur_char);
                    current = current + 1;
                    cur_char = input_chars[current];
                }

                add_new_token(STRING, str_value, &mut tokens);

                current = current + 1; // skip the closing '"' character

                continue;
            }
            _ if cur_char.is_whitespace() => {
                current = current + 1;
                continue;
            }
            _ if cur_char.is_alphabetic() => {
                let mut letter_value = String::new();

                while cur_char.is_alphabetic() {
                    letter_value.push(cur_char);
                    current = current + 1;
                    cur_char = input_chars[current];
                }

                add_new_token(NAME, letter_value, &mut tokens);
            }
            _ => {}
        }
    }

    return tokens;
}

fn add_new_token_from_char(token_type: &str, value: char, tokens: &mut Vec<Token>) {
    let mut char_string = String::new();
    char_string.push(value);
    add_new_token(token_type, char_string, tokens);
}

fn add_new_token(token_type: &str, value: String, tokens: &mut Vec<Token>) {
    trace!("Creating new token: Token [ ttype: {}, value: {} ]", token_type, value);
    tokens.push(Token { ttype: String::from(token_type), value: value });
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate env_logger;

    #[test]
    fn test_tokenizer() {
        let _ = env_logger::init();
        info!("can log from tests");
        let string = "(add 2 (subtract 4 2))";
        tokenizer(string);
    }
}