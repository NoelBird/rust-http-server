use std::{fs, io};

pub(crate) fn read_file(path: String) -> io::Result<Vec<u8>> {
    fs::read(path)
}

pub(crate) fn write_file(path: String, contents: &[u8]) -> io::Result<()> {
    fs::write(path, &contents)
}
