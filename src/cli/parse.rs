use ariadne::{Label, Report, ReportKind, Source};
use chumsky::{input::Input, Parser};

use crate::{
    cli::lex::lex,
    parser::file::{file_parser, File},
};

pub fn parse(filename: &str, text: &str) -> File {
    let tokens = lex(filename, text);
    let (output, errs) = file_parser()
        .parse(tokens.spanned((0..0).into()))
        .into_output_errors();
    let error = ariadne::Color::Fixed(1);
    for err in errs {
        let span = err.span();
        Report::build(ReportKind::Error, filename, span.start)
            .with_code(2)
            .with_message("Parse error")
            .with_label(
                Label::new((filename, (*span).into()))
                    .with_message(err.reason().to_string())
                    .with_color(error),
            )
            .finish()
            .print((filename, Source::from(&text)))
            .unwrap();
    }
    match output {
        Some(file) => file,
        None => panic!("Failed to parse file"),
    }
}
