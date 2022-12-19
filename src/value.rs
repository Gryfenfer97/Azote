use std::fmt;
use std::rc::Rc;
use crate::node::Node;


pub struct MonadicFunctionHolder {
    pub function: Rc<dyn Fn(&Value) -> Result<Value, String>>,
    pub node: Node,
}

impl fmt::Debug for MonadicFunctionHolder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.node)
    }
}

pub struct DyadicFunctionHolder {
    pub function: Rc<dyn Fn(&Value, &Value) -> Result<Value, String>>,
    pub node: Node,
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