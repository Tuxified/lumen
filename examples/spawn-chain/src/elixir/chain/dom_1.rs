use std::sync::Arc;

use liblumen_alloc::erts::exception::system::Alloc;
use liblumen_alloc::erts::process::code::stack::frame::{Frame, Placement};
use liblumen_alloc::erts::process::{code, Process};
use liblumen_alloc::erts::term::prelude::*;
use liblumen_alloc::erts::ModuleFunctionArity;

use crate::elixir::chain::{dom_output_1, run_2};

/// ```elixir
/// # pushed to stack: (n)
/// # returned from call: N/A
/// # full stack: (n)
/// # returns: final_answer
/// def dom(n) do
///   run(n, &dom_output/1)
/// end
/// ```
pub fn place_frame_with_arguments(
    process: &Process,
    placement: Placement,
    n: Term,
) -> Result<(), Alloc> {
    assert!(n.is_integer());
    process.stack_push(n)?;
    process.place_frame(frame(), placement);

    Ok(())
}

// Private

fn code(arc_process: &Arc<Process>) -> code::Result {
    arc_process.reduce();

    let n = arc_process.stack_pop().unwrap();
    assert!(n.is_integer());

    let dom_output_closure = dom_output_1::closure(arc_process)?;
    run_2::place_frame_with_arguments(arc_process, Placement::Replace, n, dom_output_closure)?;

    Process::call_code(arc_process)
}

fn frame() -> Frame {
    Frame::new(module_function_arity(), code)
}

fn function() -> Atom {
    Atom::try_from_str("dom").unwrap()
}

fn module_function_arity() -> Arc<ModuleFunctionArity> {
    Arc::new(ModuleFunctionArity {
        module: super::module(),
        function: function(),
        arity: 1,
    })
}
