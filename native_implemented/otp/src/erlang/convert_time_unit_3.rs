#[cfg(all(not(target_arch = "wasm32"), test))]
mod test;

use std::convert::TryInto;

use anyhow::*;
use num_bigint::BigInt;

use liblumen_alloc::erts::exception;
use liblumen_alloc::erts::process::Process;
use liblumen_alloc::erts::term::prelude::Term;

use crate::runtime::time;

#[native_implemented::function(erlang:convert_time_unit/3)]
pub fn result(
    process: &Process,
    time: Term,
    from_unit: Term,
    to_unit: Term,
) -> exception::Result<Term> {
    let time_big_int: BigInt = time
        .try_into()
        .with_context(|| format!("time ({}) must be an integer", time))?;
    let from_unit_unit = term_try_into_time_unit!(from_unit)?;
    let to_unit_unit = term_try_into_time_unit!(to_unit)?;
    let converted_big_int = time::convert(time_big_int, from_unit_unit, to_unit_unit);
    let converted_term = process.integer(converted_big_int);

    Ok(converted_term)
}
