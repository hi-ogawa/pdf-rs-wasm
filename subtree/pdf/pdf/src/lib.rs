#![allow(non_camel_case_types)] /* TODO temporary becaues of pdf_derive */
#![allow(unused_doc_comments)] // /* TODO temporary because of err.rs */
#![allow(
    clippy::len_zero,
    clippy::should_implement_trait,
    clippy::manual_map,
    clippy::from_over_into
)]

#[macro_use]
extern crate pdf_derive;
#[macro_use]
extern crate snafu;
#[macro_use]
extern crate log;

#[macro_use]
pub mod error;
pub mod any;
pub mod backend;
pub mod build;
pub mod content;
pub mod encoding;
pub mod file;
pub mod font;
pub mod object;
pub mod parser;
pub mod primitive;
pub mod xref;

// mod content;
pub mod crypt;
pub mod enc;

// pub use content::*;
pub use crate::error::PdfError;
