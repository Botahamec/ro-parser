#![feature(test)]

extern crate ro_backend;

mod function;
mod result;
mod tokenizer;
mod program;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod benches;

//use ro_backend::{Program, Function, Parameter};

// the main function
//fn parse(code: String) -> Program {tokenize(code)}
