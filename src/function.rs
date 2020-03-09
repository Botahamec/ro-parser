use crate::tokenizer::TokenList;

use std::collections::HashMap;

pub type CallList = Vec<CallType>;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct FuncParser {
	pub signature: TokenList,
	pub code: TokenList,
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct FuncSig {
	pub name: Option<String>,
	pub parameters: Option<HashMap<String, String>>,
	pub return_type: Option<String>,
	pub result: Option<String>,
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Function {
	signature: FuncSig,
	calls: CallList,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Operation {
	Add,
	Sub,
	Mult,
	Div,
	Mod,
}

impl Operation {
	fn from_str(s: String) -> std::io::Result<Self> {
		if s == "+" {
			Ok(Operation::Add)
		} else if s == "-" {
			Ok(Operation::Sub)
		} else if s == "*" {
			Ok(Operation::Mult)
		} else if s == "/" {
			Ok(Operation::Div)
		} else if s == "%" {
			Ok(Operation::Mod)
		} else {
			Err(std::io::Error::new(
				std::io::ErrorKind::InvalidData,
				"invalid op",
			))
		}
	}
}

#[derive(Clone, Debug, PartialEq)]
pub enum CallType {
	Return(String),
	Init(String),
	Set(String, TokenList),
	Call(String, Vec<String>),
	Move(String, String),
	Operate(String, String, Operation, String),
}

impl CallType {
	/** Creates a list of calls from a token list */
	pub fn vec_from_tokens(tokens: TokenList) -> Vec<CallType> {
		let mut token: usize = 0;
		let mut calls: Vec<CallType> = Vec::new();
		while token < tokens.len() {
			if tokens[token] == "ret" {
				token += 1;
				calls.push(CallType::Return(tokens[token].clone()));
			} else if tokens[token] == "var" {
				token += 1;
				let var_name = tokens[token].clone();
				calls.push(CallType::Init(var_name.clone()));
				if tokens.len() > token + 1 && tokens[token + 1] == "=" {
					token += 2;
					let mut set: TokenList = Vec::with_capacity(1);
					set.push(tokens[token].clone());
					/*if tokens.len() > token + 1
						&& crate::tokenizer::OPERATORS
							.contains(&tokens[token + 1].as_str())
					{
						token += 1;
						set.push(tokens[token].clone());
					}*/
					while tokens.len() > token + 1
						&& crate::tokenizer::OPERATORS
							.contains(&tokens[token + 1].as_str())
					{
						token += 1;
						if tokens.len() > token
							&& crate::tokenizer::OPERATORS
								.contains(&tokens[token].as_str())
						{
							//token += 1;
							set.push(tokens[token].clone());
							token += 1;
							set.push(tokens[token].clone());
						}

						/*if tokens.len() > token + 1 {
							token += 1;
							set.push(tokens[token].clone());
						}*/
					}
					calls.push(CallType::Set(var_name, set));
				}
			} else if tokens.len() > token + 1 && tokens[token + 1] == "=" {
				let var_name = tokens[token].clone();
				token += 2;
				let mut set: TokenList = Vec::with_capacity(1);
				set.push(tokens[token].clone());
				/*if tokens.len() > token + 1
					&& crate::tokenizer::OPERATORS
						.contains(&tokens[token + 1].as_str())
				{
					token += 1;
					set.push(tokens[token].clone());
				}*/
				while tokens.len() > token + 1
					&& crate::tokenizer::OPERATORS
						.contains(&tokens[token + 1].as_str())
				{
					token += 1;
					if tokens.len() > token
						&& crate::tokenizer::OPERATORS
							.contains(&tokens[token].as_str())
					{
						//token += 1;
						set.push(tokens[token].clone());
						token += 1;
						set.push(tokens[token].clone());
					}

					/*if tokens.len() > token + 1 {
						token += 1;
						set.push(tokens[token].clone());
					}*/
				}
				calls.push(CallType::Set(var_name, set));
			} else if tokens.len() > 1 && tokens[token + 1] == "(" {
				let func_name = tokens[token].clone();
				token += 2;
				let mut parameters: Vec<String> = vec![];
				while tokens.len() > token && tokens[token] != ")" {
					parameters.push(tokens[token].clone());
					token += 1;
					if tokens[token] == "," {
						token += 1;
					}
				}

				calls.push(CallType::Call(func_name, parameters));
			}

			token += 1;
		}

		calls
	}

	/**
	 * Converts Set calls to Operate and Move calls
	 */
	pub fn sets_to_ops(calls: CallList) -> CallList {
		let mut new_calls: CallList = vec![];

		for call in calls.clone() {
			if let CallType::Set(var, tokens) = call {
				if tokens.len() == 1 {
					new_calls.push(CallType::Move(var, tokens[0].clone()));
				} else if tokens.len() == 3 {
					new_calls.push(CallType::Operate(
						var,
						tokens[0].clone(),
						Operation::from_str(tokens[1].clone()).unwrap(),
						tokens[2].clone(),
					));
				}
			}
		}

		new_calls
	}
}

impl FuncParser {
	/** Creates a function parser from a tokenlist */
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
				let mut brackets: usize = 1; // the number of brackets that need to be closed
				loop {
					token += 1;
					if tokens[token] == "{" {
						brackets += 1;
					}
					if tokens[token] == "}" {
						brackets -= 1;
					}
					if brackets == 0 {
						break;
					}
					code.push(tokens[token].clone());
				}

				funcs.push(FuncParser { signature, code })
			}
			token += 1;
		}

		funcs
	}

	pub fn vec_from_string(code: String) -> Vec<FuncParser> {
		let tokens = crate::tokenizer::tokenize(code);
		Self::vec_from_tokens(tokens)
	}

	pub fn parse_signature(&self) -> FuncSig {
		FuncSig::from_tokens(self.signature.clone())
	}

	pub fn parse_calls(&self) -> CallList {
		CallType::sets_to_ops(CallType::vec_from_tokens(self.code.clone()))
	}

	pub fn parse(self) -> Function {
		Function::from_parser(self)
	}
}

