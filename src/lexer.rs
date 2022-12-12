use crate::token::{Token};
use std::iter::Peekable;

pub fn scan_tokens(src: &String) -> Result<Vec<Token>, String>{
    let mut tokens: Vec<Token> = Vec::new();
    let mut it =  src.chars().peekable();
    tokens.push(Token::Eof);
    while let Some(&c) = it.peek(){
        match c{
            '0'..='9' | '¯' => {
                it.next();
                let num = number(c, &mut it);
                tokens.push(Token::Number(num));
            },
            'a'..='z' | 'A'..='Z' => {
                it.next();
                let id = get_id(c, &mut it);
                tokens.push(Token::Id(id));
            }
            '+' | '-' | '×' | '÷' => {
                tokens.push(Token::Function(c));
                it.next();
            },
            '⍨' => {
                tokens.push(Token::MonadicOperator(c));
                it.next();
            }
            '(' | ')' => {
                tokens.push(Token::Paren(c));
                it.next();
            },
            '←' => {
                tokens.push(Token::Assign);
                it.next();
            },
            '⋄' => {
                tokens.push(Token::Diamond);
                it.next();
            }
            ' ' => {it.next();},
            _ => {
                return Err(format!("unexpected character {}", c));
            }
        }
    }
    Ok(tokens)
}

fn number<T: Iterator<Item = char>>(mut c: char, iter: &mut Peekable<T>) -> f32{
    let mut negative: f32 = 1.;
    if c == '¯' {
        negative = -1.;
        c = iter.next().unwrap_or('a'); // We put a random letter that will fail the expect
    }
    let mut number = c.to_string().parse::<f32>().expect("The caller should have passed a digit.");
    while let Some(Ok(digit)) = iter.peek().map(|c| c.to_string().parse::<f32>()) {
        number = number * 10. + digit;
        iter.next();
    }
    number * negative
}

fn get_id<T: Iterator<Item = char>>(c: char, iter: &mut Peekable<T>) -> String{
    let mut id: String = c.to_string();
    while let Some(c) = iter.peek() {
        if !c.is_alphanumeric() {break;}
        id.push(*c);
        iter.next();
    }
    id
}
