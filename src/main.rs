mod token;
mod lexer;
mod parser;
mod node;
mod interpreter;

fn main(){
    let code = "×⍨ 4 3 56".to_string();
    // let code = "×⍨ 4".to_string();
    // let code = "×¯3".to_string();
    let mut tokens = lexer::scan_tokens(&code).unwrap();
    tokens.reverse();
    // println!("{:?}", tokens);
    let ast = parser::parse(&tokens).unwrap();
    println!("{:?}", ast);
    println!("\n\n> {}", code);
    let mut interpreter = interpreter::Interpreter::new();
    let v = interpreter.interpret(&ast);
    // let v = interpreter::interpret(&ast);
    // println!("{:?}", v);

}