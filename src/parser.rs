use chumsky::prelude::*;

use crate::lexer::Token;

pub enum LiteralValue {
    Number(f32),
    String(String),
    Boolean(bool),
    Array(Vec<Expr>)
}

pub enum Expr {
    Literal(LiteralValue),
    Function {
        body: Box<Expr>
    }
}

pub fn expr() -> impl Parser<Token, Expr, Error = Simple<Token>>
{
    let boolean = just(Token::True).or(just(Token::False)).map(|boolean| match boolean
        {
        Token::True => LiteralValue::Boolean(true),
        Token::False => LiteralValue::Boolean(false),
        _ => unreachable!()
    });
    let literal = boolean.map(Expr::Literal);

    literal

}