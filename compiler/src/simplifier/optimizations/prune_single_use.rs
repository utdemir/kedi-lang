use std::mem;

use crate::simplifier::simple;

pub fn run(impl_: &mut simple::FunImpl) {
    let mut body = &mut impl_.body.value;
    let len = body.len();

    let mut current_ix = 0;
    let mut next_ix = 1;

    while next_ix < len {
        let (curr, next) = indices::indices!(&mut body, current_ix, next_ix);

        match (&curr, next) {
            (
                simple::FunStmt::Assignment(ref assignment_curr),
                simple::FunStmt::Assignment(ref mut assignment_next),
            ) => {
                let sui_curr = assignment_target_single_use_identifier(&assignment_curr.value);
                let sui_next = assignment_value_single_use_identifier(&assignment_next.value);

                if sui_curr.is_some() && sui_curr == sui_next {
                    // std::mem::swap(assignment_curr.value, assignment_next.value);
                    // *curr = simple::FunStmt::Nop;

                    assignment_next.value = simple::Assignment {
                        target: assignment_next.value.target.clone(),
                        value: assignment_curr.value.value.clone(),
                    };

                    let _ = mem::replace(curr, simple::FunStmt::Nop);
                }
            }
            _ => {}
        }

        current_ix += 1;
        next_ix += 1;
    }
}

fn assignment_target_single_use_identifier(
    assignment: &simple::Assignment,
) -> Option<simple::SingleUseIdent> {
    match assignment.target {
        simple::Ident::SingleUse(sid) => Some(sid.value),
        _ => None,
    }
}

fn assignment_value_single_use_identifier(
    assignment: &simple::Assignment,
) -> Option<simple::SingleUseIdent> {
    match assignment.value {
        simple::AssignmentValue::Ident(simple::Ident::SingleUse(sid)) => Some(sid.value),
        _ => None,
    }
}
