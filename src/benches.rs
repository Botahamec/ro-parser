use crate::*;
use function::*;
use result::*;
use tokenizer::*;

use std::fs::read_to_string;

extern crate test;
use test::Bencher;

#[bench]
fn tokenize_bench(b: &mut Bencher) {
    let code = read_to_string("src/benchmark.ro").unwrap();
    b.iter(|| tokenize_with_block_comments(code.clone()))
}

#[bench]
fn code_block_bench(b: &mut Bencher) {
    let code = read_to_string("src/benchmark.ro").unwrap();
    let tokens = tokenize_with_block_comments(code);
    b.iter(|| remove_block_comments(tokens.clone()))
}

#[bench]
fn parse_fns_bench(b: &mut Bencher) {
    let code = read_to_string("src/benchmark.ro").unwrap();
    let tokens = tokenize(code);
    let parse = |tokens: TokenList| -> Vec<FuncParser> { FuncParser::vec_from_tokens(tokens) };
    b.iter(|| parse(tokens.clone()))
}

#[bench]
fn parse_results_bench(b: &mut Bencher) {
    let code = read_to_string("src/benchmark.ro").unwrap();
    let tokens = tokenize(code);
    let parse = |tokens: TokenList| -> ProgramParser { parse_for_results_and_fns(tokens) };
    b.iter(|| parse(tokens.clone()))
}

#[bench]
fn parse_result_sig_bench(b: &mut Bencher) {
    let code = read_to_string("src/benchmark.ro").unwrap();
    let tokens = tokenize(code);
    let program_parse = parse_for_results_and_fns(tokens);
    let signature = program_parse.results[0].clone().signature;
    b.iter(|| ResultSig::from_tokens(signature.clone()))
}

#[bench]
fn parse_fn_sig_bench(b: &mut Bencher) {
    let string_vec =
        |vec: Vec<&str>| -> Vec<String> { vec.iter().map(|s| String::from(*s)).collect() };
    let mut func_parser = FuncParser::default();
    func_parser.signature = string_vec(vec![
        "(", "one", ":", "float", ",", "two", ":", "float", ")", ":", "float", "=>", "add",
    ]);
    b.iter(|| func_parser.clone().parse_signature())
}
