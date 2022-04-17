pub use crate::Location;

pub type Id = String;

#[derive(Debug, PartialEq)]
pub struct Located<T, U = ()> {
    pub location: Location,
    pub custom: U,
    pub node: T,
}

impl<T> Located<T> {
    pub fn new(location: Location, node: T) -> Self {
        Self {
            location,
            custom: (),
            node,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Verb {
    Add,
    Sub,
    Mult,
    Div,
    Mod,
    Pow,
    LShift,
    RShift,
    Or,
    Xor,
    And,
}

#[derive(Debug, PartialEq)]
pub enum Adverb {

}

#[derive(Debug, PartialEq)]
pub enum Cmpop {
    Eq,
    NotEq,
    Lt,
    LtEq,
    Gt,
    GtEq,
}
