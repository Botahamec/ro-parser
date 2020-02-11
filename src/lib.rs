#![feature(test)]

extern crate ro_backend;

pub mod function;
pub mod result;
pub mod tokenizer;
pub mod program;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod benches;

// the main function
//fn parse(code: String) -> Program {tokenize(code)}
