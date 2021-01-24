#[derive(Debug, PartialEq)]
pub struct Program {
    pub lines: Vec<Line>,
}

#[derive(Debug, PartialEq)]
pub struct Line {
    pub indentation: usize,
    pub code: Code
}

#[derive(Debug, PartialEq)]
pub enum Code {
    Empty,
    Special(Special),
    Raw(String),
}

#[derive(Debug, PartialEq)]
pub enum Special {
    Env(String),
    Input(String),
    Control(Control),
    Escaped(String),
    Main,
}

#[derive(Debug, PartialEq)]
pub enum Control {
    If(String),
    Range(String, String, String),
    For(String, String, String),
    While(String),
    Loop,
}
