
use crate::*;
use tokenizer::*;
use function::*;
use result_sig::*;

use std::collections::HashMap;

#[test]
fn tokenize_test() {

	// test a simple function
	let mut code = String::from("fn main() {\n\t// comment\n\tprint(add(1, 2).to_string())\n}");
	let mut tokens = vec!["fn", "main", "(", ")", "{", "print", "(", "add", "(", "1", ",", "2", ")", ".", "to_string", "(", ")", ")", "}"];
	assert_eq!(tokenize_with_block_comments(code), tokens);

	// test a function signature
	code = String::from("result sort(list : float): float");
	tokens = vec!["result", "sort", "(", "list", ":", "float", ")", ":", "float"];
	assert_eq!(tokenize_with_block_comments(code), tokens);

	// test operators when there are no spaces
	code = String::from("fn addtwo (one:float, two: float): float {\n\treturn one+two\n}");
	tokens = vec!["fn", "addtwo", "(", "one", ":", "float", ",", "two", ":", "float", ")", ":", "float", "{", "return", "one", "+", "two", "}"];
	assert_eq!(tokenize_with_block_comments(code), tokens);

	// test operations with no spaces
	code = String::from("fn => Add {\n\tvar one'=one+1-1\n\treturn one' + two}");
	tokens = vec!["fn", "=>", "Add", "{", "var", "one'", "=", "one", "+", "1", "-", "1", "return", "one'", "+", "two", "}"];
	assert_eq!(tokenize_with_block_comments(code), tokens);

	// test toggleable comment blocks
	code = String::from("fn {\n\t //* a comment that has been toggled off\n\treturn one + two\n\t*/\n}");
	tokens = vec!["fn", "{", "return", "one", "+", "two", "*/", "}"];
	assert_eq!(tokenize_with_block_comments(code), tokens);

	// teat weird spacing
	code = String::from("fn add1 {\n\t/// docstring\n\tvar temp :float = one +two\n\treturn temp\n}");
	tokens = vec!["fn", "add1", "{", "var", "temp", ":", "float", "=", "one", "+", "two", "return", "temp", "}"];
	assert_eq!(tokenize_with_block_comments(code), tokens);
}

#[test]
fn code_block_test() {

	// test a simple block
	let mut code = String::from("fn main() {\n\t/* comment */\n\tprint(add(1, 2).to_string())\n}");
	let mut tokens = vec!["fn", "main", "(", ")", "{", "print", "(", "add", "(", "1", ",", "2", ")", ".", "to_string", "(", ")", ")", "}"];
	assert_eq!(tokenize(code), tokens);

	// test a block which removes operators
	code = String::from("result sort(list /*:*/ float): float");
	tokens = vec!["result", "sort", "(", "list", "float", ")", ":", "float"];
	assert_eq!(tokenize(code), tokens);

	// test comment blocks at the end of a word
	code = String::from("fn addtwo (one:floa/*t*/, two: float): float {\n\treturn one+two\n}");
	tokens = vec!["fn", "addtwo", "(", "one", ":", "floa", ",", "two", ":", "float", ")", ":", "float", "{", "return", "one", "+", "two", "}"];
	assert_eq!(tokenize(code), tokens);

	// test toggle off blocks
	code = String::from("fn {\n\t //* a comment that has been toggled off\n\treturn one + two\n\t*/\n}");
	tokens = vec!["fn", "{", "return", "one", "+", "two", "}"];
	assert_eq!(tokenize(code), tokens);

	// test toggled on blocks
	code = String::from("fn {\n\t /* a comment that has been toggled on\n\treturn one + two\n\t*/\n}");
	tokens = vec!["fn", "{", "}"];
	assert_eq!(tokenize(code), tokens);
}

#[test]
fn parse_fns_test() {

	let string_vec = |vec: Vec<&str>| -> Vec<String> {vec.iter().map(|s| String::from(*s)).collect()};
	let parse = |tokens: TokenList| -> Vec<FuncParser> {FuncParser::vec_from_tokens(tokens)};

	// a simple function
	let mut tokens = string_vec(vec!["fn", "main", "(", ")", "{", "println", "(", ")", "}"]);
	let mut signature = string_vec(vec!["main", "(", ")"]);
	let mut code = string_vec(vec!["println", "(", ")"]);
	assert_eq!(parse(tokens)[0], FuncParser{signature, code});

	// a compilcated function
	tokens = string_vec(vec!["fn", "temp_convert", "(", "farenheit", ":", "isize", ")", "{", "var", "temp1", "=", "farenheit", "-", "32", "var", "temp2", "=", "temp1", "*", "5", "var", "temp3", "=", "temp2", "/", "9", "return", "temp3", "}"]);
	signature = string_vec(vec!["temp_convert", "(", "farenheit", ":", "isize", ")"]);
	code = string_vec(vec!["var", "temp1", "=", "farenheit", "-", "32", "var", "temp2", "=", "temp1", "*", "5", "var", "temp3", "=", "temp2", "/", "9", "return", "temp3"]);
	assert_eq!(parse(tokens)[0], FuncParser{signature, code});

	// multiple functions
	tokens = string_vec(vec!["fn", "main", "(", ")", "{", "println", "(", ")", "}", "fn", "temp_convert", "(", "farenheit", ":", "isize", ")", "{", "var", "temp1", "=", "farenheit", "-", "32", "var", "temp2", "=", "temp1", "*", "5", "var", "temp3", "=", "temp2", "/", "9", "return", "temp3", "}"]);
	let fns = parse(tokens);
	assert_eq!(fns.len(), 2);

	signature = string_vec(vec!["main", "(", ")"]);
	code = string_vec(vec!["println", "(", ")"]);
	assert_eq!(fns[0], FuncParser{signature, code});

	signature = string_vec(vec!["temp_convert", "(", "farenheit", ":", "isize", ")"]);
	code = string_vec(vec!["var", "temp1", "=", "farenheit", "-", "32", "var", "temp2", "=", "temp1", "*", "5", "var", "temp3", "=", "temp2", "/", "9", "return", "temp3"]);
	assert_eq!(fns[1], FuncParser{signature, code});

}

