use std::iter::Peekable;
use std::slice::Iter;

use alembers_common::token::Token;

#[derive(Debug, Clone)]
pub enum ASTNode {
    Number(f64),
    UnaryOp(char, Box<ASTNode>),
    BinaryOp(char, Box<ASTNode>, Box<ASTNode>),
    Identifier(String),
}

impl ASTNode {
    pub fn to_text(&self) -> String {
        match self {
            ASTNode::Number(num) => num.to_string(),
            ASTNode::UnaryOp(op, expr) => format!("{}{}", op, expr.to_text()),
            ASTNode::BinaryOp(op, left, right) => {
                let left_str = left.to_text();
                let right_str = right.to_text();

                let left_str = if needs_parentheses(left) {
                    format!("({})", left_str)
                } else {
                    left_str
                };

                let right_str = if needs_parentheses(right) {
                    format!("({})", right_str)
                } else {
                    right_str
                };

                format!("{} {} {}", left_str, op, right_str)
            }
            ASTNode::Identifier(name) => name.clone(),
        }
    }
}

fn needs_parentheses(expr: &ASTNode) -> bool {
    match expr {
        ASTNode::BinaryOp(op, _, _) => match op {
            '^' | '*' | '/' | '+' | '-' => true,
            _ => unreachable!(),
        },
        ASTNode::UnaryOp(_, _) => true,
        _ => false,
    }
}

pub struct Parser<'a> {
    tokens: Peekable<Iter<'a, Token>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Parser<'a> {
        Parser {
            tokens: tokens.iter().peekable(),
        }
    }

    fn expect(&mut self, expected_token: Token) -> Option<&Token> {
        let token = self.tokens.next().filter(|&token| *token == expected_token);

        if token.is_none() {
            self.tokens
                .peek()
                .filter(|&token| *token == &expected_token)
                .copied()
        } else {
            token
        }
    }

    fn parse_primary(&mut self) -> Option<ASTNode> {
        match self.tokens.next()? {
            Token::PositiveInteger(val) => Some(ASTNode::Number(*val as f64)),
            Token::Float(val) => Some(ASTNode::Number(*val)),
            Token::NegativeInteger(val) => Some(ASTNode::UnaryOp(
                '-',
                Box::new(ASTNode::Number(*val as f64)),
            )),
            Token::LeftParenthesis => {
                let expr = self.parse_expr()?;
                self.expect(Token::RightParenthesis)?;
                Some(expr)
            }
            Token::Indentifier(id) => Some(ASTNode::Identifier(id.to_owned())),
            _ => None,
        }
    }

    fn parse_expr(&mut self) -> Option<ASTNode> {
        let mut left = self.parse_primary()?;

        while let Some(operator_token) = self.tokens.peek().cloned() {
            let operator = match operator_token {
                Token::Plus => '+',
                Token::Minus => '-',
                Token::Exp => '^',
                Token::Multiply => '*',
                Token::Divide => '/',
                Token::Equal => '=',
                _ => break,
            };

            self.tokens.next();

            let right = self.parse_primary()?;

            left = ASTNode::BinaryOp(operator, Box::new(left), Box::new(right));
        }
        Some(left)
    }

    pub fn parse(&mut self) -> Option<ASTNode> {
        self.parse_expr()
    }
}
