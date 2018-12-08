extern crate libc;
extern crate simple_signal;

mod native;
mod current_state;

use std::thread::sleep;
use std::time::Duration;
use current_state::CurrentState;
use current_state::StateWrapper;
use std::mem::size_of;
use simple_signal::{set_handler, Signal};
use std::process::exit;
use std::ptr::NonNull;

use native::shared::*;

fn main() {
    let duration = Duration::from_secs(1);
    let memory_id =
        allocate_mem(size_of::<CurrentState>()).expect("Failed to allocate shared memory");
    let shared_mem = attach_mem(memory_id).expect(
        "Failed to attach the allocated shared memory",
    ) as *mut CurrentState;
    let ptr_wrapper = StateWrapper(NonNull::new(shared_mem).unwrap());
    set_handler(&[Signal::Term, Signal::Int], move |_| {
        detach_mem(ptr_wrapper.0.as_ptr()).expect("Unable to detach the shared memory");
        remove_mem(memory_id).expect("Unable to remove the shared memory");
        exit(0);
    });
    let mut state = CurrentState::new();
    loop {
        unsafe {
            shared_mem.write(state.clone());
        }
        sleep(duration);
        state.update();
    }
}