#[test]
fn parse_results_test() {

	let string_vec = |vec: Vec<&str>| -> Vec<String> {vec.iter().map(|s| String::from(*s)).collect()};

	// a basic result
	let mut tokens = string_vec(vec!["result", "add", "(", "one", ":", "float", ",", "two", ":", "float", ")", "{", "}"]);
	let mut signature = string_vec(vec!["add", "(", "one", ":", "float", ",", "two", ":", "float", ")"]);
	let mut functions : Vec<FuncParser> = vec![];
	assert_eq!(parse_for_results_and_fns(tokens), ProgramParser{results: vec![ResultParser {signature, functions: functions.clone()}], functions});

	// a result containing a function
	tokens = string_vec(vec!["result", "add", "(", "one", ":", "float", ",", "two", ":", "float", ")", "{", "fn", "{", "return", "one", "+", "two", "}", "}"]);
	signature = string_vec(vec!["add", "(", "one", ":", "float", ",", "two", ":", "float", ")"]);
	functions = vec![FuncParser {signature: vec![], code: string_vec(vec!["return", "one", "+", "two"])}];
	assert_eq!(parse_for_results_and_fns(tokens), ProgramParser{results: vec![ResultParser {signature, functions: functions.clone()}], functions: vec![]});

	// a result containing two functions
	tokens = string_vec(vec!["result", "add", "(", "one", ":", "float", ",", "two", ":", "float", ")", "{", "fn", "{", "return", "one", "+", "two", "}", "fn", "{", "return", "one", "+", "two", "}", "}"]);
	signature = string_vec(vec!["add", "(", "one", ":", "float", ",", "two", ":", "float", ")"]);
	let mut function = FuncParser {signature: vec![], code: string_vec(vec!["return", "one", "+", "two"])};
	functions = vec![function.clone(), function.clone()];
	assert_eq!(parse_for_results_and_fns(tokens), ProgramParser{results: vec![ResultParser {signature, functions: functions.clone()}], functions: vec![]});

	// a function outside of the result
	tokens = string_vec(vec!["result", "add", "(", "one", ":", "float", ",", "two", ":", "float", ")", "{", "fn", "{", "return", "one", "+", "two", "}", "}", "fn", "=>", "add", "{", "return", "one", "+", "two", "}"]);
	signature = string_vec(vec!["add", "(", "one", ":", "float", ",", "two", ":", "float", ")"]);
	function = FuncParser {signature: vec![], code: string_vec(vec!["return", "one", "+", "two"])};
	functions = vec![function];
	let function1 = FuncParser {signature: string_vec(vec!["=>", "add"]), code: string_vec(vec!["return", "one", "+", "two"])};
	assert_eq!(parse_for_results_and_fns(tokens), ProgramParser{results: vec![ResultParser {signature, functions: functions.clone()}], functions: vec![function1]});
}

#[test]
fn parse_result_sig_test() {

	let string_vec = |vec: Vec<&str>| -> Vec<String> {vec.iter().map(|s| String::from(*s)).collect()};

	// complicated signature
	let mut signature = string_vec(vec!["add", "(", "one", ":", "float", ",", "two", ":", "float", ")", ":", "float"]);
	let mut parameters = HashMap::new();
	let rt = Some(String::from("float"));
	let name = String::from("add");
	parameters.insert(String::from("one"), String::from("float"));
	parameters.insert(String::from("two"), String::from("float"));
	assert_eq!(parse_signature(signature), ResultSig{name: name.clone(), return_type: rt.clone(), parameters: parameters.clone()});

	// simple signature
	signature = string_vec(vec!["add", "(", ")"]);
	parameters.clear();
	assert_eq!(parse_signature(signature), ResultSig{name: name.clone(), return_type: None, parameters: parameters.clone()});

	// return signature
	signature = string_vec(vec!["add", "(", ")", ":", "float"]);
	assert_eq!(parse_signature(signature), ResultSig{name: name.clone(), return_type: rt.clone(), parameters: parameters.clone()});

	// one parameter with an unnecessary comma
	signature = string_vec(vec!["add", "(", "one", ":", "float", ",", ")"]);
	parameters.insert(String::from("one"), String::from("float"));
	assert_eq!(parse_signature(signature), ResultSig{name: name.clone(), return_type: None, parameters: parameters.clone()});
}

#[test]