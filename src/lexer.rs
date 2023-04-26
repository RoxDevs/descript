use std::fmt;

use logos::{Logos, SpannedIter};

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

#[derive(Debug, Clone, Logos, PartialEq)]
pub enum Token<'source> {
    #[token("+")]
    OpAdd,
    #[token("-")]
    OpSub,
    #[token("*")]
    OpMul,
    #[token("/")]
    OpDiv,
    #[token("(")]
    ParenBegin,
    #[token(")")]
    ParenEnd,
    #[regex(r"[0-9]+", priority = 3, callback = |lex| {lex.slice()})]
    Int(&'source str),
    #[regex(r#"""#)]
    Quote,
    #[regex(r#""[a-z|A-Z|0-9|\s]*""#, callback = |lex| {lex.slice()}, priority = 2)]
    Str(&'source str),
    #[regex(r#"\s+"#)]
    ExpectedWS,
    #[token("echo")]
    KwEcho,
    #[error]
    Error,
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LexicalError {
    InvalidToken,
}

pub struct Lexer<'source> {
    internal: SpannedIter<'source, Token<'source>>,
}

impl<'a> Lexer<'a> {
    pub fn new(str: &'a str) -> Self {
        Self {
            internal: Token::lexer(str).spanned(),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Spanned<Token<'a>, usize, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.internal.next().map(|(token, span)| match token {
            Token::Error => Err(LexicalError::InvalidToken),
            _ => Ok((span.start, token, span.end)),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tok_tests() {
        let tests = vec![(r#"""#, vec![Token::Quote]), ("2", vec![Token::Int("2")])];
        for (input, expected) in tests {
            let tokens = Lexer::new(input);
            assert_eq!(
                tokens.map(|tok| tok.unwrap().1).collect::<Vec<Token>>(),
                expected,
                "Panic on: {}",
                input
            )
        }
    }
}
