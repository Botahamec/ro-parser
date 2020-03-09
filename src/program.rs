use crate::function::FuncParser;
use crate::function::Function;
use crate::result::ResultParser;
use crate::result::RoResult;
use crate::tokenizer::TokenList;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ProgramParser {
	pub results: Vec<ResultParser>,
	pub functions: Vec<FuncParser>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Program {
	pub results: Vec<RoResult>,
	pub functions: Vec<Function>,
}

impl ProgramParser {
	pub fn from_tokens(tokens: TokenList) -> ProgramParser {
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

				program_parser
					.functions
					.push(FuncParser { signature, code })
			}

			if token > tokens.len() {
				break;
			} // prevents an error here

			if tokens[token] == "result" {
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

				let functions = FuncParser::vec_from_tokens(code);
				program_parser.results.push(ResultParser {
					signature,
					functions,
				});
			}

			token += 1;
		}
		program_parser
	}

	/** Creates a list of functions */
	pub fn parse_funcs(&self) -> Vec<Function> {
		let mut funcs = Vec::with_capacity(self.functions.len());
		for func in self.functions.clone() {
			funcs.push(func.parse());
		}
		funcs
	}

	/** Creates a list of results */
	// TODO: put some functions into results
	pub fn parse_results(&self) -> Vec<RoResult> {
		let mut results = Vec::with_capacity(self.results.len());
		for result in self.results.clone() {
			results.push(result.parse());
		}
		results
	}

	/** Creates a Program */
	pub fn parse(&self) -> Program {
		Program {
			functions: self.parse_funcs(),
			results: self.parse_results(),
		}
	}
}
