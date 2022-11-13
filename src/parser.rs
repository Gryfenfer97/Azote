use crate::token::Token;

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Scalar {
        token: Token,
    },

    Array {
        values: Vec<Node>,
    },

    MonadicOperator {
        operator: Token,
        child: Option<Box<Node>>,
    },
    Dyad {
        left: Option<Box<Node>>,
        operator: Box<Node>,
        right: Option<Box<Node>>,
    },
    F {
        token: Token,
    },
    Monad {
        operator: Box<Node>,
        right: Option<Box<Node>>,
    },
    Assignement {
        varname: Box<Node>,
        value: Box<Node>,
    },
    Var {
        token: Token,
    },
    Statement {
        children: Vec<Node>,
    },
}

pub fn parse(tokens: &Vec<Token>) -> Result<Node, String> {
    let statement_list: Node = parse_statement_list(&tokens, 0)?;
    Ok(statement_list)
}

fn parse_statement_list(tokens: &Vec<Token>, mut index: usize) -> Result<Node, String> {
    let mut statement_list: Vec<Node> = Vec::new();
    let mut statement: Node;
    (statement, index) = parse_statement(tokens, index)?;
    statement_list.push(statement);
    while tokens[index] == Token::Diamond {
        index += 1;
        (statement, index) = parse_statement(tokens, index)?;
        statement_list.push(statement);
    }
    let root = Node::Statement {
        children: statement_list,
    };
    Ok(root)
}

fn parse_statement(tokens: &Vec<Token>, mut index: usize) -> Result<(Node, usize), String> {
    println!("Parsing statement from {:?}", &tokens[index..]);
    let mut statement: Node;
    (statement, index) = parse_array(tokens, index)?;
    loop {
        match tokens[index] {
            Token::Assign => {
                index += 1;
                statement = Node::Assignement {
                    varname: Box::new(Node::Var {
                        token: tokens[index].clone(),
                    }),
                    value: Box::new(statement),
                }
            }
            Token::Function(_) | Token::MonadicOperator(_) => {
                let func: Node;
                (func, index) = parse_function(tokens, index)?;
                match tokens[index] {
                    Token::Paren(')') | Token::Number(_) | Token::Id(_) => {
                        let array;
                        (array, index) = parse_array(tokens, index)?;
                        statement = Node::Dyad {
                            left: Some(Box::new(array)),
                            operator: Box::new(func),
                            right: Some(Box::new(statement)),
                        }
                    }
                    _ => {
                        statement = Node::Monad {
                            operator: Box::new(func),
                            right: Some(Box::new(statement)),
                        }
                    }
                }
            }
            _ => {
                break;
            }
        }
    }
    return Ok((statement, index));
}

pub fn parse_array(tokens: &Vec<Token>, mut index: usize) -> Result<(Node, usize), String> {
    println!("Parsing array from {:?}", &tokens[index..]);
    let mut node: Vec<Node> = [].to_vec();
    loop {
        match tokens[index] {
            Token::Paren(')') => {
                index += 1;
                let statement: Node;
                (statement, index) = parse_statement(tokens, index)?;
                node.push(statement);
                index = eat(tokens, index, Token::Paren('('))?;
            }
            Token::Number(_) => {
                node.push(Node::Scalar {
                    token: tokens[index].clone(),
                });
                index += 1;
            }
            _ => {
                break;
            }
        }
    }
    node.reverse();
    match node.len() {
        0 => {
            return Err("Failed to parse scalars inside an array.".to_string());
        }
        1 => {
            return Ok((node[0].clone(), index));
        }
        _ => {
            return Ok((Node::Array { values: node }, index));
        }
    }
}

fn parse_function(tokens: &Vec<Token>, mut index: usize) -> Result<(Node, usize), String> {
    println!("Parsing function from {:?}", &tokens[index..]);
    let mut node: Node;
    match tokens[index] {
        Token::MonadicOperator(_) => {
            (node, index) = parse_mop(tokens, index)?;
            let function;
            (function, index) = parse_function(tokens, index)?;
            if let Node::MonadicOperator {
                operator: _,
                ref mut child,
            } = node
            {
                *child = Some(Box::new(function));
            }
        }
        _ => {
            (node, index) = parse_f(tokens, index)?;
        }
    }
    return Ok((node, index));
}

fn parse_mop(tokens: &Vec<Token>, mut index: usize) -> Result<(Node, usize), String> {
    println!("Parsing mop from {:?}", &tokens[index..]);
    let mop: Node = Node::MonadicOperator {
        operator: tokens[index].clone(),
        child: None,
    };
    index += 1;
    Ok((mop, index))
}

fn parse_f(tokens: &Vec<Token>, mut index: usize) -> Result<(Node, usize), String> {
    println!("Parsing f from {:?}", &tokens[index..]);
    let node: Node = Node::F {
        token: tokens[index].clone(),
    };
    index += 1;
    Ok((node, index))
}

fn eat(tokens: &Vec<Token>, index: usize, token: Token) -> Result<usize, String> {
    if tokens[index] == token {
        return Ok(index);
    }
    return Err("Bad token".to_string());
}
