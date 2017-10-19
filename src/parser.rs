use super::lexer::Token;

#[derive(Debug)]
pub enum ASTNodeType {
    NumberLiteral,
    StringLiteral,
    CallExpression
}

#[derive(Debug)]
pub struct Node {
    node_type: ASTNodeType,
    name: String,
}

#[derive(Debug)]
pub struct BaseNode {
    node_type: ASTNodeType,
    name: String,
    params: Vec<ASTNode>
}

#[derive(Debug)]
pub enum ASTNode {
    Node(Node),
    BaseNode(BaseNode)
}

#[derive(Debug)]
pub struct AST {
    ttype: &'static str,
    body: Vec<ASTNode>
}

pub fn parser(tokens: &[Token]) -> AST{
    let mut current: usize = 0;

    info!("tokens are: {:?}", tokens);

    let mut ast = AST { ttype: "Program", body: Vec::new() };
    while current < tokens.len() {
        ast.body.push(walk(tokens, &mut current).unwrap());
    }

    ast
}

fn walk(tokens: &[Token], current: &mut usize) -> Option<ASTNode> {
    let mut token: &Token = &tokens[*current];

    match *token {
        Token::Number(ref value) => {
            *current += 1;
            Some(ASTNode::Node(Node { node_type: ASTNodeType::NumberLiteral, name: value.clone() }))
        },
        Token::String(ref value) => {
            *current += 1;
            Some(ASTNode::Node(Node { node_type: ASTNodeType::StringLiteral, name: value.clone() }))
        },
        Token::Paren('(') => {
            *current += 1;
            token = &tokens[*current];

            let mut node: BaseNode = BaseNode {
                node_type: ASTNodeType::CallExpression,
                name: token_value(token),
                params: Vec::new()
            };

            *current += 1;

            token = &tokens[*current];

            while token_value(token) != ")" {
                node.params.push(walk(tokens, current).unwrap());
                token = &tokens[*current];
            }

            *current += 1;

            Some(ASTNode::BaseNode(node))
        },
        Token::Paren(_) | Token::Name(_) => { None }
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

#[cfg(test)]
mod tests {
    use super::*;

    use lexer;

    extern crate env_logger;

    #[test]
    fn test_parser() {
        let _ = env_logger::init();
        let string = "(add 2 (subtract 4 2))";
        let tokens = lexer::tokenizer(string);
        let ast = parser(&tokens);
        info!("ast: {:?}", ast);
    }
}

