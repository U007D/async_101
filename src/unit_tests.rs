#![allow(non_snake_case)]
mod test_utils;
use super::*;
use assert2::assert;
use test_utils::FileMaker;
use std::mem::discriminant;

#[test]
fn read__non_existent_file_gives_error() {
    // Given
    let filename = "foo.txt";
    let bytes_to_read = 1_000;
    let sut = read_n_bytes;

    // When
    let res = sut(filename, bytes_to_read);

    // Then
    assert!(discriminant(&res) == discriminant(&Err(Error::IoError("".into()))));
}

#[test]
fn read__can_read_one_byte_from_existing_file() {
    // Given
    let file_contents = vec![b'x'];
    let filename = "foo.txt";
    let _testfile = FileMaker::new(filename, file_contents);
    let bytes_to_read = 1;
    let sut = read_n_bytes;

    // When
    let res = sut(filename, bytes_to_read);

    // Then
    assert!(res == Ok(vec![b'x']));
}
