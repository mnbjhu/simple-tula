use chumsky::{
    error::Rich,
    extra,
    primitive::{choice, just, none_of, one_of},
    text::{self},
    IterParser, Parser,
};

use crate::lexer::{
    token::{Keyword, Token},
    util::Spanned,
};

pub fn lexer<'src>(
) -> impl Parser<'src, &'src str, Vec<Spanned<Token>>, extra::Err<Rich<'src, char>>> {
    let ident = text::ident().map(|ident| match ident {
        "start" => Token::Keyword(Keyword::Start),
        "default" => Token::Keyword(Keyword::Default),
        ident => Token::Ident(ident.to_string()),
    });

    let comment = just("//")
        .ignored()
        .then(none_of("\n").repeated().to_slice());

    let new_line = comment
        .or_not()
        .then(just('\n'))
        .then(one_of(" \t").repeated())
        .repeated()
        .at_least(1)
        .map(|_| Token::Newline);

    let int = text::digits(10).to_slice();

    let int = int.map(|s: &str| Token::Ident(s.to_string()));

    let specific_op = choice((
        just("->").map(|_| Token::Op("->".to_string())),
        just("<-").map(|_| Token::Op("<-".to_string())),
    ));

    let op_chars = one_of("+-*/%&|!=");
    let op = op_chars
        .repeated()
        .at_least(1)
        .to_slice()
        .map(|s: &str| Token::Ident(s.to_string()));

    choice((new_line, specific_op, op, ident, int))
        .map_with(|i, s| (i, s.span()))
        .padded_by(one_of(" \t").labelled("whitespace").repeated())
        .repeated()
        .collect::<Vec<_>>()
        .padded()
}
