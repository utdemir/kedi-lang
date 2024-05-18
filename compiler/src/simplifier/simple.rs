use crate::renamer::RenamedIdentifier;

pub struct Module {
    pub functions: Vec<Function>,
}

pub struct Label {
    pub id: i32,
}

pub struct Function {
    pub name: RenamedIdentifier,
    pub arity: usize,
    pub body: Vec<Expr>,
}

pub struct Assignment {
    pub target: RenamedIdentifier,
    pub value: AssignmentValue,
}

pub enum AssignmentValue {
    Call(RenamedIdentifier, RenamedIdentifier, Vec<RenamedIdentifier>),
    Literal(RenamedIdentifier, i32),
}

pub enum Expr {
    Loop(Label),
    Assignment(Assignment),
    Branch(Label),
}
