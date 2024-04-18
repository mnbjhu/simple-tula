use crate::lexer::token::Token;
use chumsky::{error::Rich, extra, span::SimpleSpan};

pub type Span = SimpleSpan<usize>;

pub type Spanned<T> = (T, Span);

pub type ParserInput<'tokens> = chumsky::input::SpannedInput<Token, Span, &'tokens [(Token, Span)]>;
pub type Extra<'src> = extra::Full<Rich<'src, Token>, (), ()>;
