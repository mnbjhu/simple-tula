use std::path::PathBuf;

use ariadne::{Label, Report, ReportKind, Source};
use chumsky::{input::Input, Parser};

use crate::{
    cli::{lex::lex, parse::parse},
    parser::tape::tape_parser,
    runtime::state::AppState,
};

pub fn run(src: &PathBuf, tape_file: &PathBuf) {
    let src_text = std::fs::read_to_string(src).unwrap();
    let tape_text = std::fs::read_to_string(tape_file).unwrap();
    let machine = parse(src.to_str().unwrap(), &src_text);
    let tape_filename = tape_file.to_str().unwrap();
    let tape_tokens = lex(tape_filename, &tape_text);
    let error = ariadne::Color::Fixed(1);
    let (tape, errs) = tape_parser()
        .parse(tape_tokens.spanned((0..0).into()))
        .into_output_errors();
    for err in errs {
        let span = err.span();
        Report::build(ReportKind::Error, tape_file.to_str().unwrap(), span.start)
            .with_code(2)
            .with_message("Parse error")
            .with_label(
                Label::new((tape_filename, (*span).into()))
                    .with_message(err.reason().to_string())
                    .with_color(error),
            )
            .finish()
            .print((tape_filename, Source::from(&tape_text)))
            .unwrap();
    }
    let tape = tape.unwrap_or_default();
    let mut state = AppState {
        tape,
        position: 0,
        state: machine.start,
        default: machine.default,
    };

    loop {
        state.print_tape();
        state.act(&machine.machine);
    }
}
