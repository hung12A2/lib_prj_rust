//! # Serde JSON
//!
//! Json là một định dạng dưới dạng key, value, có thể đọc được
//! Được sử dụng chủ yếu trong việc truyền đi dữ liệu
//!
//! Có ba cách thông dụng mà bạn có thể gặp phải với dữ liệu JSON trong Rust.
//! - Dạng dữ liệu văn bản. 
//! Một chuỗi dữ liệu JSON chưa xử lý mà bạn nhận được tại một điểm cuối HTTP, 
//! đọc từ một tập tin hoặc chuẩn bị gửi đến một máy chủ từ xa.
//! - Dạng biểu diễn không có kiểu
//! Có thể bạn muốn kiểm tra một số dữ liệu JSON hợp lệ trước khi truyền nó đi, 
//! nhưng không biết cấu trúc của nó. 
//! Hoặc bạn muốn thực hiện các thao tác cơ bản như chèn một khóa tại một vị trí nhất định.
//! - Dang cấu trúc dữ liệu được kiểu hóa mạnh trong Rust. 
//! Khi bạn mong đợi tất cả hoặc hầu hết dữ liệu phù hợp với cấu trúc cụ thể 
//! và muốn thực hiện thao tác với phần dữ liệu đấy 
//! 
//! 
//! ```json
//! {
//!     "name": "John Doe",
//!     "age": 43,
//!     "address": {
//!         "street": "10 Downing Street",
//!         "city": "London"
//!     },
//!     "phones": [
//!         "+44 1234567",
//!         "+44 2345678"
//!     ]
//! }
//! ```
//!
//! Serde JSON provides efficient, flexible, safe ways of converting data
//! between each of these representations.
//!
//! # Operating on untyped JSON values
//!
//! Any valid JSON data can be manipulated in the following recursive enum
//! representation. This data structure is [`serde_json::Value`][value].
//!
//! ```
//! # use serde_json::{Number, Map};
//! #
//! # #[allow(dead_code)]
//! enum Value {
//!     Null,
//!     Bool(bool),
//!     Number(Number),
//!     String(String),
//!     Array(Vec<Value>),
//!     Object(Map<String, Value>),
//! }
//! ```
//!
//! A string of JSON data can be parsed into a `serde_json::Value` by the
//! [`serde_json::from_str`][from_str] function. There is also
//! [`from_slice`][from_slice] for parsing from a byte slice &[u8] and
//! [`from_reader`][from_reader] for parsing from any `io::Read` like a File or
//! a TCP stream.
//!
//! ```
//! use serde_json::{Result, Value};
//!
//! fn untyped_example() -> Result<()> {
//!     // Some JSON input data as a &str. Maybe this comes from the user.
//!     let data = r#"
//!         {
//!             "name": "John Doe",
//!             "age": 43,
//!             "phones": [
//!                 "+44 1234567",
//!                 "+44 2345678"
//!             ]
//!         }"#;
//!
//!     // Parse the string of data into serde_json::Value.
//!     let v: Value = serde_json::from_str(data)?;
//!
//!     // Access parts of the data by indexing with square brackets.
//!     println!("Please call {} at the number {}", v["name"], v["phones"][0]);
//!
//!     Ok(())
//! }
//! #
//! # fn main() {
//! #     untyped_example().unwrap();
//! # }
//! ```
//! use serde::{Deserialize, Serialize};
//! use serde_json::Result;
//!
//! #[derive(Serialize, Deserialize)]
//! struct Person {
//!     name: String,
//!     age: u8,
//!     phones: Vec<String>,
//! }
//!
//! fn typed_example() -> Result<()> {
//!     // Some JSON input data as a &str. Maybe this comes from the user.
//!     let data = r#"
//!         {
//!             "name": "John Doe",
//!             "age": 43,
//!             "phones": [
//!                 "+44 1234567",
//!                 "+44 2345678"
//!             ]
//!         }"#;
//!
//!     let p: Person = serde_json::from_str(data)?;
//!
//!     println!("Please call {} at the number {}", p.name, p.phones[0]);
//!
//!     Ok(())
//! }
//! #
//! # fn main() {
//! #     typed_example().unwrap();
//! # }
//! ```
//!

#![doc(html_root_url = "https://docs.rs/serde_json/1.0.92")]
// Ignored clippy lints
#![allow(
    clippy::collapsible_else_if,
    clippy::comparison_chain,
    clippy::deprecated_cfg_attr,
    clippy::doc_markdown,
    clippy::excessive_precision,
    clippy::explicit_auto_deref,
    clippy::float_cmp,
    clippy::manual_range_contains,
    clippy::match_like_matches_macro,
    clippy::match_single_binding,
    clippy::needless_doctest_main,
    clippy::needless_late_init,
    // clippy bug: https://github.com/rust-lang/rust-clippy/issues/8366
    clippy::ptr_arg,
    clippy::return_self_not_must_use,
    clippy::transmute_ptr_to_ptr,
    clippy::unnecessary_wraps,
    // clippy bug: https://github.com/rust-lang/rust-clippy/issues/5704
    clippy::unnested_or_patterns,
)]
// Ignored clippy_pedantic lints
#![allow(
    // buggy
    clippy::iter_not_returning_iterator, // https://github.com/rust-lang/rust-clippy/issues/8285
    // Deserializer::from_str, into_iter
    clippy::should_implement_trait,
    // integer and float ser/de requires these sorts of casts
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    // correctly used
    clippy::enum_glob_use,
    clippy::if_not_else,
    clippy::integer_division,
    clippy::map_err_ignore,
    clippy::match_same_arms,
    clippy::similar_names,
    clippy::unused_self,
    clippy::wildcard_imports,
    // things are often more readable this way
    clippy::cast_lossless,
    clippy::module_name_repetitions,
    clippy::redundant_else,
    clippy::shadow_unrelated,
    clippy::single_match_else,
    clippy::too_many_lines,
    clippy::unreadable_literal,
    clippy::unseparated_literal_suffix,
    clippy::use_self,
    clippy::zero_prefixed_literal,
    // we support older compilers
    clippy::checked_conversions,
    clippy::mem_replace_with_default,
    // noisy
    clippy::missing_errors_doc,
    clippy::must_use_candidate,
)]
#![allow(non_upper_case_globals)]
#![deny(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]

extern crate alloc;

#[cfg(feature = "std")]
#[doc(inline)]
pub use crate::de::from_reader;
#[doc(inline)]
pub use crate::de::{from_slice, from_str, Deserializer, StreamDeserializer};
#[doc(inline)]
pub use crate::error::{Error, Result};
#[doc(inline)]
pub use crate::ser::{to_string, to_string_pretty, to_vec, to_vec_pretty};
#[cfg(feature = "std")]
#[doc(inline)]
pub use crate::ser::{to_writer, to_writer_pretty, Serializer};
#[doc(inline)]
pub use crate::value::{from_value, to_value, Map, Number, Value};

// We only use our own error type; no need for From conversions provided by the
// standard library's try! macro. This reduces lines of LLVM IR by 4%.
macro_rules! tri {
    ($e:expr $(,)?) => {
        match $e {
            core::result::Result::Ok(val) => val,
            core::result::Result::Err(err) => return core::result::Result::Err(err),
        }
    };
}

#[macro_use]
mod macros;

pub mod de;
pub mod error;
pub mod map;
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub mod ser;
pub mod value;


mod io;
#[cfg(feature = "std")]
mod iter;
mod number;
mod read;

