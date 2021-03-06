#[cfg(all(not(target_arch = "wasm32"), test))]
mod test;

use anyhow::*;
use num_bigint::BigInt;

use liblumen_alloc::erts::exception::{self, *};
use liblumen_alloc::erts::process::trace::Trace;
use liblumen_alloc::erts::process::Process;
use liblumen_alloc::erts::term::prelude::*;

/// `bnot/1` prefix operator.
#[native_implemented::function(erlang:bnot/1)]
pub fn result(process: &Process, integer: Term) -> exception::Result<Term> {
    match integer.decode().unwrap() {
        TypedTerm::SmallInteger(small_integer) => {
            let integer_isize: isize = small_integer.into();
            let output = !integer_isize;
            let output_term = process.integer(output);

            Ok(output_term)
        }
        TypedTerm::BigInteger(big_integer) => {
            let big_int: &BigInt = big_integer.as_ref().into();
            let output_big_int = !big_int;
            let output_term = process.integer(output_big_int);

            Ok(output_term)
        }
        _ => Err(badarith(
            Trace::capture(),
            Some(anyhow!("integer ({}) is not an integer", integer).into()),
        )
        .into()),
    }
}
