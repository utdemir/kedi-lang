use crate::simplifier::simple;

pub fn run(impl_: &mut simple::FunImpl) {
    impl_
        .body
        .value
        .retain(|stmt| !matches!(stmt, simple::FunStmt::Nop));
}
