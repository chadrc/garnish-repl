use std::env;
use std::fs;
use garnish_lang_compiler::{make_ast, build_byte_code, Lexer, Parser};
use garnish_lang_runtime::{ExpressionRuntime};

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(filename) => {
            let input = fs::read_to_string(filename)
                .expect("Could not read file.");
    
            let lexed = Lexer::new().lex(&input).unwrap();
            let parsed = Parser::new().make_groups(&lexed).unwrap();
            let ast = make_ast(parsed).unwrap();
            let instructions = build_byte_code("main", ast).unwrap();

            let mut runtime = ExpressionRuntime::new(&instructions);
            let result = runtime.execute("main").unwrap();

            println!("{}", result.as_integer().unwrap());
        }
        None => println!("Please provide a file")
    }
}
