mod token;
mod lexer;
mod parser;
mod interpreter;

fn main(){
    let code = "×⍨ 4 3 56".to_string();
    let mut tokens = lexer::scan_tokens(&code).unwrap();
    tokens.reverse();
    println!("{:?}", tokens);
    let ast = parser::parse(&tokens).unwrap();
    println!("{:?}", ast);
}