use crate::lexer::Token;

#[derive(Debug)]
pub enum Expr {
    Number(i64),
    Command(Command),
}

#[derive(Debug)]
pub enum Command {
    Push(i64),
    OutputAsNumber,
    Exit,
}

pub fn build_ast(tokens: &[Token]) -> Vec<Expr> {
    let mut ast = Vec::new();
    let mut iter = tokens.iter().peekable();

    while let Some(token) = iter.next() {
        match token {
            Token::Push => {
                if let Some(&Token::Number(value)) = iter.peek() {
                    iter.next(); // Consume the Number token
                    ast.push(Expr::Command(Command::Push(*value)));
                } else {
                    panic!("Expected a number after 'push' command!");
                }
            }
            Token::OutputAsNumber => ast.push(Expr::Command(Command::OutputAsNumber)),
            Token::Exit => ast.push(Expr::Command(Command::Exit)),
            Token::Number(value) => ast.push(Expr::Number(*value)),
            _ => {
                // Handle other tokens or syntax errors
                panic!("Invalid token encountered while building AST!");
            }
        }
    }

    ast
}

fn convert_number(value: i64) -> String {
    let mut code = String::new();

    if value >= 0 {
        code.push_str(" ");
    } else {
        code.push_str(r"\t");
    }
    let mut value = value;
    while value != 0 {
        if value % 2 == 0 {
            code.push_str(" ");
        } else {
            code.push_str(r"\t");
        }
        value /= 2;
    }

    code.push_str("\\n");

    code
}

pub fn generate_code(ast: &[Expr]) -> String {
    let mut code: String = String::new();

    for expr in ast {
        match expr {
            Expr::Number(_) => {
                panic!("Unexpected number in AST");
            }
            Expr::Command(command) => match command {
                Command::Push(arg) => code.push_str(&format!("  {}", convert_number(*arg))),
                Command::OutputAsNumber => code.push_str(r"\t\n \t"),
                Command::Exit => code.push_str(r"\n\n\n"),
            },
        }
    }

    code
}

#[cfg(test)]
mod tests {
    use crate::lexer;

    use super::*;

    #[test]
    fn test_code_generation() {
        let input = "push +1 output_as_number exit";
        let tokens = lexer::tokenize(input);
        let ast = build_ast(&tokens);

        let expected_output = r"   \t\n\t\n \t\n\n\n";
        let generated_code = generate_code(&ast);

        assert_eq!(generated_code, expected_output);
    }
}
