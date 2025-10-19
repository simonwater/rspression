use crate::expr::*;
use crate::parser::Parser;
use crate::visitors::VariableSet;

pub struct VarsQuery {
    vars: VariableSet,
}

impl VarsQuery {
    pub fn new() -> Self {
        VarsQuery {
            vars: VariableSet::default(),
        }
    }

    pub fn execute_src(&mut self, expression: String) -> Option<&VariableSet> {
        self.vars = VariableSet::default();
        let expr = Parser::new(&expression).parse().ok()?;
        self.execute(&expr)
    }

    pub fn reset(&mut self) {
        self.vars = VariableSet::default();
    }

    pub fn execute(&mut self, expr: &Expr) -> Option<&VariableSet> {
        expr.accept(self);
        Some(&self.vars)
    }

    fn visit_object(&self, expr: &Expr, names: &mut Vec<String>) {
        match expr {
            Expr::Id(id_expr) => {
                names.push(id_expr.name.lexeme.to_string());
            }
            Expr::Get(get_expr) => {
                self.visit_object(&get_expr.object, names);
                names.push(get_expr.name.lexeme.to_string());
            }
            _ => {}
        }
    }
}

impl Visitor<()> for VarsQuery {
    fn visit_binary(&mut self, expr: &BinaryExpr) {
        self.execute(&expr.left);
        self.execute(&expr.right);
    }

    fn visit_logic(&mut self, expr: &LogicExpr) {
        self.execute(&expr.left);
        self.execute(&expr.right);
    }

    fn visit_literal(&mut self, _expr: &LiteralExpr) {}

    fn visit_unary(&mut self, expr: &UnaryExpr) {
        self.execute(&expr.right);
    }

    fn visit_id(&mut self, expr: &IdExpr) {
        self.vars.add_depend(expr.name.lexeme.to_string());
    }

    fn visit_assign(&mut self, expr: &AssignExpr) {
        let AssignExpr { left, .. } = expr;
        if let Expr::Id(id_expr) = &**left {
            self.vars.add_assign(id_expr.name.lexeme.to_string());
            self.execute(&expr.right);
        }
    }

    fn visit_call(&mut self, expr: &CallExpr) {
        for arg in &expr.arguments {
            self.execute(arg);
        }
    }

    fn visit_if(&mut self, expr: &IfExpr) {
        let IfExpr {
            condition,
            then_branch,
            else_branch,
        } = expr;
        self.execute(condition);
        self.execute(then_branch);
        if let Some(else_branch_expr) = else_branch {
            self.execute(else_branch_expr);
        }
    }

    fn visit_get(&mut self, expr: &GetExpr) {
        let mut names: Vec<String> = Vec::new();
        self.visit_object(&expr.object, &mut names);
        names.push(expr.name.lexeme.to_string());
        let id = names.join(".");
        self.vars.add_depend(id);
    }

    fn visit_set(&mut self, expr: &SetExpr) {
        let mut names: Vec<String> = Vec::new();
        self.visit_object(&expr.object, &mut names);
        names.push(expr.name.lexeme.to_string());
        let id = names.join(".");
        self.vars.add_assign(id);
        self.execute(&expr.value);
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

        println!("开始解析公式...");
        let start = Instant::now();
        let mut exprs = Vec::with_capacity(cnt);
        for line in &lines {
            let expr = Parser::new(line).parse().unwrap();
            exprs.push(expr);
        }
        println!("公式解析时间: {:?}", start.elapsed());

        println!("开始查询变量...");
        let start = Instant::now();
        let mut result: Option<&VariableSet> = None;
        let mut var_query = VarsQuery::new();
        for expr in &exprs {
            var_query.reset();
            result = var_query.execute(expr);
        }
        println!("{:?}", result);
        println!("变量提取用时: {:?}", start.elapsed());
        println!("==========");
    }
}
