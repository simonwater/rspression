use crate::token::Token;
use crate::values::Value;

pub enum Expr {
    Binary(BinaryExpr),
    Logic(LogicExpr),
    Literal(LiteralExpr),
    Unary(UnaryExpr),
    Id(IdExpr),
    Assign(AssignExpr),
    Call(CallExpr),
    If(IfExpr),
    Get(GetExpr),
    Set(SetExpr),
}

pub trait Visitor<R> {
    fn visit_binary(&mut self, expr: &BinaryExpr) -> R;
    fn visit_logic(&mut self, expr: &LogicExpr) -> R;
    fn visit_literal(&mut self, expr: &LiteralExpr) -> R;
    fn visit_unary(&mut self, expr: &UnaryExpr) -> R;
    fn visit_id(&mut self, expr: &IdExpr) -> R;
    fn visit_assign(&mut self, expr: &AssignExpr) -> R;
    fn visit_call(&mut self, expr: &CallExpr) -> R;
    fn visit_if(&mut self, expr: &IfExpr) -> R;
    fn visit_get(&mut self, expr: &GetExpr) -> R;
    fn visit_set(&mut self, expr: &SetExpr) -> R;
}

impl Expr {
    pub fn accept<R, V: Visitor<R>>(&self, visitor: &mut V) -> R {
        match self {
            Expr::Binary(expr) => visitor.visit_binary(expr),
            Expr::Logic(expr) => visitor.visit_logic(expr),
            Expr::Literal(expr) => visitor.visit_literal(expr),
            Expr::Unary(expr) => visitor.visit_unary(expr),
            Expr::Id(expr) => visitor.visit_id(expr),
            Expr::Assign(expr) => visitor.visit_assign(expr),
            Expr::Call(expr) => visitor.visit_call(expr),
            Expr::If(expr) => visitor.visit_if(expr),
            Expr::Get(expr) => visitor.visit_get(expr),
            Expr::Set(expr) => visitor.visit_set(expr),
        }
    }

    pub fn binary(left: Expr, operator: Token, right: Expr) -> Self {
        Expr::Binary(BinaryExpr {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }

    pub fn logic(left: Expr, operator: Token, right: Expr) -> Self {
        Expr::Logic(LogicExpr {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }

    pub fn literal(value: Value) -> Self {
        Expr::Literal(LiteralExpr { value })
    }

    pub fn unary(operator: Token, right: Expr) -> Self {
        Expr::Unary(UnaryExpr {
            operator,
            right: Box::new(right),
        })
    }

    pub fn id(token: Token) -> Self {
        Expr::Id(IdExpr { name: token })
    }

    pub fn assign(left: Expr, operator: Token, right: Expr) -> Self {
        Expr::Assign(AssignExpr {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }

    pub fn call(callee: Expr, arguments: Vec<Expr>, r_paren: Token) -> Self {
        Expr::Call(CallExpr {
            callee: Box::new(callee),
            arguments,
            r_paren,
        })
    }

    pub fn if_expr(condition: Expr, then_branch: Expr, else_branch: Option<Expr>) -> Self {
        Expr::If(IfExpr {
            condition: Box::new(condition),
            then_branch: Box::new(then_branch),
            else_branch: else_branch.map(Box::new),
        })
    }

    pub fn get(object: Expr, name: Token) -> Self {
        Expr::Get(GetExpr {
            object: Box::new(object),
            name,
        })
    }

    pub fn set(object: Expr, name: Token, value: Expr) -> Self {
        Expr::Set(SetExpr {
            object: Box::new(object),
            name,
            value: Box::new(value),
        })
    }
}

pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

pub struct LogicExpr {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

pub struct LiteralExpr {
    pub value: Value,
}

pub struct UnaryExpr {
    pub operator: Token,
    pub right: Box<Expr>,
}

pub struct IdExpr {
    pub name: Token,
}

pub struct AssignExpr {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

pub struct CallExpr {
    pub callee: Box<Expr>,
    pub arguments: Vec<Expr>,
    pub r_paren: Token,
}

pub struct IfExpr {
    pub condition: Box<Expr>,
    pub then_branch: Box<Expr>,
    pub else_branch: Option<Box<Expr>>,
}

pub struct GetExpr {
    pub object: Box<Expr>,
    pub name: Token,
}

pub struct SetExpr {
    pub object: Box<Expr>,
    pub name: Token,
    pub value: Box<Expr>,
}
