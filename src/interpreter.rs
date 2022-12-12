use crate::node::visit::{Acceptor, Visitor};
use crate::node::Node;
use crate::token::Token;
use std::fmt;
use std::rc::Rc;

pub struct MonadicFunctionHolder {
    pub function: Rc<dyn Fn(&Value) -> Result<Value, String>>,
    node: Node,
}

impl fmt::Debug for MonadicFunctionHolder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.node)
    }
}

pub struct DyadicFunctionHolder {
    pub function: Rc<dyn Fn(&Value, &Value) -> Result<Value, String>>,
    node: Node,
}

impl fmt::Debug for DyadicFunctionHolder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.node)
    }
}

#[derive(Debug)]
pub enum Value {
    Number(f32),
    // String(String),
    Array(Vec<Value>),
    MonadicFunction(MonadicFunctionHolder),
    DyadicFunction(DyadicFunctionHolder),
    None,
}

pub struct Interpreter {
    // environment: Vec<Node>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Self {
            // environment: Vec::new(),
        }
    }

    pub fn interpret(&mut self, node: &Node) -> Result<Value, String> {
        self.visit_node(node, None)
    }

    fn visit_node(&mut self, node: &Node, valence: Option<i32>) -> Result<Value, String> {
        node.accept(self, valence)
    }


}

impl Visitor<Result<Value, String>> for Interpreter {
    fn visit_scalar(&mut self, token: &Token) -> Result<Value, String> {
        match token {
            Token::Number(value) => Ok(Value::Number(*value)),
            _ => Err("scalar must be a number".to_string()),
        }
    }

    fn visit_array(&mut self, values: &Vec<Node>) -> Result<Value, String> {
        let mut array: Vec<Value> = Vec::new();
        for value in values {
            array.push(self.visit_node(value, None)?)
        }
        Ok(Value::Array(array))
    }

    // Evaluate a function that have only one argument
    fn visit_monad(&mut self, operator: &Node, right: &Node) -> Result<Value, String> {
        match operator{
            Node::MonadicOperator{operator, child} => {
                match operator{
                    Token::MonadicOperator('⍨') => {
                        return self.visit_dyad(right, child.as_ref().unwrap(), right);
                    }
                    _ => {
                        Err("Operator not implemented".to_string())
                    }
                }
            }
            _ => {
                let function: Value = self.visit_node(operator, Some(1))?;
                if let Value::MonadicFunction(f) = function {
                    let omega: Value = self.visit_node(right, None)?;
                    return Ok((f.function)(&omega)?);
                }
                Err("Problem".to_string())
            }
        }
    }

    fn visit_dyad(&mut self, alpha: &Node, operator: &Node, omega: &Node) -> Result<Value, String> {
        let function: Value = self.visit_node(operator, Some(2))?;
        if let Value::DyadicFunction(f) = function {
            let alpha_v: Value = self.visit_node(alpha, None)?;
            let omega_v: Value = self.visit_node(omega, None)?;
            return Ok((f.function)(&alpha_v, &omega_v)?);
        }
        Err("Problem".to_string())
    }

