use std::convert::TryInto;

use proptest::strategy::Just;

use liblumen_alloc::erts::term::prelude::Atom;

use crate::erlang::atom_to_binary_2::result;
use crate::test::strategy;

#[test]
fn without_atom_errors_badarg() {
    run!(
        |arc_process| {
            (
                Just(arc_process.clone()),
                strategy::term::is_not_atom(arc_process.clone()),
                strategy::term::is_encoding(),
            )
        },
        |(arc_process, atom, encoding)| {
            prop_assert_is_not_atom!(result(&arc_process, atom, encoding), atom);

            Ok(())
        },
    );
}

#[test]
fn with_atom_without_atom_encoding_errors_badarg() {
    run!(
        |arc_process| {
            (
                Just(arc_process.clone()),
                strategy::term::atom(),
                strategy::term::is_not_atom(arc_process.clone()),
            )
        },
        |(arc_process, atom, encoding)| {
            prop_assert_badarg!(
                result(&arc_process, atom, encoding),
                format!("invalid encoding name value: `{}` is not an atom", encoding)
            );

            Ok(())
        },
    );
}

#[test]
fn with_atom_with_atom_without_name_encoding_errors_badarg() {
    run!(
        |arc_process| {
            (
                Just(arc_process.clone()),
                strategy::term::atom(),
                strategy::term::atom::is_not_encoding(),
            )
        },
        |(arc_process, atom, encoding)| {
            let encoding_atom: Atom = encoding.try_into().unwrap();

            prop_assert_badarg!(result(&arc_process, atom, encoding), format!("invalid atom encoding name: '{}' is not one of the supported values (latin1, unicode, or utf8)", encoding_atom.name()));

            Ok(())
        },
    );
}

// `with_atom_with_encoding_atom_returns_name_in_binary` in integration tests
