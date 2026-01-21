#![warn(clippy::pedantic)]

pub mod reader;
pub mod utils;

mod error;
mod model;

#[cfg(test)]
mod yaserde_tests;
