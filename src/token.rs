#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    Paren(char),
    Function(char),
    MonadicOperator(char),
    Assign,
    Number(f32),
    // String(String),
    Id(String),
    Diamond,
    Eof,
}