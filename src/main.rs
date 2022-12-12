mod token;
mod lexer;
mod parser;
mod node;
mod interpreter;

fn main(){
    // let code = "×⍨ 4 3 56".to_string();
    // let code = "×⍨ 4".to_string();
    // let code = "×¯3".to_string();
    let code = "1 3 4 0 9 ⌈ 1.2 2 3 4 5".to_string();
    let mut tokens = lexer::scan_tokens(&code).unwrap();
    tokens.reverse();
    // println!("{:?}", tokens);
    let ast = parser::parse(&tokens).unwrap();
    // println!("{:?}", ast);
    println!("\n\n> {}", code);
    let mut interpreter = interpreter::Interpreter::new();
    let _v = interpreter.interpret(&ast);

    // let v = interpreter::interpret(&ast);
    // println!("{:?}", v);

}