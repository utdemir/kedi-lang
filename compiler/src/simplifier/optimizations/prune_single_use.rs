use crate::simplifier::simple;

pub fn run(impl_: &mut simple::FunImpl) {
    let mut body = &mut impl_.body.value;
    let len = body.len();

    let mut current_ix = 0;
    let mut next_ix = 1;

    while next_ix < len {
        let (curr, next) = indices::indices!(&mut body, current_ix, next_ix);

        match (&mut curr.value, &mut next.value) {
            (
                simple::Statement::Assignment(assignment_curr),
                simple::Statement::Assignment(assignment_next),
            ) => {
                let sui_curr = assignment_target_single_use_identifier(&assignment_curr);
                let sui_next = assignment_value_single_use_identifier(&assignment_next);

                if sui_curr.is_some() && sui_curr == sui_next {
                    std::mem::swap(&mut assignment_curr.value, &mut assignment_next.value);
                    *curr = curr.map(|_| simple::Statement::Nop);
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
) -> Option<simple::SingleUseIdentifier> {
    match assignment.target.value {
        simple::Identifier::SingleUse(sid) => Some(sid),
        _ => None,
    }
}

fn assignment_value_single_use_identifier(
    assignment: &simple::Assignment,
) -> Option<simple::SingleUseIdentifier> {
    match assignment.value.value {
        simple::AssignmentValue::Identifier(simple::Identifier::SingleUse(sid)) => Some(sid),
        _ => None,
    }
}
