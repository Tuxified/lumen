use super::*;

mod with_timer;

#[test]
fn without_timer_returns_ok_and_sends_cancel_timer_message() {
    with_process(|process| {
        let timer_reference = process.next_reference();

        assert_eq!(
            result(process, timer_reference, options(process)),
            Ok(Atom::str_to_term("ok"))
        );
        assert_eq!(
            receive_message(process),
            Some(cancel_timer_message(timer_reference, false.into(), process))
        );
    });
}
