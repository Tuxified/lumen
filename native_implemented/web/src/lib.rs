#![deny(warnings)]

pub mod r#async;
pub mod document;
pub mod element;
pub mod event;
pub mod event_listener;
pub mod executor;
pub mod html_form_element;
pub mod html_input_element;
pub mod js_value;
pub mod math;
pub mod node;
pub mod promise;
pub mod web_socket;
pub mod window;

pub use lumen_rt_full as runtime;

use std::cell::RefCell;
use std::rc::Rc;

use panic_control::chain_hook_ignoring;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use web_sys::{DomException, Window};

#[cfg(not(test))]
use liblumen_core::entry;

use liblumen_alloc::atom;
use liblumen_alloc::erts::process::Process;
use liblumen_alloc::erts::term::prelude::Term;
use liblumen_alloc::erts::time::Milliseconds;

use crate::runtime::scheduler;
use crate::runtime::time::monotonic;
use crate::window::add_event_listener;

/// Starts the scheduler loop.  It yield and reschedule itself using
/// [requestAnimationFrame](https://developer.mozilla.org/en-US/docs/Web/API/window/requestAnimationFrame).
#[cfg_attr(not(test), entry)]
pub fn start() {
    // Ignore panics created by full runtime's `__lumen_start_panic`.  `catch_unwind` although
    // it stops the panic does not suppress the printing of the panic message and stack
    // backtrace without this.
    chain_hook_ignoring::<Term>();
    add_event_listeners();
    request_animation_frames();
}

// Private

const MILLISECONDS_PER_SECOND: MillisecondsPerSecond = MillisecondsPerSecond(1000);
const FRAMES_PER_SECOND: FramesPerSecond = FramesPerSecond(60);
const MILLISECONDS_PER_FRAME: MillisecondsPerFrame =
    MILLISECONDS_PER_SECOND.const_div(FRAMES_PER_SECOND);

fn add_event_listeners() {
    let window = web_sys::window().unwrap();
    add_submit_listener(&window);
}

fn add_submit_listener(window: &Window) {
    add_event_listener(
        window,
        "submit",
        window::module(),
        window::on_submit_1::function(),
        Default::default(),
    );
}

fn error_tuple(process: &Process, js_value: JsValue) -> Term {
    let error = atom!("error");
    let dom_exception = js_value.dyn_ref::<DomException>().unwrap();

    match dom_exception.name().as_ref() {
        "SyntaxError" => {
            let tag = atom!("syntax");
            let message = process.binary_from_str(&dom_exception.message());
            let reason = process.tuple_from_slice(&[tag, message]);

            process.tuple_from_slice(&[error, reason])
        }
        name => unimplemented!(
            "Converting {} DomException: {}",
            name,
            dom_exception.message()
        ),
    }
}

fn ok_tuple<V: Clone + 'static>(process: &Process, value: V) -> Term {
    let ok = atom!("ok");
    let resource_term = process.resource(value);

    process.tuple_from_slice(&[ok, resource_term])
}

fn option_to_ok_tuple_or_error<V: Clone + 'static>(process: &Process, option: Option<V>) -> Term {
    match option {
        Some(value) => ok_tuple(process, value),
        None => atom!("error"),
    }
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    web_sys::window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .unwrap();
}

fn request_animation_frames() {
    // Based on https://github.com/rustwasm/wasm-bindgen/blob/603d5742eeca2a7a978f13614de9282229d1835e/examples/request-animation-frame/src/lib.rs
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        run_for_milliseconds(MILLISECONDS_PER_FRAME.const_mul(Frames(1)));

        // Schedule ourselves for another requestAnimationFrame callback.
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
}

fn run_for_milliseconds(duration: Milliseconds) {
    let scheduler = scheduler::current();
    let timeout = monotonic::time() + duration;

    while (monotonic::time() < timeout) && scheduler.run_once() {}
}

struct Frames(u64);

struct FramesPerSecond(u64);

struct MillisecondsPerFrame(u64);

impl MillisecondsPerFrame {
    const fn const_mul(self, rhs: Frames) -> Milliseconds {
        Milliseconds(self.0 * rhs.0)
    }
}

struct MillisecondsPerSecond(u64);

impl MillisecondsPerSecond {
    const fn const_div(self, rhs: FramesPerSecond) -> MillisecondsPerFrame {
        MillisecondsPerFrame(self.0 / rhs.0)
    }
}
