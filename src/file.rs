use std::{fs, io};

pub(crate) fn read_file(path: String) -> io::Result<Vec<u8>> {
    fs::read(path)
}
