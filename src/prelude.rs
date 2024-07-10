#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused)]

pub use crate::error::Error;
pub use std::format as f;

pub type Result<T> = core::result::Result<T, Error>;

// pub struct W<T>(pub T);
