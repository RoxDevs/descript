#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Str(String),
    Int(i32),
    Float(f64),
    NestedStmt(Box<Stmt>)
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

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Kw {
    Echo
}

#[derive(Debug, PartialEq, Clone)]
pub struct Stmt {
    pub kw: Kw,
    pub args: Vec<Literal>
}