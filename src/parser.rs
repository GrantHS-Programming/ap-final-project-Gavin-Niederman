use chumsky::prelude::*;

use crate::lexer::Token;

#[derive(Debug)]
pub enum LiteralValue {
    Number(u64),
    String(String),
    Boolean(bool),
    Array(Vec<Expr>),
}

#[derive(Debug)]
pub enum Expr {
    Literal(LiteralValue),
    Function { body: Box<Expr> },
}

fn literal() -> impl Parser<Token, LiteralValue, Error = Simple<Token>> {
    let boolean = just(Token::True)
        .or(just(Token::False))
        .map(|boolean| match boolean {
            Token::True => LiteralValue::Boolean(true),
            Token::False => LiteralValue::Boolean(false),
            _ => unreachable!(),
        });
    let number = (choice((
        just(Token::One),
        just(Token::Two),
        just(Token::Three),
        just(Token::Four),
        just(Token::Five),
        just(Token::Six),
        just(Token::Seven),
        just(Token::Eight),
        just(Token::Nine),
        just(Token::Zero),
    ))
    .map(|token| match token {
        Token::One => 1,
        Token::Two => 2,
        Token::Three => 3,
        Token::Four => 4,
        Token::Five => 5,
        Token::Six => 6,
        Token::Seven => 7,
        Token::Eight => 8,
        Token::Nine => 9,
        Token::Zero => 0,
        _ => unreachable!(),
    }))
    .repeated()
    .at_least(1)
    .map(|nums| {
        LiteralValue::Number(
            nums.into_iter()
                .enumerate()
                .fold(0, |acc, (i, cur)| if cur != 0 { acc + cur * 10u64.pow(i as _) } else {acc * 10} )
        )
    });
    choice((boolean, number))
}

pub fn expr() -> impl Parser<Token, Expr, Error = Simple<Token>> {
    recursive(|expr| {
        let literal = literal().map(Expr::Literal);

        let function = just(Token::LeftBrace)
            .ignored()
            .then_ignore(just(Token::RightBrace))
            .then_ignore(just(Token::Arrow))
            .then(expr)
            .map(|(_, expr)| Expr::Function {
                body: Box::new(expr),
            });

        function.or(literal)
    })
}
