#![allow(dead_code)]

extern crate ro_backend;

mod tokenizer;
mod function;

#[cfg(test)]
mod tests;

use std::collections::LinkedList;

//use ro_backend::{Program, Function, Parameter};

use tokenizer::TokenList;
use function::FuncParser;

#[derive(PartialEq, Clone)]
enum LexMode {
	Function,
	Result
}

#[derive(Default, PartialEq, Clone)]
struct LexStack {
	array: LinkedList<LexMode>
}

impl LexStack {
	fn top(self) -> LexMode {
		self.array.front().unwrap().clone()
	}

	fn push(&mut self, mode: LexMode) {
		self.array.push_front(mode);
	}

	fn pop(&mut self) -> LexMode {
		self.array.pop_front().unwrap()
	}
}

#[derive(Default)]
struct ProgramParser {
	results: Vec<ResultParser>,
	functions: Vec<FuncParser>
}

#[derive(Default)]
struct ResultParser {
	signature: TokenList,
	functions: Vec<FuncParser>
}

fn parse_for_results_and_fns(tokens: TokenList) -> ProgramParser {
	let mut program_parser = ProgramParser::default();

	let mut token = 0;
	while token < tokens.len() {
		if tokens[token] == "fn" {

			let mut signature = TokenList::new();
			token += 1;
			while tokens[token] != "fn" {
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
	}
	program_parser
}

// TODO: remove unwraps
/*
fn lexer(tokens: TokenList) -> Program {

	let mut new_program = Program::default();

	let mut i = 0;
	while i < tokens.len() {

		let token = tokens.get(i).unwrap();

		if token == "fn" {
			let mut new_func = Function::default();
			new_func.name = String::from(tokens.get(i + 1).unwrap());
			let mut j = 0;
			i += 2;
			while *tokens.get(i + (3 * j) + 1).unwrap() != String::from(")") {
				let mut new_parameter = Parameter::default();
				new_parameter.name = tokens.get(i + (3 * j) + 1).unwrap().clone();
				new_parameter.rotype = tokens.get(i + (3 * j) + 3).unwrap().clone();
				j += 1
			}
			i += 3 * j + 1;
			if *tokens.get(i + 1).unwrap() == String::from(":") {
				new_func.return_type = tokens.get(i + 2).unwrap().to_string();
				i += 2;
			} else {i += 1;}

			// TODO: add call parsing

			new_program.functions.push(new_func);
		}

		i += 1;
	}

	new_program
}*/

// the main function
//fn parse(code: String) -> Program {tokenize(code)}