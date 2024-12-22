use crate::regex::Regex;

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Reader(Regex),
    Concat(Vec<Statement>),
    Alternation(Box<Statement>, Box<Statement>),
    ZeroOrOne(Box<Statement>),
    ZeroOrMore(Box<Statement>),
    OneOrMore(Box<Statement>),
    Condition {
        cond: Box<Statement>,
        inner: Box<Statement>,
    },
    Save(String, Box<Statement>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
    Match(Regex),
    Split(usize, usize),
    Jmp(usize),
    JumpF(usize),
    Save(String),
    StartCapture(String),
    EndCapture(String),
    Flag,
    UnFlag,
    Carry,
    Push(usize),
    Pop,
}

#[derive(Debug, PartialEq, Clone)]
pub enum State {
    StartCapture(String, usize),
    Text(String, usize, usize),
    List(String, Vec<State>),
}

#[derive(Debug, Clone)]
pub struct Token {
    pub name: String,
    pub text: String,
    pub start: usize,
    pub end: usize,
    pub children: Vec<Token>,
}

impl State {
    pub fn into_token(self) -> Option<Token> {
        match self {
            State::Text(text, start, end) => Some(Token {
                name: "".to_string(),
                text,
                start,
                end,
                children: vec![],
            }),
            State::StartCapture(_, _) => None,
            State::List(name, list) => {
                let children: Vec<Token> = list
                    .into_iter()
                    .filter_map(|state| state.into_token())
                    .collect();

                if children.is_empty() {
                    return None;
                }
                let start = children.iter().map(|t| t.start).min().unwrap_or(0);
                let end = children.iter().map(|t| t.end).max().unwrap_or(0);

                let text = children.iter().map(|t| t.text.as_str()).collect::<String>();

                Some(Token {
                    name: name.to_string(),
                    text,
                    start,
                    end,
                    children,
                })
            }
        }
    }
}
