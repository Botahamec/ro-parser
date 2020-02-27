use std::iter::FromIterator;

// a list of valid operators
pub const OPERATORS: [&str; 17] = [
	"(", ":", ",", ".", ")", "{", "}", ">", "=", "+", "-", "*", "/", "=>",
	"//", "/*", "*/",
];

// a list of characters which are considered whitespace
const WHITESPACE: [char; 4] = [' ', '\n', '\t', '\r'];

// tells the tokenizer what to expect next
#[derive(PartialEq)]
enum TokenizerMode {
	LineComment,
	Operator,
	Normal,
}

pub type TokenList = Vec<String>;

/**
 * Converts Ro code into a list of tokens
 * Does not remove block comments
 *
 * @author  Mike White
 * @param   code the code as a String
 */
pub fn tokenize_with_block_comments(code: String) -> TokenList {
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
		} else if OPERATORS
			.contains(&String::from_iter(vec![character]).as_str())
		{
			// runs if the character combined with the rest of the current token is an operator
			if OPERATORS.contains(
				&(current_token.clone() + &String::from_iter(vec![character]))
					.as_str(),
			) {
				current_token.push(character.clone());

				// ignores the rest of the line if there's a line comment
				if current_token == "//" {
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
		tokens.push(current_token);
	}
	tokens
}

pub fn remove_block_comments(tokens: TokenList) -> TokenList {
	let mut in_comment = false;
	let mut new_list = TokenList::new();

	for token in tokens {
		if token == "/*" {
			in_comment = true;
		} else if token == "*/" {
			in_comment = false;
		} else if !in_comment {
			new_list.push(token);
		}
	}

	new_list
}

pub fn tokenize(code: String) -> TokenList {
	remove_block_comments(tokenize_with_block_comments(code))
}
