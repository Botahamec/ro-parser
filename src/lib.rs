#![feature(test)]
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
pub fn parse(code: String) -> program::Program {
	program::ProgramParser::from_tokens(tokenizer::tokenize(code)).parse()
}
