#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Str(String),
    Int(i32),
    Float(f64),
}

impl From<String> for Literal {
    fn from(value: String) -> Self {
        Literal::Str(value)
    }
}

impl From<f64> for Literal {
    fn from(value: f64) -> Self {
        Literal::Float(value)
    }
}

impl ToString for Literal {
    fn to_string(&self) -> String {
        match self {
            Literal::Str(s) => s.clone(),
            Literal::Int(i) => i.to_string(),
            Literal::Float(f) => f.to_string()
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Kw {
    Echo
}

#[derive(Debug, PartialEq, Clone)]
pub struct Stmt {
    pub kw: Kw,
    pub args: Vec<Literal>
}