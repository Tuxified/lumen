use liblumen_alloc::erts::process::Process;
use liblumen_alloc::erts::term::prelude::Term;

#[native_implemented::function(erlang:erase/1)]
pub fn result(process: &Process, key: Term) -> Term {
    process.erase_value_from_key(key)
}
