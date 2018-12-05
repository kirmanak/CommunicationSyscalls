extern crate libc;
extern crate dirs;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate bincode;

mod native;
mod current_state;

use current_state::CurrentState;
use dirs::cache_dir;

use native::{create_socket, connect_socket, read_data};

const FILENAME: &'static str = "225072_socket";

fn main() {
    let mut tmp_path = cache_dir().expect("Unable to find a dir to create the temporary file");
    tmp_path.push(FILENAME);
    let sock_file = create_socket().expect("Unable to create a socket");
    connect_socket(&sock_file, &tmp_path).expect("Unable to connect to the socket");
    let state = read_data::<CurrentState>(&sock_file).expect("Unable to read from the socket");
    println!("{:?}", state);
}
