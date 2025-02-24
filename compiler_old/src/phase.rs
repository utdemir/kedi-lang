pub struct Module<Stmt> {
    pub statements: Vec<Stmt>,
}

pub trait UnitPhase<In, Out, Err> {
    fn name(&self) -> &'static str;
    fn run(&self, input: In) -> Result<Out, Err>;
}

pub trait UnitOptimization<In, Err>: UnitPhase<In, In, Err> {}

pub trait LinkerPhase<In, Out, Err> {
    fn name(&self) -> &'static str;
    fn run(&self, input: Module<In>) -> Result<Module<Out>, Err>;
}

pub trait LinkerOptimization<In, Err>: LinkerPhase<In, In, Err> {}

pub trait OutputPhase<In, Out, Err> {
    fn name(&self) -> &'static str;
    fn run(&self, input: Module<In>) -> Result<Out, Err>;
}
