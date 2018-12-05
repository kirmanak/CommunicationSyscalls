extern crate libc;

mod native;
mod current_state;

use current_state::CurrentState;
use std::mem::size_of;

use native::{allocate_mem, attach_mem};

fn main() {
    let memory_id =
        allocate_mem(size_of::<CurrentState>()).expect("Failed to allocate shared memory");
    let shared_mem = attach_mem(memory_id).expect(
        "Failed to attach the allocated shared memory",
    ) as *mut CurrentState;
    let state: CurrentState = unsafe { shared_mem.read() };
    println!("{:?}", state);
}
