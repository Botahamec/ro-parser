use crate::TokenList;

use std::collections::HashMap;

#[derive(Debug, Default, PartialEq)]
pub struct ResultSig {
	pub name: String,
	pub return_type: String,
	pub parameters: HashMap<String, String>,
}

pub fn parse_signature(tokens: TokenList) -> ResultSig {
	let mut token: usize = 2; // the current token number
	let mut signature = ResultSig::default();
	signature.name = tokens[0].clone();
	while tokens[token] != ")" {
		let parameter_name = tokens[token].clone();
		let parameter_type = tokens[token + 2].clone();
		signature.parameters.insert(parameter_name, parameter_type);
		token += 3;
	}
	if tokens[token] == ":" {
		signature.return_type = tokens[token + 1].clone();
	}
	signature
}
