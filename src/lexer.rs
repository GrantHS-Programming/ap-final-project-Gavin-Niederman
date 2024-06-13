use std::fmt::Display;

use chumsky::prelude::*;

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
    Let,
    In,

    Newline,
    Space,

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

    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
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
            Token::A => "a",
            Token::B => "b",
            Token::C => "c",
            Token::D => "d",
            Token::E => "e",
            Token::F => "f",
            Token::G => "g",
            Token::H => "h",
            Token::I => "i",
            Token::J => "j",
            Token::K => "k",
            Token::L => "l",
            Token::M => "m",
            Token::N => "n",
            Token::O => "o",
            Token::P => "p",
            Token::Q => "q",
            Token::R => "r",
            Token::S => "s",
            Token::T => "t",
            Token::U => "u",
            Token::V => "v",
            Token::W => "w",
            Token::X => "x",
            Token::Y => "y",
            Token::Z => "z",
            Token::Let => "let",
            Token::In => "in",
            Token::Space => " ",
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
    let space = just(" ").to(Token::Space);

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

    let letter = filter(|c: &char| c.is_alphabetic()).map(|c: char| match c {
        'a' => Token::A,
        'b' => Token::B,
        'c' => Token::C,
        'd' => Token::D,
        'e' => Token::E,
        'f' => Token::F,
        'g' => Token::G,
        'h' => Token::H,
        'i' => Token::I,
        'j' => Token::J,
        'k' => Token::K,
        'l' => Token::L,
        'm' => Token::M,
        'n' => Token::N,
        'o' => Token::O,
        'p' => Token::P,
        'q' => Token::Q,
        'r' => Token::R,
        's' => Token::S,
        't' => Token::T,
        'u' => Token::U,
        'v' => Token::V,
        'w' => Token::W,
        'x' => Token::X,
        'y' => Token::Y,
        'z' => Token::Z,
        _ => unreachable!(),
    });

    let let_ = just("let").to(Token::Let);
    let in_ = just("in").to(Token::In);

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
        space,
        true_,
        false_,
        quote,
        let_,
        in_,
        number,
        letter,
    ))
    .padded_by(just(" ").repeated())
}
