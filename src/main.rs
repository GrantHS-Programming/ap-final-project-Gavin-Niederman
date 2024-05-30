use std::error::Error;

use ariadne::{sources, ColorGenerator, Label, Report, Span};
use chumsky::{primitive::end, Parser};
use lexer::Token;
use parser::expr;

mod lexer;
mod parser;

#[derive(clap::Parser)]
enum Cli {
    Tokenize { source: String },
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = <Cli as clap::Parser>::parse();

    match args {
        Cli::Tokenize { source } => {
            let source_text = std::fs::read_to_string(source.clone())?;

            let source = Box::leak(Box::new(source));
            let tokens = match lexer::token()
                .repeated()
                .then_ignore(end())
                .parse(source_text.clone())
            {
                Ok(tokens) => tokens,
                Err(errors) => {
                    for error in errors {
                        let mut colors = ColorGenerator::new();

                        let mut report = Report::build(
                            ariadne::ReportKind::Error,
                            source.as_str(),
                            error.span().start(),
                        )
                        .with_code("E0001")
                        .with_message("Failed to lex.");
                        match error.reason() {
                            chumsky::error::SimpleReason::Unexpected => {
                                let expected_len = error.expected().len();
                                let expected_label = format!(
                                    "Expected one of: {}. Found: {}.",
                                    error
                                        .expected()
                                        .enumerate()
                                        .map(|(i, c)| {
                                            if let Some(c) = c {
                                                let mut entry = match c {
                                                    '\n' => String::from("'\\n'"),
                                                    c => format!("'{c}'"),
                                                };
                                                if i < expected_len - 1 {
                                                    entry.push_str(", ")
                                                }
                                                entry
                                            } else {
                                                String::new()
                                            }
                                        })
                                        .collect::<String>(),
                                    error
                                        .found()
                                        .map(|c| format!("'{c}'"))
                                        .unwrap_or_else(|| String::from("this"))
                                );

                                report = report.with_label(
                                    Label::new((source.as_str(), error.span()))
                                        .with_message(expected_label)
                                        .with_color(colors.next()),
                                );
                            }
                            chumsky::error::SimpleReason::Unclosed {
                                span: _,
                                delimiter: _,
                            } => unreachable!("Delimiters are not lexed"),
                            chumsky::error::SimpleReason::Custom(_) => {
                                unreachable!("Lexer doesn't produce custom errors")
                            }
                        }

                        report
                            .finish()
                            .eprint(sources(vec![(source.as_str(), source_text.as_str())]))?;
                    }
                    return Err("Failed to lex".into());
                }
            };
            let expr = match expr().parse(tokens) {
                Ok(tokens) => tokens,
                Err(errors) => {
                    for error in errors {
                        let mut colors = ColorGenerator::new();

                        let mut report = Report::build(
                            ariadne::ReportKind::Error,
                            source.as_str(),
                            error.span().start(),
                        )
                        .with_code("E0002")
                        .with_message("Failed to parse.");

                        match error.reason() {
                            chumsky::error::SimpleReason::Unexpected => {
                                let expected_len = error.expected().len();
                                let expected_label = format!(
                                    "Expected one of: {}. Found: {}.",
                                    error
                                        .expected()
                                        .enumerate()
                                        .map(|(i, c)| {
                                            if let Some(c) = c {
                                                let mut entry = match c {
                                                    Token::Newline => String::from("'\\n'"),
                                                    c => format!("'{c}'"),
                                                };
                                                if i < expected_len - 1 {
                                                    entry.push_str(", ")
                                                }
                                                entry
                                            } else {
                                                String::new()
                                            }
                                        })
                                        .collect::<String>(),
                                    error
                                        .found()
                                        .map(|c| format!("'{c}'"))
                                        .unwrap_or_else(|| String::from("this"))
                                );

                                report = report.with_label(
                                    Label::new((source.as_str(), error.span()))
                                        .with_message(expected_label)
                                        .with_color(colors.next()),
                                );
                            }
                            chumsky::error::SimpleReason::Unclosed { span, delimiter } => todo!(),
                            chumsky::error::SimpleReason::Custom(_) => todo!(),
                        }
                        report
                            .finish()
                            .eprint(sources(vec![(source.as_str(), source_text.as_str())]))?;
                    }
                    return Err("Failed to parse".into());
                }
            };

            println!("{:?}", expr)
        }
    }

    Ok(())
}
