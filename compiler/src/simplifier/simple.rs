use crate::util::loc::{Located, Tagged};
use std::collections::HashMap;

use crate::{
    parser::syntax,
    renamer::plain,
    util::loc,
    util::pp::{SExpr, SExprTerm},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SingleUseIdentifier {
    pub id: u32,
}

impl SExpr for SingleUseIdentifier {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::Atom(format!("${}", self.id))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Identifier {
    Plain(plain::Identifier),
    SingleUse(SingleUseIdentifier),
}

impl SExpr for Identifier {
    fn to_sexpr(&self) -> SExprTerm {
        match self {
            Identifier::Plain(p) => p.to_sexpr(),
            Identifier::SingleUse(u) => u.to_sexpr(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Module {
    pub statements: Vec<Located<TopLevelStmt>>,
}

impl SExpr for Module {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::call(
            "module",
            self.statements.iter().map(|x| x.to_sexpr()).collect(),
        )
    }
}

#[derive(Clone, Debug)]
pub enum TopLevelStmt {
    FunDecl(Located<FunDecl>),
}

impl SExpr for TopLevelStmt {
    fn to_sexpr(&self) -> SExprTerm {
        match self {
            TopLevelStmt::FunDecl(f) => f.to_sexpr(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct FunDecl {
    pub name: Located<syntax::Identifier>,
    pub implementation: Located<FunImpl>,
    pub tag_map: loc::TagMap,
    pub refs: HashMap<plain::GlobalIdentifier, syntax::Identifier>,
}

impl SExpr for FunDecl {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::call(
            "fun_decl",
            vec![
                self.name.to_sexpr(),
                SExprTerm::call(
                    "refs",
                    self.refs
                        .iter()
                        .map(|(k, v)| SExprTerm::call("ref", vec![k.to_sexpr(), v.to_sexpr()]))
                        .collect(),
                ),
                SExprTerm::call("implementation", vec![self.implementation.to_sexpr()]),
            ],
        )
    }
}

#[derive(Clone, Debug)]
pub struct FunImpl {
    pub parameters: Tagged<Vec<Tagged<plain::LocalIdentifier>>>,
    pub body: Tagged<Vec<Tagged<Statement>>>,
}

impl SExpr for FunImpl {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::List(vec![
            SExprTerm::atom("fun_impl"),
            SExprTerm::call(
                "parameters",
                self.parameters.value.iter().map(|x| x.to_sexpr()).collect(),
            ),
            SExprTerm::List(self.body.value.iter().map(|x| x.to_sexpr()).collect()),
        ])
    }
}

#[derive(Clone, Debug)]
pub struct Assignment {
    pub target: Tagged<Identifier>,
    pub value: Tagged<AssignmentValue>,
}

impl SExpr for Assignment {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::call(
            "assignment",
            vec![self.target.to_sexpr(), self.value.to_sexpr()],
        )
    }
}

#[derive(Clone, Debug)]
pub struct Call {
    pub fun_name: Tagged<plain::GlobalIdentifier>,
    pub arguments: Tagged<Vec<Tagged<Identifier>>>,
}

impl SExpr for Call {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::call(
            "call",
            vec![self.fun_name.to_sexpr(), self.arguments.to_sexpr()],
        )
    }
}

#[derive(Clone, Debug)]
pub enum AssignmentValue {
    Call(Call),
    Identifier(Identifier),
    LiteralNumber(i32),
}

impl SExpr for AssignmentValue {
    fn to_sexpr(&self) -> SExprTerm {
        match self {
            AssignmentValue::Call(c) => c.to_sexpr(),
            AssignmentValue::Identifier(v) => v.to_sexpr(),
            AssignmentValue::LiteralNumber(n) => {
                SExprTerm::call("lit_number", vec![SExprTerm::number(*n)])
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Label {
    pub id: u32,
}

impl SExpr for Label {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::call("label", vec![SExprTerm::number(self.id)])
    }
}

#[derive(Clone, Debug)]
pub struct If {
    pub condition: Tagged<Identifier>,
    pub then: Tagged<Vec<Tagged<Statement>>>,
    pub else_: Tagged<Vec<Tagged<Statement>>>,
}

impl SExpr for If {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::call(
            "if",
            vec![
                self.condition.to_sexpr(),
                SExprTerm::call(
                    "then",
                    self.then.value.iter().map(|x| x.to_sexpr()).collect(),
                ),
                SExprTerm::call(
                    "else",
                    self.else_.value.iter().map(|x| x.to_sexpr()).collect(),
                ),
            ],
        )
    }
}

#[derive(Clone, Debug)]
pub struct Loop {
    pub label: Label,
    pub body: Tagged<Vec<Tagged<Statement>>>,
}

impl SExpr for Loop {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::call(
            "loop",
            vec![
                self.label.to_sexpr(),
                SExprTerm::call(
                    "body",
                    self.body.value.iter().map(|x| x.to_sexpr()).collect(),
                ),
            ],
        )
    }
}

#[derive(Debug, Clone)]
pub struct InlineWasm {
    pub input_stack: Tagged<Vec<Tagged<Identifier>>>,
    pub output_stack: Tagged<Vec<Tagged<Identifier>>>,
    pub wasm: Tagged<wast::core::Instruction<'static>>,
}

impl SExpr for InlineWasm {
    fn to_sexpr(&self) -> SExprTerm {
        SExprTerm::List(vec![
            SExprTerm::Atom("inline_wasm".to_string()),
            SExprTerm::List(
                self.input_stack
                    .value
                    .iter()
                    .map(|x| x.to_sexpr())
                    .collect(),
            ),
            SExprTerm::List(
                self.output_stack
                    .value
                    .iter()
                    .map(|x| x.to_sexpr())
                    .collect(),
            ),
            SExprTerm::Atom(format!("{:?}", self.wasm.value)),
        ])
    }
}

#[derive(Clone, Debug)]
pub enum Statement {
    // Loop(Label),
    Loop(Loop),
    Assignment(Assignment),
    Branch(Label),
    Break(Label),
    Return(Identifier),
    If(If),
    InlineWasm(InlineWasm),
    Nop,
}

impl SExpr for Statement {
    fn to_sexpr(&self) -> SExprTerm {
        match self {
            Statement::Loop(l) => l.to_sexpr(),
            Statement::Assignment(a) => a.to_sexpr(),
            Statement::Branch(l) => SExprTerm::call("branch", vec![l.to_sexpr()]),
            Statement::Break(l) => SExprTerm::call("break", vec![l.to_sexpr()]),
            Statement::Return(i) => SExprTerm::call("return", vec![i.to_sexpr()]),
            Statement::If(i) => i.to_sexpr(),
            Statement::Nop => SExprTerm::atom("nop"),
            Statement::InlineWasm(i) => i.to_sexpr(),
        }
    }
}
