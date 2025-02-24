use std::{collections::HashMap, hash::Hash};

pub trait CompilerPhase<In, Out, Err> {
    fn name(&self) -> &'static str;
    fn run(&self, input: In) -> Result<Out, Err>;
}

pub struct Namespace<Name, Value> {
    pub values: HashMap<Name, Value>,
}

pub struct TransformPhase<In: 'static, Out: 'static, Err: 'static> {
    pub name: &'static str,
    pub run: Box<dyn Fn(In) -> Result<Out, Err> + 'static>,
}

impl<In: 'static, Out: 'static, Err: 'static> CompilerPhase<In, Out, Err>
    for TransformPhase<In, Out, Err>
{
    fn name(&self) -> &'static str {
        self.name
    }
    fn run(&self, input: In) -> Result<Out, Err> {
        (self.run)(input)
    }
}

pub struct ProcessPhase<In: 'static, Err: 'static> {
    pub name: &'static str,
    pub run: Box<dyn Fn(In) -> Result<In, Err> + 'static>,
}

impl<In: 'static, Err: 'static> CompilerPhase<In, In, Err> for ProcessPhase<In, Err> {
    fn name(&self) -> &'static str {
        self.name
    }
    fn run(&self, input: In) -> Result<In, Err> {
        (self.run)(input)
    }
}

pub struct TransformMapPhase<In: 'static, Out: 'static, Err: 'static> {
    pub name: &'static str,
    pub run: Box<dyn Fn(In) -> Result<Out, Err> + 'static>,
}

impl<Name: 'static + Hash + Eq, In: 'static, Out: 'static, Err: 'static>
    CompilerPhase<Namespace<Name, In>, Namespace<Name, Out>, Err>
    for TransformMapPhase<In, Out, Err>
{
    fn name(&self) -> &'static str {
        self.name
    }
    fn run(&self, input: Namespace<Name, In>) -> Result<Namespace<Name, Out>, Err> {
        let mut values = HashMap::new();
        for (name, value) in input.values {
            values.insert(name, (self.run)(value)?);
        }
        Ok(Namespace { values })
    }
}

pub struct Compiler<Current, Err> {
    current: Current,
    _err: std::marker::PhantomData<Err>,
}

impl<Current, Err> Compiler<Current, Err> {
    pub fn new(current: Current) -> Self {
        Compiler {
            current,
            _err: std::marker::PhantomData,
        }
    }

    pub fn run_phase<PhaseOut, PhaseErr, Phase>(
        self,
        phase: &Phase,
    ) -> Result<Compiler<PhaseOut, Err>, Err>
    where
        Phase: CompilerPhase<Current, PhaseOut, PhaseErr>,
        PhaseErr: Into<Err>,
    {
        let phase_out = phase.run(self.current).map_err(|err| err.into())?;
        Ok(Compiler::new(phase_out))
    }
}
