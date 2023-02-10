//! Tái tạo lại nền tảng logic và các kiểu từ std::io 
//! theo một cách thân thiện với alloc.

use alloc::vec::Vec;
use core::fmt::{self, Display};
use core::result;

pub enum ErrorKind {
    Other,
}

// Các lỗi IO không bao giờ xảy ra trong chế độ no std
pub struct Error;

impl Display for Error {
    fn fmt(&self, _formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        unreachable!() // panic 
    }
}

impl Error {
    pub(crate) fn new(_kind: ErrorKind, _error: &'static str) -> Error {
        Error
    }
}

pub type Result<T> = result::Result<T, Error>;

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()> {
        // Tất cả phần thực thi write trong chế độ no-std
        // luôn viết toàn bộ vào bộ đệm
        let result = self.write(buf);
        debug_assert!(result.is_ok());
        debug_assert_eq!(result.unwrap_or(0), buf.len());
        Ok(())
    }

    fn flush(&mut self) -> Result<()>;
}

impl<W: Write> Write for &mut W {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        (*self).write(buf)
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> Result<()> {
        (*self).write_all(buf)
    }

    #[inline]
    fn flush(&mut self) -> Result<()> {
        (*self).flush()
    }
}

impl Write for Vec<u8> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.extend_from_slice(buf);
        Ok(buf.len())
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> Result<()> {
        self.extend_from_slice(buf);
        Ok(())
    }

    #[inline]
    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}
