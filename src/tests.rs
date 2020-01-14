
use crate::*;

#[test]
fn tokenize_empty_function() {
	let code = String::from("fn main(args: float) {}");
	assert_eq!(tokenize(code), vec!["fn", "main", "(", "args", ":", "float", ")", "{", "}"]);
}