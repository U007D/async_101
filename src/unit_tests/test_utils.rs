use crate::error::Result;
use std::{
    fs::File,
    path::Path,
    io::Write
};

#[derive(Debug)]
pub struct FileMaker<P: AsRef<Path>> {
    filename: P,
}

impl<P: AsRef<Path>> FileMaker<P> {
    pub fn new<A: AsRef<[u8]>>(filename: P, contents: A) -> Result<Self> {
        let res = Self { filename };
        File::create(&res.filename)?.write_all(contents.as_ref())?;
        Ok(res)
    }
}

impl<P: AsRef<Path>> Drop for FileMaker<P> {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.filename);
    }
}
