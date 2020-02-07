#[cfg(test)]
mod test {
    use crate::ast::*;
    use crate::err::ParserError;
    use crate::lexer::Lexer;
    use crate::parser::*;

    fn parse_program(input: &str) -> Result<Program, ParserError> {
        let mut lex = Lexer::new(&input);
        let mut par = Parser::new(&mut lex);
        par.parse_program()
    }

    #[test]
    fn test_parser_errors() {
        let input = "let x;";
        let parsed = parse_program(&input).unwrap_err();
        match parsed {
            ParserError::AssignmentExpected(_) => assert!(true),
            _ => assert!(false),
        }
        let input = "let =";
        let parsed = parse_program(&input).unwrap_err();
        match parsed {
            ParserError::IdentifierExpected => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_precedence() {
        assert!(Precedence::Lowest < Precedence::Equals);
        assert!(Precedence::Prefix == Precedence::Prefix)
    }

    #[test]
    fn test_identifier_expression() {
        let input = "foobar;";
        let parsed = parse_program(&input);
        assert_eq!(
            Statement::Expr(Expression::Identifier("foobar".to_string())),
            parsed.unwrap().statements[0]
        );
    }

    #[test]
    fn test_integer_literal_expression() {
        let input = "5;";
        let parsed = parse_program(&input);
        assert_eq!(
            Statement::Expr(Expression::IntegerLiteral(5 as i64)),
            parsed.unwrap().statements[0]
        );
    }

    #[test]
    fn test_prefix_expression() {
        let input = "-5;";
        let parsed = parse_program(&input);
        assert_eq!(
            Statement::Expr(Expression::Prefix(
                "-".to_string(),
                Box::new(Expression::IntegerLiteral(5 as i64))
            )),
            parsed.unwrap().statements[0]
        );
    }

    fn test_operator_precedence_parsing(inputs: &[&str], outputs: &[&str]) {
        for (input, output) in inputs.iter().zip(outputs) {
            let parsed = parse_program(input).unwrap();
            assert_eq!(format!("{}", parsed.statements[0]), *output)
        }
    }

    #[test]
    fn test_infix_expression() {
        let input = "-5 == 10;";
        let parsed = parse_program(&input).unwrap();
        let stmt = Statement::Expr(Expression::Infix(
            Box::new(Expression::Prefix(
                "-".to_string(),
                Box::new(Expression::IntegerLiteral(5 as i64)),
            )),
            "==".to_string(),
            Box::new(Expression::IntegerLiteral(10 as i64)),
        ));
        assert_eq!(stmt, parsed.statements[0]);

        let inputs = [
            "a + b * c + d / e - f",
            "a != 10;",
            "c > 6;",
            "3 + 4 * 5 == 3 * 1 + 4 * 5",
        ];
        let outputs = [
            "(((a + (b * c)) + (d / e)) - f)",
            "(a != 10)",
            "(c > 6)",
            "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
        ];
        test_operator_precedence_parsing(&inputs, &outputs)
    }

    #[test]
    fn test_bool_expression() {
        let inputs = ["true", "false", "3 > 5 == false"];
        let outputs = ["true", "false", "((3 > 5) == false)"];
        test_operator_precedence_parsing(&inputs, &outputs)
    }

    #[test]
    fn test_grouped_expression() {
        let inputs = ["1 + (2 + 3) + 4"];
        let outputs = ["((1 + (2 + 3)) + 4)"];
        test_operator_precedence_parsing(&inputs, &outputs)
    }
}
