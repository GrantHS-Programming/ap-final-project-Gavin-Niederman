use std::fmt::Display;

use chumsky::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token {
    Equals,

    Plus,
    Minus,
    Star,
    Slash,

    Colon,
    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,

    Comma,

    Arrow,

    Space,
    Newline,
}
impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Token::Equals => "=",
            Token::Plus => "+",
            Token::Minus => "-",
            Token::Star => "*",
            Token::Slash => "/",
            Token::Colon => ":",
            Token::LeftBrace => "{",
            Token::RightBrace => "}",
            Token::LeftParen => "(",
            Token::RightParen => ")",
            Token::Comma => ",",
            Token::Arrow => "->",
            Token::Space => " ",
            Token::Newline => "\n",
        };

        write!(f, "{}", string)
    }
}

pub fn token() -> impl Parser<char, Token, Error = Simple<char>> {
    let equals = just("=").to(Token::Equals);
    let plus = just("+").to(Token::Plus);
    let minus = just("-").to(Token::Minus);
    let star = just("*").to(Token::Star);
    let slash = just("/").to(Token::Slash);

    let colon = just(":").to(Token::Colon);
    let left_brace = just("{").to(Token::LeftBrace);
    let right_brace = just("}").to(Token::RightBrace);
    let left_paren = just("(").to(Token::LeftParen);
    let right_paren = just(")").to(Token::RightParen);

    let comma = just(",").to(Token::Comma);

    let arrow = just("->").to(Token::Arrow);

    let space = just(" ").to(Token::Space);
    let newline = just("\n").to(Token::Newline);

    choice((
        // Arrow must take priority over minus
        arrow,

        equals,
        plus,
        minus,
        star,
        slash,
        colon,
        left_brace,
        right_brace,
        left_paren,
        right_paren,
        comma,
        space,
        newline,
    ))
}
