use crate::ir::{ExecuteContext, ExprInfo};
use crate::tracer::Tracer;

pub struct LoxContext {
    tracer: Tracer,
    exec_context: ExecuteContext,
}

impl LoxContext {
    pub fn new() -> Self {
        let mut ctx = LoxContext {
            tracer: Tracer::new(),
            exec_context: ExecuteContext::new_dummy(), // 先用 dummy，后面再初始化
        };
        ctx.exec_context = ExecuteContext::new(ctx.clone());
        ctx
    }

    pub fn get_tracer(&self) -> &Tracer {
        &self.tracer
    }

    pub fn get_exec_context(&self) -> &ExecuteContext {
        &self.exec_context
    }

    pub fn prepare_execute(&mut self, expr_infos: Vec<ExprInfo>) {
        self.exec_context.pre_execute(expr_infos);
    }
}

// 需要为 LoxContext 实现 Clone trait
impl Clone for LoxContext {
    fn clone(&self) -> Self {
        LoxContext {
            tracer: self.tracer.clone(),
            exec_context: self.exec_context.clone(),
        }
    }
}
