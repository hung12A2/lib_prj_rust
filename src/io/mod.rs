//! Một lib nhỏ gọn, thân thiện với no_std xung quanh std::io.
//! Trường hợp có sử dụng std sẽ export std::io
pub use self::imp::{Error, ErrorKind, Result, Write};

#[cfg(not(feature = "std"))]
#[path = "core.rs"]
mod imp;

#[cfg(feature = "std")]
use std::io as imp;

#[cfg(feature = "std")]
pub use std::io::{Bytes, Read};
