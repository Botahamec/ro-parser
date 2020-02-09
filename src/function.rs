
use crate::TokenList;

use std::collections::HashMap;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct FuncParser {
	pub signature: TokenList,
	pub code: TokenList
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct FuncSig {
	pub name: Option<String>,
	pub parameters: Option<HashMap<String, String>>,
	pub return_type: Option<String>,
	pub result: Option<String>
}

impl FuncParser {

	pub fn vec_from_tokens(tokens: TokenList) -> Vec<FuncParser> {
		let mut funcs = Vec::new();

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

				funcs.push(FuncParser {signature, code})
			}
			token += 1;
		}

		funcs
	}

	pub fn vec_from_string(code: String) -> Vec<FuncParser> {
		let tokens = crate::tokenizer::tokenize(code);
		Self::vec_from_tokens(tokens)
	}

	pub fn parse_signature(self) -> FuncSig {

		let tokens = self.signature;
		let mut token : usize = 0;
		let mut signature = FuncSig::default();

		if tokens[token] != "(" && tokens[token] != ":" && tokens[token] != "=>" {
			signature.name = Some(tokens[token].clone());
			token += 1;
		} else {
			signature.name = None;
		}

		if tokens[token] == "(" {
			token += 1;
			signature.parameters = Some(HashMap::new());
			while tokens[token] != ")" {
				let parameter_name = tokens[token].clone();
				let parameter_type = tokens[token + 2].clone();
				signature.parameters.as_mut().unwrap().insert(parameter_name, parameter_type);
				token += 3;
			}
		} else {
			signature.parameters = None;
		}

		if tokens[token] == ":" {
			token += 2;
			signature.return_type = Some(tokens[token].clone());
		} else {
			signature.return_type = None;
		}

		if tokens[token] == "=>" {
			token += 2;
			signature.result = Some(tokens[token].clone());
		} else {
			signature.result = None;
		}

		signature
	}
}