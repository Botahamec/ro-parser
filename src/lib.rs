#![allow(dead_code)]

extern crate ro_backend;

#[cfg(test)]
mod tests;

use std::iter::FromIterator;
use std::collections::LinkedList;

use ro_backend::{Program, Function, Parameter, RoResult};

type TokenList = Vec<String>;

// tells the tokenizer what to expect next
#[derive(PartialEq)]
enum TokenizerMode {
	LineComment,
	BlockComment,
	Operator,
	Normal
}

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

#[derive(Default, Debug, PartialEq)]
struct FuncParser {
	signature: TokenList,
	code: TokenList
}

#[derive(Default)]
struct ResultParser {
	signature: TokenList,
	functions: Vec<FuncParser>
}

// a list of valid operators
const OPERATORS : [&str; 17] = ["(", ":", ",", ".", ")", "{", "}", ">", "=", "+", "-", "*", "/", "=>", "//", "/*", "*/"];

// a list of keywords
const KEYWORDS : [&str; 4] = ["fn", "result", "var", "return"];

// a list of characters which are considered whitespace
const WHITESPACE : [char; 4] = [' ', '\n', '\t', '\r'];

/**
 * Converts Ro code into a list of tokens
 *
 * @author	Mike White
 * @param	code the code as a String
 */
fn tokenize(code: String) -> TokenList {

	let mut tokens = TokenList::new(); // what will be returned
	let mut current_token = String::new(); // the token currently being parsed
	let mut mode = TokenizerMode::Normal; // the mode that tells the tokenizer what to expect

	// check each character in the String
	for character in code.chars() {

		// skip over the rest of the line if there's a line comment
		if mode == TokenizerMode::LineComment {
			if character == '\n' {
				mode = TokenizerMode::Normal;
			}

		// end tokens at whitespace
		} else if WHITESPACE.contains(&character) {
			if current_token != String::new() {
				tokens.push(current_token.clone());
			}
			current_token = String::new();
			mode = TokenizerMode::Normal;

		// runs if the character is an operator
		} else if OPERATORS.contains(&String::from_iter(vec![character]).as_str()) {

			// runs if the character combined with the rest of the current token is an operator
			if OPERATORS.contains(&(current_token.clone() + &String::from_iter(vec![character])).as_str()) {
				current_token.push(character.clone());

				// ignores the rest of the line if there's a line comment
				if current_token == String::from("//") {
					current_token = String::new();
					mode = TokenizerMode::LineComment;

				// otherwise makes sure the tokenizer expects an operator
				} else {
					mode = TokenizerMode::Operator;
				}

			// runs if it's now two operators
			} else {
				tokens.push(current_token.clone());
				current_token = String::from_iter(vec![character]);
				mode = TokenizerMode::Operator;
			}

		// ends the operator token if it was expecting more operators
		} else if mode == TokenizerMode::Operator {
			tokens.push(current_token.clone());
			current_token = String::from_iter(vec![character]);
			mode = TokenizerMode::Normal;

		// otherwise just add the character to the token
		} else {
			current_token.push(character);
		}
	}
	if current_token != String::new() {
		tokens.push(current_token.clone());
	}
	tokens
}

fn remove_block_comments(tokens: TokenList) -> TokenList {
	let mut in_comment = false;
	let mut new_list = TokenList::new();

	for token in tokens {
		if token == String::from("/*") {
			in_comment = true;
		} else if token == String::from("*/") {
			in_comment = false;
		} else if !in_comment {
			new_list.push(token);
		}
	}

	new_list
}

fn parse_for_fns(tokens: TokenList) -> Vec<FuncParser> {

	let mut funcs = Vec::new();

	let mut token = 0;
	while token < tokens.len() {
		if tokens[token] == String::from("fn") {

			let mut signature = TokenList::new();
			token += 1;
			while tokens[token] != String::from("{") {
				signature.push(tokens[token].clone());
				token += 1;
			}

			let mut code = TokenList::new();
			let mut brackets : usize = 1; // the number of brackets that need to be closed
			loop {
				token += 1;
				if tokens[token] == String::from("{") {brackets += 1;}
				if tokens[token] == String::from("}") {brackets -= 1;}
				if brackets == 0 {break;}
				code.push(tokens[token].clone());
			}

			funcs.push(FuncParser {signature, code})
		}
		token += 1;
	}

	funcs
}

fn parse_for_results_and_fns(tokens: TokenList) -> ProgramParser {
	let mut program_parser = ProgramParser::default();

	let mut token = 0;
	while token < tokens.len() {
		if tokens[token] == String::from("fn") {

			let mut signature = TokenList::new();
			token += 1;
			while tokens[token] != String::from("{") {
				signature.push(tokens[token].clone());
				token += 1;
			}

			let mut code = TokenList::new();
			let mut brackets : usize = 1; // the number of brackets that need to be closed
			loop {
				token += 1;
				if tokens[token] == String::from("{") {brackets += 1;}
				if tokens[token] == String::from("}") {brackets -= 1;}
				if brackets == 0 {break;}
				code.push(tokens[token].clone());
			}

			program_parser.functions.push(FuncParser {signature, code})
		}

		if tokens[token] == String::from("result") {

			let mut signature = TokenList::new();
			token += 1;
			while tokens[token] != String::from("{") {
				signature.push(tokens[token].clone());
				token += 1;
			}

			let mut code = TokenList::new();
			let mut brackets : usize = 1; // the number of brackets that need to be closed
			loop {
				token += 1;
				if tokens[token] == String::from("{") {brackets += 1;}
				if tokens[token] == String::from("}") {brackets -= 1;}
				if brackets == 0 {break;}
				code.push(tokens[token].clone());
			}

			let functions = parse_for_fns(code);
			program_parser.results.push(ResultParser{signature, functions});
		}
	}
	program_parser
}

// TODO: remove unwraps
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
}

// the main function
fn parse(code: String) -> Program {lexer(remove_block_comments(tokenize(code)))}