#![allow(non_snake_case)]
use assert2::assert;
use super::*;
use std::io::Write;

#[derive(Debug)]
struct FileMaker<P: AsRef<Path>> {
    filename: P,
}

impl<P: AsRef<Path>> FileMaker<P> {
    pub fn new(filename: P, contents: Vec<u8>) -> Result<Self> {
        let res = Self { filename };
        File::create(&res.filename)?.write_all(&contents)?;
        Ok(res)
    }
}

impl<P: AsRef<Path>> Drop for FileMaker<P> {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.filename);
    }
}

#[test]
fn read__non_existent_file_gives_error() {
    // Given
    let expected_res = Err(Error::IoError("No such file or directory (os error 2)".into()));
    let filename = "foo.txt";
    let bytes_to_read = 1_000;
    let sut = read_n_bytes;

    // When
    let res = sut(filename, bytes_to_read);

    // Then
    assert!(res == expected_res);
}

#[test]
fn read__can_read_one_byte_from_existing_file() {
    // Given
    let file_contents = vec![b'x'];
    let expected_res = Ok(file_contents.clone());
    let filename = "foo.txt";
    let _testfile = FileMaker::new(filename, file_contents);
    let bytes_to_read = 1;
    let sut = read_n_bytes;

    // When
    let res = sut(filename, bytes_to_read);

    // Then
    assert!(res == expected_res);
}
