use std::fs;

use crate::transpiler::CodeOutput;

mod ast;
mod lexer;
mod specs;
mod transpiler;

mod util;

fn main() {
    let code = "
    int m = 11;
    print(m);
    exit;
    ";
    let mut lexer = lexer::Lexer::new(code);
    let tokens: Vec<lexer::Token> = lexer.tokenize();
    println!("---TOKENS---\n{:?}\n", tokens);

    match ast::parse(&tokens) {
        Ok(ast) => {
            println!("--AST--\n{:#?}\n", ast);
            let CodeOutput { code, debug_code } = transpiler::transpile(ast);
            println!("---DEBUG---\n{}\n", debug_code);
            println!("---VISIBLE---\n{}\n", util::unbleach(code.clone()));
            fs::create_dir_all("out").expect("Unable to create directory");
            fs::write("out/a.out", code).expect("Unable to write file");
        }
        Err(err) => println!("Error: {}", err),
    }
}
