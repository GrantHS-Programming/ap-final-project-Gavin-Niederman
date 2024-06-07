use crate::parser::{Expr, LiteralValue};

pub fn interpret(expr: Expr) -> Result<LiteralValue, String> {
    if let Expr::Function { body } = expr {
        Ok(interpret_expr(*body))
    } else {
        Err("File was not parsed as a function".into())
    }
}

fn interpret_expr(expr: Expr) -> LiteralValue {
    match expr {
        Expr::Literal(literal) => literal,
        Expr::Function { body } => interpret_expr(*body),
        Expr::Call { ident: _ident } => todo!(),
    }
}