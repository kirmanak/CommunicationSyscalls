extern crate libc;
extern crate dirs;
extern crate simple_signal;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate bincode;

mod native;
mod current_state;

use std::thread::sleep;
use std::time::Duration;
use current_state::CurrentState;
use dirs::cache_dir;
use std::fs::remove_file;
use simple_signal::{Signal, set_handler};
use std::process::exit;

use native::sockets::{create_socket, bind_socket, listen_socket, set_nonblock, check_write};

const FILENAME: &'static str = "225072_socket";

fn main() {
    let duration = Duration::from_secs(1);
    let mut state = CurrentState::new();
    let mut socket_file = create_socket().expect("Unable to open a socket");
    set_nonblock(&socket_file).expect("Unable to set flag O_NONBLOCK on the socket");
    let mut tmp_path = cache_dir().expect("Unable to find a dir to create the temporary file");
    tmp_path.push(FILENAME);
    bind_socket(&socket_file, &tmp_path).expect("Unable to bind socket to a temporary file");
    set_handler(&[Signal::Term, Signal::Int], move |_| {
        remove_file(&tmp_path).expect("Unable to delete the temporary file");
        exit(0);
    });
    listen_socket(&socket_file, 5).expect("Unable to listen to the socket");
    loop {
        sleep(duration);
        state.update();
        check_write(&mut socket_file, &state);
    }
}
