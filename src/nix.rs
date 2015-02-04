use libc::c_char;
use std;

use errno::Errno;

pub type NixResult<T> = Result<T, NixError>;

#[derive(Copy)]
pub enum NixError {
    Sys(Errno),
    InvalidPath
}

pub trait NixPath {
    fn with_nix_path<T, F>(&self, f: F) -> Result<T, NixError>
        where F: FnOnce(*const c_char) -> T;
}

impl<'a> NixPath for &'a [u8] {
    fn with_nix_path<T, F>(&self, f: F) -> Result<T, NixError>
        where F: FnOnce(*const c_char) -> T
    {
        // TODO: Extract this size as a const
        let mut buf = [0u8; 4096];

        if self.len() >= 4096 {
            return Err(NixError::InvalidPath);
        }

        match self.position_elem(&0) {
            Some(_) => Err(NixError::InvalidPath),
            None => {
                std::slice::bytes::copy_memory(&mut buf, self);
                Ok(f(buf.as_ptr() as *const c_char))
            }
        }
    }
}
