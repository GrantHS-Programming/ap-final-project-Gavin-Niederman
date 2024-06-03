use std::fmt::Display;

use chumsky::prelude::*;
use text::digits;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

    Newline,

    True,
    False,

    Quote,

    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Zero,
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
            Token::Newline => "\\n",
            Token::False => "false",
            Token::True => "true",
            Token::Quote => "\"",
            Token::One => "1",
            Token::Two => "2",
            Token::Three => "3",
            Token::Four => "4",
            Token::Five => "5",
            Token::Six => "6",
            Token::Seven => "7",
            Token::Eight => "8",
            Token::Nine => "9",
            Token::Zero => "0",
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

    let newline = just("\n").to(Token::Newline);

    let true_ = just("true").to(Token::True);
    let false_ = just("false").to(Token::False);

    let quote = just("\"").to(Token::Quote);

    let number = filter(|c: &char| c.is_numeric()).map(|c: char| match c {
        '1' => Token::One,
        '2' => Token::Two,
        '3' => Token::Three,
        '4' => Token::Four,
        '5' => Token::Five,
        '6' => Token::Six,
        '7' => Token::Seven,
        '8' => Token::Eight,
        '9' => Token::Nine,
        '0' => Token::Zero,
        _ => unreachable!(),
    });

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
        newline,
        true_,
        false_,
        quote,
        number,
    )).padded_by(just(" ").repeated())
}
