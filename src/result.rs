use crate::function::FuncParser;
use crate::TokenList;

use std::collections::HashMap;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResultParser {
    pub signature: TokenList,
    pub functions: Vec<FuncParser>,
}

#[derive(Debug, Default, PartialEq)]
pub struct ResultSig {
    pub name: String,
    pub return_type: Option<String>,
    pub parameters: HashMap<String, String>,
}

impl ResultParser {
    /**
     * Parses the signature for the result
     */
    pub fn parse_signature(&self) -> ResultSig {
        ResultSig::from_tokens(self.signature.clone())
    }
}

impl ResultSig {
    /**
     * Parses the signature for the result
     */
    pub fn from_tokens(tokens: TokenList) -> Self {
        let mut token: usize = 2; // the current token number
        let mut signature = ResultSig::default();
        signature.name = tokens[0].clone();
        while tokens[token] != ")" && token < tokens.len() - 2 {
            if tokens[token] == "," {
                token += 1;
                continue;
            }
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

    /**
     * Takes a result and parses its signature
     */
    pub fn from_result_parser(parser: ResultParser) -> Self {
        parser.parse_signature()
    }
}
