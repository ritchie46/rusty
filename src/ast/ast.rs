use crate::lexer::Lexer;
use crate::token::{Token, TokenType};
use std::fmt::Error;

#[derive(Debug)]
enum Statement {
    Let(LetStatement),
}

#[derive(Debug)]
enum Expression {
    Some,
}

type ParseResult<T> = Result<T, String>;

trait Node {
    fn token_literal(&self) -> String;
}

#[derive(Debug)]
struct Identifier {
    token: Token,
    value: String,
}

#[derive(Debug)]
struct LetStatement {
    token: Token, // temporarely
    name: Identifier,
    value: Expression,
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

#[derive(Debug)]
pub struct Program {
    statements: Vec<Statement>,
}

pub struct Parser<'a> {
    lex: &'a mut Lexer<'a>,
    current_token: Token,
    peek_token: Token,
}
impl<'a> Parser<'a> {
    pub fn new(lex: &'a mut Lexer<'a>) -> Parser<'a> {
        let current = lex.next_token();
        let peek = lex.next_token();

        let p = Parser {
            lex,
            current_token: current,
            peek_token: peek,
        };
        p
    }

    fn next_token(&mut self) {
        // cannot reference because we replace peek
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lex.next_token();
    }

    pub fn parse_program(&mut self) -> Result<Program, String> {
        let mut program = Program { statements: vec![] };

        while self.current_token.type_ != TokenType::EOF {
            let stmt = self
                .parse_statement()?;

            program.statements.push(stmt);
            self.next_token();
        }
        Ok(program)
    }

    fn parse_statement(&self) -> ParseResult<Statement> {
        let tkn = self.current_token.clone();
        let name = Identifier {
            token: tkn.clone(),
            value: "some".to_string(),
        };

        Ok(Statement::Let(LetStatement {
            token: tkn,
            name,
            value: Expression::Some,
        }))
    }
}