    fn visit_f(&mut self, token: &Token, valence: i32) -> Result<Value, String> {
        match token {
            Token::Function('×') => match valence {
                1 => {
                    return Ok(Value::MonadicFunction(MonadicFunctionHolder {
                        function: Rc::new(|v: &Value| {
                            match v {
                                Value::Number(value) => {return Ok(Value::Number(value / value.abs()));},
                                Value::Array(values) => {
                                    let mut vector: Vec<Value> = Vec::new();
                                    for value in values{
                                        if let Value::Number(n) = value {
                                            vector.push(Value::Number(n / n.abs()))
                                        }
                                    }
                                    return Ok(Value::Array(vector));
                                },
                                _ => {return Err("Domain Error".to_string());}
                            }
                        }),
                        node: Node::F {
                            token: token.clone(),
                        },
                    }));
                }
                2 => {
                    return Ok(Value::DyadicFunction(DyadicFunctionHolder {
                        function: Rc::new(|alpha, omega| {
                            match (alpha, omega) {
                                (Value::Number(value1), Value::Number(value2)) => {
                                    return Ok(Value::Number(value1 * value2));
                                }
                                (Value::Array(values1), Value::Array(values2)) => {
                                    if values1.len() != values2.len() {return Err("Vectors don't have the same rank".to_string());}
                                    let mut vector: Vec<Value> = Vec::new();
                                    for i in 0..values1.len(){
                                        if let (Value::Number(v1), Value::Number(v2)) = (&values1[i], &values2[i]) {
                                            vector.push(Value::Number(v1*v2))
                                        }
                                    }
                                    return Ok(Value::Array(vector));
                                }
                                _ => unreachable!()
                            }
                        }),
                        node: Node::F {
                            token: token.clone(),
                        },
                    }));
                }
                _ => return Err("Bad valence".to_string()),
            },
            Token::Function('⌈') => match valence {
                1 => {
                    return Ok(Value::MonadicFunction(MonadicFunctionHolder {
                        function: Rc::new(|v: &Value| {
                            match v {
                                Value::Number(value) => {return Ok(Value::Number(value.ceil()));},
                                Value::Array(values) => {
                                    let mut vector: Vec<Value> = Vec::new();
                                    for value in values{
                                        if let Value::Number(n) = value {
                                            vector.push(Value::Number(n.ceil()))
                                        }
                                    }
                                    return Ok(Value::Array(vector));
                                },
                                _ => {return Err("Domain Error".to_string());}
                            }
                        }),
                        node: Node::F {
                            token: token.clone(),
                        },
                    }));
                }
                2 => {
                    return Ok(Value::DyadicFunction(DyadicFunctionHolder {
                        function: Rc::new(|alpha, omega| {
                            match (alpha, omega) {
                                (Value::Number(value1), Value::Number(value2)) => {
                                    return Ok(Value::Number(f32::max(*value1, *value2)));
                                }
                                (Value::Array(values1), Value::Array(values2)) => {
                                    if values1.len() != values2.len() {return Err("Vectors don't have the same rank".to_string());}
                                    let mut vector: Vec<Value> = Vec::new();
                                    for i in 0..values1.len(){
                                        if let (Value::Number(v1), Value::Number(v2)) = (&values1[i], &values2[i]) {
                                            vector.push(Value::Number(f32::max(*v1, *v2)))
                                        }
                                    }
                                    return Ok(Value::Array(vector));
                                }
                                _ => unreachable!()
                            }
                        }),
                        node: Node::F {
                            token: token.clone(),
                        },
                    }));
                }
                _ => return Err("Bad valence".to_string()),
            }
            Token::Function('⌊') => match valence {
                1 => {
                    return Ok(Value::MonadicFunction(MonadicFunctionHolder {
                        function: Rc::new(|v: &Value| {
                            match v {
                                Value::Number(value) => {return Ok(Value::Number(value.floor()));},
                                Value::Array(values) => {
                                    let mut vector: Vec<Value> = Vec::new();
                                    for value in values{
                                        if let Value::Number(n) = value {
                                            vector.push(Value::Number(n.floor()))
                                        }
                                    }
                                    return Ok(Value::Array(vector));
                                },
                                _ => {return Err("Domain Error".to_string());}
                            }
                        }),
                        node: Node::F {
                            token: token.clone(),
                        },
                    }));
                }
                2 => {
                    return Ok(Value::DyadicFunction(DyadicFunctionHolder {
                        function: Rc::new(|alpha, omega| {
                            match (alpha, omega) {
                                (Value::Number(value1), Value::Number(value2)) => {
                                    return Ok(Value::Number(f32::min(*value1, *value2)));
                                }
                                (Value::Array(values1), Value::Array(values2)) => {
                                    if values1.len() != values2.len() {return Err("Vectors don't have the same rank".to_string());}
                                    let mut vector: Vec<Value> = Vec::new();
                                    for i in 0..values1.len(){
                                        if let (Value::Number(v1), Value::Number(v2)) = (&values1[i], &values2[i]) {
                                            vector.push(Value::Number(f32::min(*v1, *v2)))
                                        }
                                    }
                                    return Ok(Value::Array(vector));
                                }
                                _ => unreachable!()
                            }
                        }),
                        node: Node::F {
                            token: token.clone(),
                        },
                    }));
                }
                _ => return Err("Bad valence".to_string()),
            }
            _ => {
                println!("token: {:?}", token);
                return Err("Parsing error".to_string());
            }
        }
    }

    fn visit_stmt(&mut self, children: &Vec<Node>) -> Result<Value, String> {
        for statement in children {
            let v = self.visit_node(statement, None)?;
            println!("{:?}", v)
        }
        Ok(Value::None)
    }

}

