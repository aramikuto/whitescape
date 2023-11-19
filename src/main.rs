mod ast;
mod lexer;

fn main() {
    let input = "push +1 output_as_number exit";
    let tokens = lexer::tokenize(input);

    let ast = ast::build_ast(&tokens);

    #[cfg(debug_assertions)]
    println!("{:#?}", ast);

    let code = ast::generate_code(&ast);
    println!("{}", code);
}
