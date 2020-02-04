
use crate::TokenList;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct FuncParser {
	pub signature: TokenList,
	pub code: TokenList
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
}