use crate::ast::{Kw, Literal, Stmt};
use chumsky::{
    error::Cheap,
    extra,
    prelude::Parser,
    primitive::{any, choice, just, none_of},
    text, IterParser, ParseResult,
};

trait CheapParser<'a, O> = Parser<'a, &'a str, O, extra::Err<Cheap>>;

pub fn literal<'a>() -> impl CheapParser<'a, Literal> {
    let int = text::int::<'a, _, _, extra::Err<Cheap>>(10)
        .map(|s: &'a str| Literal::Int(s.parse().unwrap()));
    let str = just("\"")
        .ignore_then(any().and_is(none_of(r#"""#)).repeated().collect::<String>())
        .then_ignore(just("\""))
        .map(|s| Literal::Str(s.to_string()));
    let float = text::int(10)
        .then(just('.'))
        .then(text::digits(10))
        .slice()
        .from_str()
        .unwrapped()
        .map(Literal::Float);
    choice((float, int, str))
}

fn kw<'a>() -> impl CheapParser<'a, Kw> {
    choice((text::keyword::<'a, &'a str, _, _, extra::Err<Cheap>>("echo").to(Kw::Echo),))
}

fn padded_ws<'a>() -> impl CheapParser<'a, ()> {
    text::whitespace().at_least(1).ignored()
}

pub fn stmt<'a>() -> impl CheapParser<'a, Stmt> {
    kw().then_ignore(padded_ws())
        .then(
            literal()
                .separated_by(padded_ws().ignored())
                .collect::<Vec<Literal>>(),
        )
        .map(|(kw, args)| Stmt { kw: kw, args: args })
}

#[cfg(test)]
mod test {
    use chumsky::Parser;

    use super::*;

    #[test]
    fn int_test() {
        let tests = vec![
            ("2", Literal::Int(2)),
            ("123", Literal::Int(123)),
            ("1.0", Literal::Float(1.0)),
            ("2.31", Literal::Float(2.31)),
        ];
        for (i, o) in tests {
            assert_eq!(literal().parse(i).unwrap(), o)
        }
    }

    #[test]
    fn str_test() {
        let tests = vec![(r#""Hello World""#, Literal::Str("Hello World".to_string()))];
        for (i, o) in tests {
            assert_eq!(literal().parse(i).unwrap(), o)
        }
    }

    #[test]
    fn kw_test() {
        let tests = vec![("echo", Kw::Echo)];
        for (i, o) in tests {
            assert_eq!(kw().parse(i).unwrap(), o)
        }
    }

    #[test]
    fn ws_test() {
        assert_eq!(padded_ws().parse("  ").unwrap(), ())
    }

    #[test]
    fn stmt_test() {
        let tests = vec![(
            "echo 2",
            Stmt {
                kw: Kw::Echo,
                args: vec![Literal::Int(2)],
            },
        )];
        for (i, o) in tests {
            assert_eq!(stmt().parse(i).unwrap(), o)
        }
    }
}
