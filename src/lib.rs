
#![feature(test)]

extern crate ro_backend;

mod tokenizer;
mod function;
mod result;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod benches;

//use ro_backend::{Program, Function, Parameter};

use tokenizer::TokenList;
use function::FuncParser;
use result::ResultParser;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ProgramParser {
	pub results: Vec<ResultParser>,
	pub functions: Vec<FuncParser>
}

pub fn parse_for_results_and_fns(tokens: TokenList) -> ProgramParser {
	let mut program_parser = ProgramParser::default();

	let mut token = 0;
	while token < tokens.len() {
		if tokens[token] == "fn" {

			let mut signature = TokenList::new();
			token += 1;
			while tokens[token] != "{" {
				signature.push(tokens[token].clone());
				token += 1;
			}

			let mut code = TokenList::new();
			let mut brackets : usize = 1; // the number of brackets that need to be closed
			loop {
				token += 1;
				if tokens[token] == "{" {brackets += 1;}
				if tokens[token] == "}" {brackets -= 1;}
				if brackets == 0 {break;}
				code.push(tokens[token].clone());
			}

			program_parser.functions.push(FuncParser {signature, code})
		}

		if token > tokens.len() {break;} // prevents an error here

		if tokens[token] == "result" {

			let mut signature = TokenList::new();
			token += 1;
			while tokens[token] != "{" {
				signature.push(tokens[token].clone());
				token += 1;
			}

			let mut code = TokenList::new();
			let mut brackets : usize = 1; // the number of brackets that need to be closed
			loop {
				token += 1;
				if tokens[token] == "{" {brackets += 1;}
				if tokens[token] == "}" {brackets -= 1;}
				if brackets == 0 {break;}
				code.push(tokens[token].clone());
			}

			let functions = FuncParser::vec_from_tokens(code);
			program_parser.results.push(ResultParser{signature, functions});
		}

		token += 1;
	}
	program_parser
}

// the main function
//fn parse(code: String) -> Program {tokenize(code)}