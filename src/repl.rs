use crate::evaluator::eval;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::token::TokenType;
use std::io;
use std::io::Write;

const PROMPT: &str = ">>";

pub fn start() {
    let io_in = io::stdin();
    loop {
        // Use stdout() instead of print! macro
        // Print macro gets flushed when new line is encountered
        io::stdout().write_all([PROMPT, " "].concat().as_bytes());
        Write::flush(&mut io::stdout());

        let mut input = String::new();
        io_in.read_line(&mut input);

        let mut lex = Lexer::new(&input);
        let mut par = Parser::new(&mut lex);
        let program_ast = par.parse_program().unwrap();
        println!("{}", eval(&program_ast))
    }
}
