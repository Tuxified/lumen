use super::*;

#[test]
fn with_atom_message_adds_process_message_to_mailbox_and_returns_message() {
    with_message_adds_process_message_to_mailbox_and_returns_message(|process| {
        Term::next_local_reference(process)
    });
}

#[test]
fn with_local_reference_message_adds_process_message_to_mailbox_and_returns_message() {
    with_message_adds_process_message_to_mailbox_and_returns_message(|process| {
        Term::next_local_reference(process)
    });
}

#[test]
fn with_empty_list_message_adds_process_message_to_mailbox_and_returns_message() {
    with_message_adds_process_message_to_mailbox_and_returns_message(|_| Term::EMPTY_LIST);
}

#[test]
fn with_list_message_adds_process_message_to_mailbox_and_returns_message() {
    with_message_adds_process_message_to_mailbox_and_returns_message(|process| {
        Term::cons(0.into_process(&process), 1.into_process(&process), &process)
    });
}

#[test]
fn with_small_integer_message_adds_process_message_to_mailbox_and_returns_message() {
    with_message_adds_process_message_to_mailbox_and_returns_message(|process| {
        0.into_process(&process)
    });
}

#[test]
fn with_big_integer_message_adds_process_message_to_mailbox_and_returns_message() {
    with_message_adds_process_message_to_mailbox_and_returns_message(|process| {
        (crate::integer::small::MAX + 1).into_process(&process)
    });
}

#[test]
fn with_float_message_adds_process_message_to_mailbox_and_returns_message() {
    with_message_adds_process_message_to_mailbox_and_returns_message(|process| {
        0.0.into_process(&process)
    });
}

#[test]
fn with_local_pid_message_adds_process_message_to_mailbox_and_returns_message() {
    with_message_adds_process_message_to_mailbox_and_returns_message(|_| {
        Term::local_pid(0, 1).unwrap()
    });
}

#[test]
fn with_external_pid_message_adds_process_message_to_mailbox_and_returns_message() {
    with_message_adds_process_message_to_mailbox_and_returns_message(|process| {
        Term::external_pid(1, 2, 3, &process).unwrap()
    });
}

#[test]
fn with_tuple_message_adds_process_message_to_mailbox_and_returns_message() {
    with_message_adds_process_message_to_mailbox_and_returns_message(|process| {
        Term::slice_to_tuple(&[], &process)
    });
}

#[test]
fn with_map_message_adds_process_message_to_mailbox_and_returns_message() {
    with_message_adds_process_message_to_mailbox_and_returns_message(|process| {
        Term::slice_to_map(&[], &process)
    });
}

#[test]
fn with_heap_binary_message_adds_process_message_to_mailbox_and_returns_message() {
    with_message_adds_process_message_to_mailbox_and_returns_message(|process| {
        Term::slice_to_binary(&[], &process)
    });
}

#[test]
fn with_subbinary_message_adds_process_message_to_mailbox_and_returns_message() {
    with_message_adds_process_message_to_mailbox_and_returns_message(
        |process| bitstring!(1 :: 1, &process),
    );
}

fn with_message_adds_process_message_to_mailbox_and_returns_message<M>(message: M)
where
    M: FnOnce(&Process) -> Term,
{
    with_process(|process| {
        let destination = process.pid;
        let message = message(process);

        assert_eq!(erlang::send_2(destination, message, process), Ok(message));

        assert!(has_process_message(process, message));
    })
}