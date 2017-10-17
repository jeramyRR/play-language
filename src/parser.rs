use super::lexer::Token;

enum ASTNodeType {
    NumberLiteral,
    StringLiteral,
    CallExpression
}

struct Node {
    node_type: ASTNodeType,
    name: String,
}

struct BaseNode {
    node_type: ASTNodeType,
    name: String,
    params: Vec<Node>
}

enum ASTNode {
    Node(Node),
    BaseNode(BaseNode)
}

pub fn parser(tokens: Vec<Token>) {
    let mut current: usize = 0;

    walk(tokens.as_slice(), &mut current);
}

fn walk(tokens: &[Token], current: &mut usize) -> Option<ASTNode> {
    let mut token: &Token = &tokens[*current];

    match *token {
        Token::Number(ref value) => {
            *current += 1;
            Some(ASTNode::Node(Node { node_type: ASTNodeType::NumberLiteral, name: value.clone() }))
        }
        Token::String(ref value) => {
            *current += 1;
            Some(ASTNode::Node(Node { node_type: ASTNodeType::StringLiteral, name: value.clone() }))
        }
        Token::Paren('(') => {
            *current += 1;
            token = &tokens[*current];

            let mut node: BaseNode = BaseNode {
                node_type: ASTNodeType::CallExpression,
                name: tokenValue(&token),
                params: Vec::new()
            };

            *current += 1;

            token = &tokens[*current];

            while token_value(&token) != ")" {
                match walk(&tokens, current).unwrap() {
                    ASTNode::Node(val) => node.params.push(val),
                    _ => ()
                };

                token = &tokens[*current];
            }

            *current += 1;

            return Some(ASTNode::BaseNode(node));
        }
        Token::Paren(_) => { None }
        Token::Name(_) => { None }
    }
}

fn token_value(token: &Token) -> String {
    match *token {
        Token::Number(ref value) => value.clone(),
        Token::String(ref value) => value.clone(),
        Token::Paren(ref value) => {
            let mut str_val = String::new();
            str_val.push(*value);
            str_val
        }
        Token::Name(ref value) => value.clone()
    }
}