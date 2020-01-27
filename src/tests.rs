
use crate::*;

#[test]
fn tokenize_test() {

	let mut code = String::from("fn main() {\n\t// comment\n\tprint(add(1, 2).to_string())\n}");
	let mut tokens = vec!["fn", "main", "(", ")", "{", "print", "(", "add", "(", "1", ",", "2", ")", ".", "to_string", "(", ")", ")", "}"];
	assert_eq!(tokenize(code), tokens);

	code = String::from("result sort(list : float): float");
	tokens = vec!["result", "sort", "(", "list", ":", "float", ")", ":", "float"];
	assert_eq!(tokenize(code), tokens);

	code = String::from("fn addtwo (one:float, two: float): float {\n\treturn one+two\n}");
	tokens = vec!["fn", "addtwo", "(", "one", ":", "float", ",", "two", ":", "float", ")", ":", "float", "{", "return", "one", "+", "two", "}"];
	assert_eq!(tokenize(code), tokens);

	code = String::from("fn => Add {\n\tvar one'=one+1-1\n\treturn one' + two}");
	tokens = vec!["fn", "=>", "Add", "{", "var", "one'", "=", "one", "+", "1", "-", "1", "return", "one'", "+", "two", "}"];
	assert_eq!(tokenize(code), tokens);

	code = String::from("fn {\n\t //* a comment that has been toggled off\n\treturn one + two\n\t*/\n}");
	tokens = vec!["fn", "{", "return", "one", "+", "two", "*/", "}"];
	assert_eq!(tokenize(code), tokens);

	code = String::from("fn add1 {\n\t/// docstring\n\tvar temp :float = one +two\n\treturn temp\n}");
	tokens = vec!["fn", "add1", "{", "var", "temp", ":", "float", "=", "one", "+", "two", "return", "temp", "}"];
	assert_eq!(tokenize(code), tokens);
}

#[test]
fn code_block_test() {

	// closure to shorten certain code
	let test = |code| remove_block_comments(tokenize(code));

	let mut code = String::from("fn main() {\n\t/* comment */\n\tprint(add(1, 2).to_string())\n}");
	let mut tokens = vec!["fn", "main", "(", ")", "{", "print", "(", "add", "(", "1", ",", "2", ")", ".", "to_string", "(", ")", ")", "}"];
	assert_eq!(test(code), tokens);

	code = String::from("result sort(list /*:*/ float): float");
	tokens = vec!["result", "sort", "(", "list", "float", ")", ":", "float"];
	assert_eq!(test(code), tokens);

	code = String::from("fn addtwo (one:floa/*t*/, two: float): float {\n\treturn one+two\n}");
	tokens = vec!["fn", "addtwo", "(", "one", ":", "floa", ",", "two", ":", "float", ")", ":", "float", "{", "return", "one", "+", "two", "}"];
	assert_eq!(test(code), tokens);

	code = String::from("fn {\n\t //* a comment that has been toggled off\n\treturn one + two\n\t*/\n}");
	tokens = vec!["fn", "{", "return", "one", "+", "two", "}"];
	assert_eq!(test(code), tokens);

	code = String::from("fn {\n\t /* a comment that has been toggled on\n\treturn one + two\n\t*/\n}");
	tokens = vec!["fn", "{", "}"];
	assert_eq!(test(code), tokens);
}

#[test]
fn parse_fns_test() {

	let string_vec = |vec: Vec<&str>| -> Vec<String> {vec.iter().map(|s| String::from(*s)).collect()};

	let mut tokens = string_vec(vec!["fn", "main", "(", ")", "{", "println", "(", ")", "}"]);
	let mut prototype = string_vec(vec!["main", "(", ")"]);
	let mut code = string_vec(vec!["println", "(", ")"]);
	assert_eq!(parse_for_fns(tokens)[0], FuncParser{prototype, code});

	tokens = string_vec(vec!["fn", "temp_convert", "(", "farenheit", ":", "isize", ")", "{", "var", "temp1", "=", "farenheit", "-", "32", "var", "temp2", "=", "temp1", "*", "5", "var", "temp3", "=", "temp2", "/", "9", "return", "temp3", "}"]);
	prototype = string_vec(vec!["temp_convert", "(", "farenheit", ":", "isize", ")"]);
	code = string_vec(vec!["var", "temp1", "=", "farenheit", "-", "32", "var", "temp2", "=", "temp1", "*", "5", "var", "temp3", "=", "temp2", "/", "9", "return", "temp3"]);
	assert_eq!(parse_for_fns(tokens)[0], FuncParser{prototype, code});

	tokens = string_vec(vec!["fn", "main", "(", ")", "{", "println", "(", ")", "}", "fn", "temp_convert", "(", "farenheit", ":", "isize", ")", "{", "var", "temp1", "=", "farenheit", "-", "32", "var", "temp2", "=", "temp1", "*", "5", "var", "temp3", "=", "temp2", "/", "9", "return", "temp3", "}"]);
	let fns = parse_for_fns(tokens);
	assert_eq!(fns.len(), 2);

	prototype = string_vec(vec!["main", "(", ")"]);
	code = string_vec(vec!["println", "(", ")"]);
	assert_eq!(fns[0], FuncParser{prototype, code});

	prototype = string_vec(vec!["temp_convert", "(", "farenheit", ":", "isize", ")"]);
	code = string_vec(vec!["var", "temp1", "=", "farenheit", "-", "32", "var", "temp2", "=", "temp1", "*", "5", "var", "temp3", "=", "temp2", "/", "9", "return", "temp3"]);
	assert_eq!(fns[1], FuncParser{prototype, code});

}