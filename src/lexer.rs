#[derive(Debug)]
pub enum Token {
    Paren(char),
    String(String),
    Number(String),
    Name(String)
}


/// Take a string of code and tokenize it
/// (add 2 (subtract 4 2)) => [ { type: 'paren', value: '('}, ...]
pub fn tokenizer(input: &str) -> Vec<Token> {
    let mut current: usize = 0; // current location of cursor

    let mut tokens: Vec<Token> = Vec::new(); // array for tokens

    let input_chars: Vec<char> = input.chars().collect();

    while current < input.len() {
        let mut cur_char: char = input_chars[current];

        match cur_char {
            '(' | ')' => {
                debug!("Adding token: Paren({})", cur_char);
                tokens.push(Token::Paren(cur_char));
                current += 1;
                continue;
            }
            '0' ... '9' => {
                let mut num_value: String = String::new();

                while cur_char.is_numeric() {
                    num_value.push(cur_char);
                    current += 1;
                    cur_char = input_chars[current];
                }

                debug!("Adding token: Number({})", num_value);
                tokens.push(Token::Number(num_value));
                continue;
            }
            '"' => {
                let mut str_value = String::new();

                current += 1;
                cur_char = input_chars[current];

                while cur_char != '"' {
                    str_value.push(cur_char);
                    current += 1;
                    cur_char = input_chars[current];
                }

                debug!("Adding token: String({})", str_value);
                tokens.push(Token::String(str_value));

                current += 1;

                continue;
            }
            _ if cur_char.is_whitespace() => {
                trace!("Skipping whitespace");
                current += 1;
                continue;
            }
            _ if cur_char.is_alphabetic() => {
                let mut letter_value = String::new();

                while cur_char.is_alphabetic() {
                    letter_value.push(cur_char);
                    current += 1;
                    cur_char = input_chars[current];
                }

                debug!("Adding token: Name({})", letter_value);
                tokens.push(Token::Name(letter_value));
            }
            _ => {
                panic!("Unrecognized character: {}", cur_char);
            }
        }
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate env_logger;

    #[test]
    fn test_tokenizer() {
        let _ = env_logger::init();
        let string = "(add 2 (subtract 4 2))";
        let tokens = tokenizer(string);
        info!("Tokens: {:?}", tokens);
    }
}
