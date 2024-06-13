use chumsky::prelude::*;

use crate::lexer::Token;

#[derive(Debug, Clone)]
pub enum LiteralValue {
    Number(u64),
    String(String),
    Boolean(bool),
    Array(Vec<Expr>),
    Function { body: Box<Expr> },
}

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(LiteralValue),
    Call { fun: Box<Expr> },
    Ident(String),
    Let {
        ident: Box<Expr>,
        value: Box<Expr>,
        body: Box<Expr>
    },
    Grouping(Box<Expr>),
    Addition {
        lhs: Box<Expr>,
        rhs: Box<Expr>
    }
}

pub fn is_ident_reserved(ident: impl AsRef<str>) -> bool {
    match ident.as_ref() {
        "add" => true,
        _ => false,
    }
}

fn literal() -> impl Parser<Token, LiteralValue, Error = Simple<Token>> + Clone {
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
                .rev()
                .enumerate()
                .fold(0, |acc, (i, cur)| acc + cur * 10u64.pow(i as _)),
        )
    });
    choice((boolean, number))
}

fn ident(allow_reserved: bool) -> impl Parser<Token, Expr, Error = Simple<Token>> + Clone {
      filter(|t| match t {
        Token::A => true,
        Token::B => true,
        Token::C => true,
        Token::D => true,
        Token::E => true,
        Token::F => true,
        Token::G => true,
        Token::H => true,
        Token::I => true,
        Token::J => true,
        Token::K => true,
        Token::L => true,
        Token::M => true,
        Token::N => true,
        Token::O => true,
        Token::P => true,
        Token::Q => true,
        Token::R => true,
        Token::S => true,
        Token::T => true,
        Token::U => true,
        Token::V => true,
        Token::W => true,
        Token::X => true,
        Token::Y => true,
        Token::Z => true,
        _ => false,
    })
    .map(|t| t.to_string())
    .repeated()
    .at_least(1)
    .map(|chars| chars.into_iter().collect())
    .validate(move |string: String, span, emit| {if !allow_reserved && is_ident_reserved(&string) {
            emit(Simple::custom(span, format!("'{string}' is a reserved keyword")));   
        }
        Expr::Ident(string)
    })
}

pub fn expr() -> impl Parser<Token, Expr, Error = Simple<Token>> {
    recursive(|expr| {
        let grouping = just(Token::LeftParen).ignore_then(expr.clone()).then_ignore(just(Token::RightParen)).map(|e| Expr::Grouping(Box::new(e)));
        let literal = literal().map(Expr::Literal).or(grouping);
        let p_ident = ident(true).or(literal);

        let call = just(Token::Colon)
            .ignore_then(expr.clone())
            .then_ignore(just(Token::LeftBrace))
            .then_ignore(just(Token::RightBrace))
            .map(|expr| Expr::Call {
                fun: Box::new(expr),
            })
            .or(p_ident);

        let function = just(Token::LeftBrace)
            .ignored()
            .then_ignore(just(Token::RightBrace))
            .then_ignore(just(Token::Arrow))
            .then(expr.clone())
            .map(|(_, expr)| Expr::Literal(LiteralValue::Function {
                body: Box::new(expr),
            }))
            .or(call);

        let let_ = just(Token::Let)
            .ignore_then(ident(false))
            .then_ignore(just(Token::Equals))
            .then(expr.clone())
            .then_ignore(just(Token::In))
            .then(expr)
            .map(|((ident, value), body)| Expr::Let { ident: Box::new(ident), value: Box::new(value), body: Box::new(body) })
            .or(function);
        

        let addition = let_.clone().then_ignore(just(Token::Plus)).then(let_.clone()).map(|(lhs, rhs)| Expr::Addition { lhs: Box::new(lhs), rhs: Box::new(rhs) }).or(let_);

        
        addition
    })
}
