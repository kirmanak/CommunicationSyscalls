mod errno;
mod strings;

use self::errno::Errno;
use std::fs::File;
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::path::PathBuf;
use std::mem::size_of;
use bincode::deserialize_from;
use serde::de::DeserializeOwned;

use native::strings::path_to_c;

use libc::{socket, connect, AF_UNIX, SOCK_STREAM, sockaddr_un, c_char, sockaddr, socklen_t,
           sa_family_t};

pub fn create_socket() -> Result<File, Errno> {
    let status = unsafe { socket(AF_UNIX, SOCK_STREAM, 0) };
    if status == -1 {
        Err(Errno::current())
    } else {
        let file = unsafe { File::from_raw_fd(status) };
        Ok(file)
    }
}

pub fn connect_socket(sock: &File, path: &PathBuf) -> Result<sockaddr_un, Errno> {
    let fd = sock.as_raw_fd();
    let path = path_to_c(path).into_bytes();
    let mut address = sockaddr_un {
        sun_family: AF_UNIX as sa_family_t,
        sun_path: vec_to_array(&path),
    };
    let pointer = &mut address as *mut _ as *mut sockaddr;
    if unsafe { connect(fd, pointer, size_of::<sockaddr_un>() as socklen_t) } == 0 {
        Ok(address)
    } else {
        Err(Errno::current())
    }
}

fn vec_to_array(vector: &Vec<u8>) -> [c_char; 108] {
    let mut array = [0i8; 108];
    for (counter, element) in vector.iter().enumerate() {
        array[counter] = *element as c_char;
    }
    array
}

pub fn read_data<T: DeserializeOwned>(socket_file: &File) -> Result<T, Errno> {
    Ok(deserialize_from(socket_file).expect(
        "Unable to read data from the socket",
    ))
}
