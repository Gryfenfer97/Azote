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


pub mod visit {
    use crate::node::Node;
    use crate::token::Token;

    pub trait Visitor<T> {
        fn visit_scalar(&mut self, token: &Token) -> T;
        fn visit_array(&mut self, values: &Vec<Node>) -> T;
        fn visit_monad(&mut self, operator: &Node, right: &Node) -> T;
        fn visit_dyad(&mut self, alpha: &Node, operator: &Node, omega: &Node) -> T;
        fn visit_f(&mut self, token: &Token, valence: i32) -> T;
        fn visit_stmt(&mut self, children: &Vec<Node>) -> T;
    }

    pub trait Acceptor<T> {
        fn accept(&self, visitor: &mut dyn Visitor<T>, valence: Option<i32>) -> T;
    }
}

impl<T> visit::Acceptor<T> for Node{
    fn accept(&self, visitor: &mut dyn visit::Visitor<T>, valence: Option<i32>) -> T {
        match self {
            Node::Scalar{token} => visitor.visit_scalar(token),
            Node::Array{values} => visitor.visit_array(values),
            Node::Monad{operator, right} => visitor.visit_monad(operator, right.as_ref().unwrap()),
            Node::Dyad{left, operator, right} => visitor.visit_dyad(left.as_ref().unwrap(), operator, right.as_ref().unwrap()),
            Node::F{token} => visitor.visit_f(token, valence.unwrap_or(1)),
            Node::Statement{children} => visitor.visit_stmt(children),
            _ => todo!()
        }
    }
}