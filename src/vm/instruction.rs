use crate::regex::Regex;

#[derive(Debug, PartialEq,Clone)]
pub enum Component {
    Reader(Regex),
    Wrapper(Vec<Box<Component>>),
    WrapperSerial(Vec<Box<Component>>),
    IFWrapper(Box<Component>, Vec<Box<Component>>),
    Group(Vec<Box<Component>>),
    GroupSerial(Vec<Box<Component>>),
}

#[derive(Debug, PartialEq,Clone)]
pub enum MicroInstruction {
    FLAG,
    UNFLAG,
    MATCH(Regex),
    JMP(usize),
    JMPF(usize),
    SPLIT(usize,usize),
}
