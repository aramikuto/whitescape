use wasm_bindgen::prelude::*;
use web_sys::console;

mod ast;
mod lexer;
mod specs;
mod transpiler;
mod util;

#[wasm_bindgen]
pub fn gen_ast(code: &str) -> JsValue {
    let mut lexer = lexer::Lexer::new(code);
    let tokens: Vec<lexer::Token> = lexer.tokenize();
    let ast = ast::parse(&tokens);
    JsValue::from_str(format!("{:#?}", ast).as_str())
}

#[wasm_bindgen]
pub struct InterpreterOutput {
    ast: String,
    debug_output: String,
    whitespace_output: String,
}

#[wasm_bindgen]
impl InterpreterOutput {
    pub fn get_ast(&self) -> String {
        self.ast.clone()
    }

    pub fn get_debug_output(&self) -> String {
        self.debug_output.clone()
    }

    pub fn get_whitespace_output(&self) -> String {
        self.whitespace_output.clone()
    }
}

#[wasm_bindgen]
pub fn gen_all(code: &str) -> InterpreterOutput {
    let mut lexer = lexer::Lexer::new(code);
    let tokens: Vec<lexer::Token> = lexer.tokenize();
    let ast = ast::parse(&tokens);
    match ast {
        Ok(ast) => {
            let ast_output = format!("{:#?}", ast);
            let transpiler::CodeOutput { code, debug_code } = transpiler::transpile(ast, None);
            InterpreterOutput {
                ast: ast_output,
                debug_output: debug_code,
                whitespace_output: code,
            }
        }
        Err(err) => InterpreterOutput {
            ast: format!("{:#?}", err),
            debug_output: format!("{:#?}", err),
            whitespace_output: format!("{:#?}", err),
        },
    }
}

#[wasm_bindgen(start)]
fn main() {
    console_error_panic_hook::set_once();
    console::log_1(&JsValue::from_str("Set panic hook!"))
}
