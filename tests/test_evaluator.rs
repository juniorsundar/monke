use monke::{lexer::Lexer, object::Object, parser::Parser};

#[test]
fn test_eval_integer_expression() {}

fn test_eval(input: String) -> Object {
    let l = Lexer::new(input);
    let p = Parser::new(l);
    let program = p.parse_program();
}
