
extern crate ro_backend;

#[cfg(test)]
mod tests;

use std::iter::FromIterator;

type TokenList = Vec<String>;

const OPERATORS : [&str; 14] = ["(", ":", ")", "{", "}", "=", "+", "-", "*", "/", "=>", "//", "/*", "*/"];
const KEYWORDS : [&str; 3] = ["fn", "result", "var"];

#[derive(PartialEq)]
enum TokenizerMode {
	LineComment,
	BlockComment,
	Operator,
	Normal
}

fn tokenize(code: String) -> TokenList {
	let mut tokens = TokenList::new();
	let mut current_token = String::new();
	let mut mode = TokenizerMode::Normal;
	for character in code.chars() {
		if mode == TokenizerMode::LineComment {
			if character == '\n' {
				mode = TokenizerMode::Normal;
			}
		} else if character == ' ' || character == '\n' && current_token != String::new() {
			tokens.push(current_token.clone());
			current_token = String::new();
			mode = TokenizerMode::Normal;
		} else if OPERATORS.contains(&String::from_iter(vec![character]).as_str()) && current_token != String::new() {
			tokens.push(current_token.clone());
			current_token = String::from_iter(vec![character]);
			if current_token == String::from("//") {
				mode = TokenizerMode::LineComment;
			}
			mode = TokenizerMode::Operator;
		} else if mode == TokenizerMode::Operator {
			tokens.push(current_token.clone());
			current_token = String::from_iter(vec![character]);
			mode = TokenizerMode::Normal;
		} else {
			current_token.push(character);
		}
	}
	if current_token != String::new() {
		tokens.push(current_token.clone());
	}
	tokens
}

// the main function
/*
fn parse(code: String) -> ro_backend::program::Program {

}
*/