use crate::expr::*;
use crate::parser::Parser;
use crate::visitors::VariableSet;

pub struct VarsQuery;

impl VarsQuery {
    pub fn new() -> Self {
        VarsQuery
    }

    fn execute_src(&mut self, expression: String) -> Option<VariableSet> {
        let expr = Parser::new(expression).parse().ok()?;
        self.execute(&expr)
    }

    fn execute(&mut self, expr: &Expr) -> Option<VariableSet> {
        expr.accept(self)
    }

    fn visit_get_inner(&self, expr: &Expr, names: &mut Vec<String>) {
        match expr {
            Expr::Id(id_expr) => {
                names.push(id_expr.name.lexeme.clone());
            }
            Expr::Get(get_expr) => {
                self.visit_get_inner(&get_expr.object, names);
                names.push(get_expr.name.lexeme.clone());
            }
            _ => {}
        }
    }
}

impl Visitor<Option<VariableSet>> for VarsQuery {
    fn visit_binary(&mut self, expr: &BinaryExpr) -> Option<VariableSet> {
        let mut result = self.execute(&expr.left);
        let rhs = self.execute(&expr.right);
        if result.is_none() {
            return rhs;
        }
        result.as_mut().unwrap().comebine(rhs);
        result
    }

    fn visit_logic(&mut self, expr: &LogicExpr) -> Option<VariableSet> {
        let mut result = self.execute(&expr.left);
        let rhs = self.execute(&expr.right);
        if result.is_none() {
            return rhs;
        }
        result.as_mut().unwrap().comebine(rhs);
        result
    }

    fn visit_literal(&mut self, _expr: &LiteralExpr) -> Option<VariableSet> {
        None
    }

    fn visit_unary(&mut self, expr: &UnaryExpr) -> Option<VariableSet> {
        self.execute(&expr.right)
    }

    fn visit_id(&mut self, expr: &IdExpr) -> Option<VariableSet> {
        Some(VariableSet::from_depends(&[&expr.name.lexeme]))
    }

    fn visit_assign(&mut self, expr: &AssignExpr) -> Option<VariableSet> {
        let AssignExpr { left, .. } = expr;
        if let Expr::Id(id_expr) = &**left {
            let mut result = VariableSet::from_assigns(&[&id_expr.name.lexeme]);
            let rhs = self.execute(&expr.right);
            result.comebine(rhs);
            Some(result)
        } else {
            None
        }
    }

    fn visit_call(&mut self, expr: &CallExpr) -> Option<VariableSet> {
        let mut result = VariableSet::default();
        for arg in &expr.arguments {
            let cur = self.execute(arg);
            result.comebine(cur);
        }
        Some(result)
    }

    fn visit_if(&mut self, expr: &IfExpr) -> Option<VariableSet> {
        let IfExpr {
            condition,
            then_branch,
            else_branch,
        } = expr;
        let mut result = VariableSet::default();
        result.comebine(self.execute(condition));
        result.comebine(self.execute(then_branch));
        if let Some(else_branch_expr) = else_branch {
            result.comebine(self.execute(else_branch_expr));
        }
        Some(result)
    }

    fn visit_get(&mut self, expr: &GetExpr) -> Option<VariableSet> {
        let mut names = Vec::new();
        self.visit_get_inner(&expr.object, &mut names);
        names.push(expr.name.lexeme.clone());
        let id = names.join(".");
        Some(VariableSet::from_depends(&[&id]))
    }

    fn visit_set(&mut self, expr: &SetExpr) -> Option<VariableSet> {
        let mut names = Vec::new();
        self.visit_get_inner(&expr.object, &mut names);
        names.push(expr.name.lexeme.clone());
        let id = names.join(".");
        let gets = Some(VariableSet::from_depends(&[&id]));

        let mut result = VariableSet::default();
        if let Some(gets) = gets {
            result.set_assigns(gets.get_depends().clone());
        }
        let rhs = self.execute(&expr.value);
        result.comebine(rhs);
        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_simple() {
        let mut var_query = VarsQuery::new();
        let result = var_query
            .execute_src("x = y = a + b*(2 + (z = h * i)) - abs(sum(c, d - e/f**g))".to_string());
        assert_eq!(result.unwrap().to_string(), "x,y,z = a,b,c,d,e,f,g,h,i");
    }

    #[test]
    fn if_else_test() {
        let mut var_query = VarsQuery::new();
        let result = var_query.execute_src(
            "if(a == b + c, if (a > d, x = y = m + n, p = q = u + v), z = w * 2)".to_string(),
        );
        assert_eq!(result.unwrap().to_string(), "p,q,x,y,z = a,b,c,d,m,n,u,v,w");
    }

    #[test]
    fn obj_test() {
        let mut var_query = VarsQuery::new();
        let result = var_query.execute_src("A.x = A.y = B.a + B.b*(2 + (A.z = C.D.h * C.D.i)) - abs(sum(B.c, B.d - C.D.e/C.D.f**C.D.g))".to_string());
        assert_eq!(
            result.unwrap().to_string(),
            "A.x,A.y,A.z = B.a,B.b,B.c,B.d,C.D.e,C.D.f,C.D.g,C.D.h,C.D.i"
        );
    }

    #[test]
    fn batch_test() {
        println!("批量查询变量测试：");
        let cnt = 10_000;
        println!("公式总数：{}", cnt);
        let fml = "A! = 1 + 2 * 3 - 6 - 1 + B! + C! * (D! - E! + 10 ** 2 / 5 - (12 + 8)) - F! * G! +  100 / 5 ** 2 ** 1";
        let mut lines = Vec::with_capacity(cnt);

        for i in 1..=cnt {
            let expr = fml.replace("!", &i.to_string());
            lines.push(expr);
        }

        let start = Instant::now();
        let mut var_query = VarsQuery::new();
        let mut result: Option<VariableSet> = None;
        for expr in lines {
            result = var_query.execute_src(expr);
        }
        println!("{:?}", result);
        println!("time: {:?}", start.elapsed());
        println!("==========");
    }
}
