use crate::simplifier::simple;

pub fn run(impl_: &mut simple::FunImpl) {
    impl_.body.value.retain(|stmt| match stmt.value {
        simple::Statement::Nop => false,
        _ => true,
    });
}
