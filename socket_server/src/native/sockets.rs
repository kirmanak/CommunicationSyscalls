use native::errno::Errno;
use std::fs::File;
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::path::PathBuf;
use std::mem::size_of;
use std::ptr::null_mut;
use bincode::serialize_into;
use serde::ser::Serialize;

use native::strings::path_to_c;

use libc::{socket, bind, AF_UNIX, SOCK_STREAM, sockaddr_un, c_char, sockaddr, socklen_t, c_int,
           listen, fcntl, F_SETFL, O_NONBLOCK, accept, EAGAIN, EWOULDBLOCK};

pub fn create_socket() -> Result<File, Errno> {
    let status = unsafe { socket(AF_UNIX, SOCK_STREAM, 0) };
    if status == -1 {
        Err(Errno::current())
    } else {
        let file = unsafe { File::from_raw_fd(status) };
        Ok(file)
    }
}

pub fn bind_socket(sock: &File, path: &PathBuf) -> Result<(), Errno> {
    let fd = sock.as_raw_fd();
    let path = path_to_c(path).into_bytes();
    let mut address = sockaddr_un {
        sun_family: AF_UNIX as u16,
        sun_path: vec_to_array(&path),
    };
    let pointer = &mut address as *mut _ as *mut sockaddr;
    if unsafe { bind(fd, pointer, size_of::<sockaddr_un>() as socklen_t) } == -1 {
        Err(Errno::current())
    } else {
        Ok(())
    }
}

pub fn set_nonblock(file: &File) -> Result<(), Errno> {
    if unsafe { fcntl(file.as_raw_fd(), F_SETFL, O_NONBLOCK) } == -1 {
        Err(Errno::current())
    } else {
        Ok(())
    }
}

pub fn listen_socket(sock: &File, backlog: c_int) -> Result<(), Errno> {
    if unsafe { listen(sock.as_raw_fd(), backlog) } == 0 {
        Ok(())
    } else {
        Err(Errno::current())
    }
}

/// Checks whether a client is listening or not. If it is the case, writes the provided data.
/// # Panics
/// Almost always. I mean it can panic in every second line of code.
pub fn check_write<T: Serialize>(sock: &mut File, data: &T) {
    let connection = unsafe { accept(sock.as_raw_fd(), null_mut(), null_mut()) };
    if connection == -1 {
        let errno = Errno::current();
        if errno.code != EAGAIN && errno.code != EWOULDBLOCK {
            panic!("Fail to accept a new connection: {:?}", errno);
        }
    } else {
        let connection = unsafe { File::from_raw_fd(connection) };
        serialize_into(connection, data).expect("Fail to write data to the socket");
    }
}

fn vec_to_array(vector: &Vec<u8>) -> [c_char; 108] {
    let mut array = [0i8; 108];
    for (counter, element) in vector.iter().enumerate() {
        array[counter] = *element as c_char;
    }
    array
}
