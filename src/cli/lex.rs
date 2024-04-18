use ariadne::{Label, Report, ReportKind, Source};
use chumsky::Parser;

use crate::lexer::{lex::lexer, token::Token, util::Spanned};

pub fn lex(filename: &str, text: &str) -> Vec<Spanned<Token>> {
    let error = ariadne::Color::Fixed(1);
    let (tokens, errs) = lexer().parse(text).into_output_errors();
    for err in errs {
        let span = err.span();
        Report::build(ReportKind::Error, filename, span.start)
            .with_code(1)
            .with_message("Lexical error")
            .with_label(
                Label::new((filename, (*span).into()))
                    .with_message(err.reason().to_string())
                    .with_color(error),
            )
            .finish()
            .print((filename, Source::from(&text)))
            .unwrap();
    }
    tokens.unwrap_or_default()
}
