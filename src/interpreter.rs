use crate::node::visit::{Acceptor, Visitor};
use crate::node::Node;
use crate::token::Token;
use crate::value::{Value, DyadicFunctionHolder, MonadicFunctionHolder};
use std::rc::Rc;
use crate::functions::*;


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
        let omega = &value_to_node(&self.visit_node(right, Some(1))?);
        match operator{
            Node::MonadicOperator{operator, child} => {
                match operator{
                    Token::MonadicOperator('⍨') => {
                        return self.visit_dyad(omega, child.as_ref().unwrap(), omega);
                    }
                    Token::MonadicOperator('/') => {
                        if let Node::Array { values } = omega {
                            if values.len() <= 1 {return Err("trying to reduce on a single element array".to_string());}
                            let mut value = values[0].clone();
                            for i in 1..values.len(){
                                value = value_to_node(&self.visit_dyad(&value, child.as_ref().unwrap(), &values[i])?);
                            }
                            Ok(node_to_value(&value))
                        }
                        else {unreachable!()}
                    },
                    Token::MonadicOperator('¨') => {
                        if let Node::Array { values } = omega {
                            let mut vector: Vec<Value> = Vec::new();
                            for value in values {
                                vector.push(self.visit_monad(child.as_ref().unwrap(), value)?)
                            }
                            Ok(Value::Array(vector))
                        }
                        else {unreachable!()}
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
            Token::Function('+') => match valence {
                1 => {
                    return Ok(Value::MonadicFunction(MonadicFunctionHolder {
                        function: Rc::new(conjugate),
                        node: Node::F {
                            token: token.clone(),
                        },
                    }));
                }
                2 => {
                    return Ok(Value::DyadicFunction(DyadicFunctionHolder {
                        function: Rc::new(plus),
                        node: Node::F {
                            token: token.clone(),
                        },
                    }));
                }
                _ => return Err("Bad valence".to_string()),
            },
            Token::Function('×') => match valence {
                1 => {
                    return Ok(Value::MonadicFunction(MonadicFunctionHolder {
                        function: Rc::new(direction),
                        node: Node::F {
                            token: token.clone(),
                        },
                    }));
                }
                2 => {
                    return Ok(Value::DyadicFunction(DyadicFunctionHolder {
                        function: Rc::new(times),
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
                        function: Rc::new(ceiling),
                        node: Node::F {
                            token: token.clone(),
                        },
                    }));
                }
                2 => {
                    return Ok(Value::DyadicFunction(DyadicFunctionHolder {
                        function: Rc::new(maximum),
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
                        function: Rc::new(floor),
                        node: Node::F {
                            token: token.clone(),
                        },
                    }));
                }
                2 => {
                    return Ok(Value::DyadicFunction(DyadicFunctionHolder {
                        function: Rc::new(minimum),
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



fn value_to_node(value: &Value) -> Node{
    match value{
        Value::Array(values) => {return Node::Array { values: values.iter().map(|el| return value_to_node(el)).collect() }},
        Value::Number(v) => {return Node::Scalar { token: Token::Number(*v) }}
        _ => unreachable!()
    }
}

fn node_to_value(node: &Node) -> Value{
    match node{
        Node::Array { values } => {return Value::Array(values.iter().map(|el| return node_to_value(el)).collect())}
        Node::Scalar { token } => {
            match token {
                Token::Number(x) => {return Value::Number(*x);}
                _ => todo!()
            }
        }
        _ => unreachable!()
    }
}