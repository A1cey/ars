//! A1cey's personal utility crate.

#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
pub mod apq;
pub mod ascii;
pub mod counted_enum;
pub mod fmt;
pub mod range;
