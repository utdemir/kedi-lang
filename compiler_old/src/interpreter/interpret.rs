use std::collections::HashMap;

use bimap::BiHashMap;

use crate::{parser::syntax, renamer::plain, simplifier::simple, util::bimap::Bimap};

use super::KediValue;

pub struct InterpretOptions {
    pub fuel_limit: Option<u64>,
}

impl Default for InterpretOptions {
    fn default() -> Self {
        InterpretOptions { fuel_limit: None }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InterpretResult {
    Success(InterpretSuccess),
    OutOfFuel(InterpretOutOfFuel),
    Error(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InterpretSuccess {
    pub value: KediValue,
    pub fuel_used: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InterpretOutOfFuel {}

pub fn run(
    options: InterpretOptions,
    simple: &simple::Module,
    fun_name: &str,
    args: Vec<KediValue>,
) -> InterpretResult {
    let mut env = InterpretEnv::new(options.fuel_limit);

    // Populate the environment with the functions
    for stmt in &simple.statements {
        match stmt {
            simple::TopLevelStmt::FunDecl(fun) => {
                env.functions
                    .insert(fun.value.name.value.clone(), fun.value.clone());
            }
        }
    }

    // Fetch the main function
    env.call(syntax::Ident(fun_name.to_string()), args)
}

struct InterpretEnv {
    functions: HashMap<syntax::Ident, simple::FunDecl>,
    fuel_used: u64,
    fuel_limit: Option<u64>,
}

impl InterpretEnv {
    fn new(fuel_limit: Option<u64>) -> Self {
        InterpretEnv {
            functions: HashMap::new(),
            fuel_used: 0,
            fuel_limit,
        }
    }

    fn ret_success(&self, value: KediValue) -> InterpretResult {
        InterpretResult::Success(InterpretSuccess {
            value,
            fuel_used: self.fuel_used,
        })
    }

    fn call(&mut self, name: syntax::Ident, args: Vec<KediValue>) -> InterpretResult {
        let fun = match self.functions.get(&name) {
            Some(f) => f.clone(),
            None => return InterpretResult::Error(format!("Function {} not found", name.0)),
        };

        let mut st = FuncState::new(self, &fun.refs);

        if args.len() != fun.implementation.value.parameters.value.len() {
            panic!("Wrong number of arguments");
        }

        for (param, arg) in fun.implementation.value.parameters.value.iter().zip(args) {
            st.locals.insert(param.value.clone(), arg);
        }

        for stmt in &fun.implementation.value.body.value {
            match st.interpret_stmt(stmt) {
                InterpretStmtResult::Ok => {}
                InterpretStmtResult::Return(value) => {
                    return self.ret_success(value);
                }
                InterpretStmtResult::Error(str) => {
                    return InterpretResult::Error(str);
                }
                InterpretStmtResult::OutOfFuel => {
                    return InterpretResult::OutOfFuel(InterpretOutOfFuel {});
                }
            }
        }

        todo!();
    }
}

struct FuncState<'t> {
    locals: HashMap<plain::LocalIdent, KediValue>,
    single_use: HashMap<simple::SingleUseIdent, KediValue>,
    interpret_env: &'t mut InterpretEnv,
    refs: HashMap<plain::GlobalIdent, syntax::Ident>,
}

impl<'t> FuncState<'t> {
    fn new(
        interpret_env: &'t mut InterpretEnv,
        refs: &Bimap<plain::GlobalIdent, syntax::Ident>,
    ) -> Self {
        FuncState {
            locals: HashMap::new(),
            single_use: HashMap::new(),
            interpret_env,
            refs: refs.iter().map(|(k, v)| (k.clone(), v.clone())).collect(),
        }
    }

    fn resolve(&self, ident: &simple::Ident) -> &KediValue {
        match ident {
            simple::Ident::SingleUse(i) => self.single_use.get(&i.value).unwrap(),
            simple::Ident::Local(i) => self.locals.get(&i.value).unwrap(),
        }
    }

    fn call_global(&mut self, name: &plain::GlobalIdent, args: Vec<KediValue>) -> InterpretResult {
        let fun_name = self.refs.get(name).unwrap().clone();
        self.interpret_env.call(fun_name, args)
    }

    fn interpret_stmts(&mut self, stmts: &[simple::FunStmt]) -> InterpretStmtResult {
        for stmt in stmts {
            match self.interpret_stmt(stmt) {
                InterpretStmtResult::Ok => {}
                InterpretStmtResult::Return(value) => {
                    return InterpretStmtResult::Return(value);
                }
                InterpretStmtResult::OutOfFuel => {
                    return InterpretStmtResult::OutOfFuel;
                }
                InterpretStmtResult::Error(str) => {
                    return InterpretStmtResult::Error(str);
                }
            }
        }
        InterpretStmtResult::Ok
    }

    fn interpret_stmt(&mut self, stmt: &simple::FunStmt) -> InterpretStmtResult {
        // Do fuel stuff
        self.interpret_env.fuel_used += 1;
        if let Some(fuel_limit) = self.interpret_env.fuel_limit {
            if self.interpret_env.fuel_used > fuel_limit {
                return InterpretStmtResult::OutOfFuel;
            }
        }

        // Interpret
        match stmt {
            simple::FunStmt::Return(ident) => {
                return InterpretStmtResult::Return(self.resolve(ident).clone());
            }
            simple::FunStmt::Assignment(assignment) => {
                let value = match &assignment.value.value {
                    simple::AssignmentValue::LitNum(lit) => {
                        let i = lit.value.0;
                        KediValue::KediNum(i.into())
                    }
                    &simple::AssignmentValue::Call(ref call) => {
                        let fun_name = call.value.fun_name.value.clone();
                        let args = call
                            .value
                            .arguments
                            .value
                            .iter()
                            .map(|x| self.resolve(x))
                            .cloned()
                            .collect();
                        match self.call_global(&fun_name, args) {
                            InterpretResult::Success(s) => s.value,
                            InterpretResult::OutOfFuel(_) => {
                                return InterpretStmtResult::OutOfFuel;
                            }
                            InterpretResult::Error(err) => {
                                return InterpretStmtResult::Error(err);
                            }
                        }
                    }
                    simple::AssignmentValue::Ident(ident) => self.resolve(ident).clone(),
                };

                match &assignment.value.target {
                    simple::Ident::SingleUse(i) => {
                        self.single_use.insert(i.value.clone(), value);
                    }
                    simple::Ident::Local(i) => {
                        self.locals.insert(i.value.clone(), value);
                    }
                }
            }
            &simple::FunStmt::If(ref if_) => {
                let cond = if_.condition;
                let cond_val = self.resolve(&cond);

                if cond_val.is_truthy() {
                    return self.interpret_stmts(&if_.then.value);
                } else {
                    if let Some(else_) = &if_.else_ {
                        return self.interpret_stmts(&else_.value);
                    }
                }
            }
            &simple::FunStmt::Loop(ref loop_) => loop {
                match self.interpret_stmts(&loop_.value.body.value) {
                    InterpretStmtResult::Ok => {}
                    InterpretStmtResult::Return(value) => {
                        return InterpretStmtResult::Return(value);
                    }
                    InterpretStmtResult::OutOfFuel => {
                        return InterpretStmtResult::OutOfFuel;
                    }
                    InterpretStmtResult::Error(str) => {
                        return InterpretStmtResult::Error(str);
                    }
                }
            },
            _ => todo!(),
        }
        return InterpretStmtResult::Ok;
    }
}

enum InterpretStmtResult {
    Ok,
    Return(KediValue),
    Error(String),
    OutOfFuel,
}
