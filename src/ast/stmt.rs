#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KeyWord {
    Echo,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Arg {
    Num(i32),
    Str(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Stmt {
    pub kw: KeyWord,
    pub args: Vec<Arg>,
}

#[cfg(test)]
mod tests {
    use crate::{grammar::StmtParser, lexer::Lexer};

    use super::*;

    #[test]
    fn stmt_tests() {
        let tests = vec![(
            r#"echo "Hello world!""#,
            Stmt {
                kw: KeyWord::Echo,
                args: vec![Arg::Str("Hello world!".to_string())],
            },
        )];
        for (input, expected) in tests {
            let parser = StmtParser::new();
            let tokens = Lexer::new(input);
            assert_eq!(
                parser.parse(tokens).unwrap(),
                expected,
                "Panic on: {}",
                input
            )
        }
    }
}
