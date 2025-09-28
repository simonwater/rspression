use crate::token::Token;
use crate::value::Value;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Literal {
        value: Value,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Variable {
        name: Token,
    },
    Assign {
        name: Token,
        value: Box<Expr>,
    },
    Logical {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        paren: Token,
        arguments: Vec<Expr>,
    },
    Get {
        object: Box<Expr>,
        name: Token,
    },
    Set {
        object: Box<Expr>,
        name: Token,
        value: Box<Expr>,
    },
    If {
        condition: Box<Expr>,
        then_branch: Box<Expr>,
        else_branch: Option<Box<Expr>>,
    },
}

impl Expr {
    pub fn binary(left: Expr, operator: Token, right: Expr) -> Self {
        Expr::Binary {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }

    pub fn unary(operator: Token, right: Expr) -> Self {
        Expr::Unary {
            operator,
            right: Box::new(right),
        }
    }

    pub fn literal(value: Value) -> Self {
        Expr::Literal { value }
    }

    pub fn grouping(expression: Expr) -> Self {
        Expr::Grouping {
            expression: Box::new(expression),
        }
    }

    pub fn variable(name: Token) -> Self {
        Expr::Variable { name }
    }

    pub fn assign(name: Token, value: Expr) -> Self {
        Expr::Assign {
            name,
            value: Box::new(value),
        }
    }

    pub fn logical(left: Expr, operator: Token, right: Expr) -> Self {
        Expr::Logical {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }

    pub fn call(callee: Expr, paren: Token, arguments: Vec<Expr>) -> Self {
        Expr::Call {
            callee: Box::new(callee),
            paren,
            arguments,
        }
    }

    pub fn get(object: Expr, name: Token) -> Self {
        Expr::Get {
            object: Box::new(object),
            name,
        }
    }

    pub fn set(object: Expr, name: Token, value: Expr) -> Self {
        Expr::Set {
            object: Box::new(object),
            name,
            value: Box::new(value),
        }
    }

    pub fn if_expr(condition: Expr, then_branch: Expr, else_branch: Option<Expr>) -> Self {
        Expr::If {
            condition: Box::new(condition),
            then_branch: Box::new(then_branch),
            else_branch: else_branch.map(Box::new),
        }
    }
}