impl FuncSig {
	/**
	 * Parses a set of tokens into a signature for a function
	 */
	pub fn from_tokens(tokens: TokenList) -> Self {
		let mut token: usize = 0;
		let mut signature = FuncSig::default();

		if token < tokens.len()
			&& tokens[token] != "("
			&& tokens[token] != ":"
			&& tokens[token] != "=>"
		{
			signature.name = Some(tokens[token].clone());
			token += 1;
		} else {
			signature.name = None;
		}

		if token < tokens.len() && tokens[token] == "(" {
			token += 1;
			signature.parameters = Some(HashMap::new());
			while token < tokens.len() - 3 && tokens[token] != ")" {
				if tokens[token] == "," {
					token += 1;
					continue;
				}
				let parameter_name = tokens[token].clone();
				let parameter_type = tokens[token + 2].clone();
				signature
					.parameters
					.as_mut()
					.unwrap()
					.insert(parameter_name, parameter_type);
				token += 3;
			}
			while tokens[token] != ")" {
				token += 1;
			}
			token += 1;
		} else {
			signature.parameters = None;
		}

		if !tokens.is_empty()
			&& token < tokens.len() - 1
			&& tokens[token] == ":"
		{
			token += 1;
			signature.return_type = Some(tokens[token].clone());
			token += 1;
		} else {
			signature.return_type = None;
		}

		if !tokens.is_empty()
			&& token < tokens.len() - 1
			&& tokens[token] == "=>"
		{
			token += 1;
			signature.result = Some(tokens[token].clone());
		} else {
			signature.result = None;
		}

		signature
	}

	pub fn from_func_parser(parser: FuncParser) -> Self {
		parser.parse_signature()
	}
}

impl Function {
	/** creates a new function */
	pub fn new(signature: FuncSig, calls: CallList) -> Self {
		Function { signature, calls }
	}

	/** converts a FuncParser to a Function */
	pub fn from_parser(parser: FuncParser) -> Self {
		let signature = parser.parse_signature();
		let calls = parser.parse_calls();
		Function { signature, calls }
	}
}
