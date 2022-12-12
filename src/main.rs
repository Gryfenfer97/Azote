mod token;
mod lexer;
mod parser;
mod node;
mod interpreter;
mod value;
mod functions;

fn main(){
    let code = "(1 2 3) (1 2 3) (3 4 5)".to_string();
    // let code= "+/1 2 3 10".to_string();
    // let code = "×⍨ 4 3 56".to_string();
    // let code = "×⍨ 4".to_string();
    // let code = "×¯3".to_string();
    // let code = "1 3 4 0 9 ⌈ 1.2 2 3 4 5".to_string();
    let mut tokens = lexer::scan_tokens(&code).unwrap();
    tokens.reverse();
    // println!("{:?}", tokens);
    let ast: node::Node = parser::parse(&tokens).unwrap();
    println!("{:?}", ast);
    println!("\n\n> {}", code);
    let mut interpreter: interpreter::Interpreter = interpreter::Interpreter::new();
    let _v = interpreter.interpret(&ast);

    // let v = interpreter::interpret(&ast);
    // println!("{:?}", v);

}