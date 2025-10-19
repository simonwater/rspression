use crate::Token;
use crate::values::Value;
use std::rc::Rc;

pub enum Expr<'a> {
    Binary(BinaryExpr<'a>),
    Logic(LogicExpr<'a>),
    Literal(LiteralExpr),
    Unary(UnaryExpr<'a>),
    Id(IdExpr<'a>),
    Assign(AssignExpr<'a>),
    Call(CallExpr<'a>),
    If(IfExpr<'a>),
    Get(GetExpr<'a>),
    Set(SetExpr<'a>),
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

impl<'a> Expr<'a> {
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

    pub fn binary(left: Expr<'a>, operator: Rc<Token<'a>>, right: Expr<'a>) -> Expr<'a> {
        Expr::Binary(BinaryExpr {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }

    pub fn logic(left: Expr<'a>, operator: Rc<Token<'a>>, right: Expr<'a>) -> Self {
        Expr::Logic(LogicExpr {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }

    pub fn literal(value: Value) -> Self {
        Expr::Literal(LiteralExpr { value })
    }

    pub fn unary(operator: Rc<Token<'a>>, right: Expr<'a>) -> Self {
        Expr::Unary(UnaryExpr {
            operator,
            right: Box::new(right),
        })
    }

    pub fn id(token: Rc<Token<'a>>) -> Self {
        Expr::Id(IdExpr { name: token })
    }

    pub fn assign(left: Expr<'a>, operator: Rc<Token<'a>>, right: Expr<'a>) -> Self {
        Expr::Assign(AssignExpr {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }

    pub fn call(callee: Expr<'a>, arguments: Vec<Expr<'a>>, r_paren: Rc<Token<'a>>) -> Self {
        Expr::Call(CallExpr {
            callee: Box::new(callee),
            arguments,
            r_paren,
        })
    }

    pub fn if_expr(
        condition: Expr<'a>,
        then_branch: Expr<'a>,
        else_branch: Option<Expr<'a>>,
    ) -> Self {
        Expr::If(IfExpr {
            condition: Box::new(condition),
            then_branch: Box::new(then_branch),
            else_branch: else_branch.map(Box::new),
        })
    }

    pub fn get(object: Expr<'a>, name: Rc<Token<'a>>) -> Self {
        Expr::Get(GetExpr {
            object: Box::new(object),
            name,
        })
    }

    pub fn set(object: Expr<'a>, name: Rc<Token<'a>>, value: Expr<'a>) -> Self {
        Expr::Set(SetExpr {
            object: Box::new(object),
            name,
            value: Box::new(value),
        })
    }
}

pub struct BinaryExpr<'a> {
    pub left: Box<Expr<'a>>,
    pub operator: Rc<Token<'a>>,
    pub right: Box<Expr<'a>>,
}

pub struct LogicExpr<'a> {
    pub left: Box<Expr<'a>>,
    pub operator: Rc<Token<'a>>,
    pub right: Box<Expr<'a>>,
}

pub struct LiteralExpr {
    pub value: Value,
}

pub struct UnaryExpr<'a> {
    pub operator: Rc<Token<'a>>,
    pub right: Box<Expr<'a>>,
}

pub struct IdExpr<'a> {
    pub name: Rc<Token<'a>>,
}

pub struct AssignExpr<'a> {
    pub left: Box<Expr<'a>>,
    pub operator: Rc<Token<'a>>,
    pub right: Box<Expr<'a>>,
}

pub struct CallExpr<'a> {
    pub callee: Box<Expr<'a>>,
    pub arguments: Vec<Expr<'a>>,
    pub r_paren: Rc<Token<'a>>,
}

pub struct IfExpr<'a> {
    pub condition: Box<Expr<'a>>,
    pub then_branch: Box<Expr<'a>>,
    pub else_branch: Option<Box<Expr<'a>>>,
}

pub struct GetExpr<'a> {
    pub object: Box<Expr<'a>>,
    pub name: Rc<Token<'a>>,
}

pub struct SetExpr<'a> {
    pub object: Box<Expr<'a>>,
    pub name: Rc<Token<'a>>,
    pub value: Box<Expr<'a>>,
}
