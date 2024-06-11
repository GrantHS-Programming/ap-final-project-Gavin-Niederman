use std::collections::HashMap;

use crate::parser::{Expr, LiteralValue};

struct Context {
    pub idents: HashMap<String, Expr>
}

pub fn interpret(expr: Expr) -> Result<LiteralValue, String> {
    if let Expr::Literal(LiteralValue::Function { body }) = expr {
        let ctx = Context {
            idents: HashMap::new()
        };
        interpret_expr(*body, ctx)
    } else {
        Err("File was not parsed as a function".into())
    }
}

fn interpret_expr(expr: Expr, mut ctx: Context) -> Result<LiteralValue, String> {
    match expr {
        Expr::Literal(literal) => Ok(literal),
        Expr::Call { fun } =>  {
            match *fun {
                Expr::Literal(LiteralValue::Function { body }) => {
                    interpret_expr(*body, ctx)
                }
                Expr::Ident(ident) => {
                    println!("ident_call");
                    let LiteralValue::Function { body} = interpret_expr(Expr::Ident(ident), ctx)? else {
                        return Err("Cannot call a non function".into())
                    };
                        interpret_expr(*body, ctx)
                    
                }
                _ => Err("Cannot call a non function".into())
            }
        },
        Expr::Ident(ref ident) => {
            if let Some(val) = ctx.idents.get(ident) {
                interpret_expr(val.clone(), ctx)
            } else {
                Err("Unknown Ident".into())
            }
        },
        Expr::Let { ident, value, body } => {
            let Expr::Ident(ident) = *ident else {
                return Err("Expected Ident".into());
            };
            ctx.idents.insert(ident, *value);
            interpret_expr(*body, ctx)
        },
    }
}