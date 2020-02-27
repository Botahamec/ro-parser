//#![feature(test)]
#![allow(clippy::tabs_in_doc_comments)]

extern crate ro_backend;

pub mod function;
pub mod program;
pub mod result;
pub mod tokenizer;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod benches;

// the main function
//fn parse(code: String) -> Program {tokenize(code)}
