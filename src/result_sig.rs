
use crate::TokenList;

use std::collections::HashMap;

#[derive(Debug, Default, PartialEq)]
pub struct ResultSig {
	pub name: String,
	pub return_type: Option<String>,
	pub parameters: HashMap<String, String>
}

pub fn parse_signature(tokens: TokenList) -> ResultSig {

	let mut token : usize = 2; // the current token number
	let mut signature = ResultSig::default();
	signature.name = tokens[0].clone();
	while tokens[token] != ")" && token < tokens.len() - 2 {
		if tokens[token] == "," {token += 1; continue;}
		let parameter_name = tokens[token].clone();
		let parameter_type = tokens[token + 2].clone();
		signature.parameters.insert(parameter_name, parameter_type);
		token += 3;
	}
	if tokens.len() >= 2 && token < tokens.len() - 2 && tokens[token + 1] == ":" {
		signature.return_type = Some(tokens[token + 2].clone());
	} else {
		signature.return_type = None;
	}
	signature
}