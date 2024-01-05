use regex::Regex;
use std::fmt;
use Token::{Identifier, Number, Operator};

#[derive(PartialEq, Clone)]
pub enum Token {
    Identifier(String),
    Number(f64),
    Operator(String),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Identifier(value) => write!(f, "Identifier: {}", value),
            Token::Number(value) => write!(f, "Number: {}", value),
            Token::Operator(value) => write!(f, "Operator: {}", value),
        }
    }
}

pub fn tokenize(string: &str) -> Vec<Token> {
    let mut matched_result: Vec<Token> = Vec::new();

    let token_regex_result: Result<Regex, regex::Error> = Regex::new(concat!(
        // Capture a group named 'ident' that starts with an alphabetic character and is followed by zero or more word characters, or match an empty string
        r"(?P<identifier>\p{Alphabetic}\w*)|",
        // Capture a group named 'number' that represents a numeric literal.
        // It starts with one or more digits, followed by an optional dot and zero or more additional digits, or match an empty string.
        r"(?P<number>\d+\.?\d*)|",
        // Capture a group named 'operator' that matches a single non-whitespace character.
        r"(?P<operator>\S)",
    ));

    // If regex initialization fails, program stops here
    // If regex initialization succeed then [Regex] object is store in [token_regex] variable
    let token_regex: Regex;
    match token_regex_result {
        Ok(val) => {
            token_regex = val;
        }
        Err(err) => {
            println!("{}", err);
            std::process::exit(1);
        }
    }

    let caps = token_regex.captures_iter(string);

    for cap in caps {
        let token = if cap.name("identifier").is_some() {
            Identifier(cap.name("identifier").unwrap().as_str().to_string())
        } else if cap.name("number").is_some() {
            match cap.name("number").unwrap().as_str().parse() {
                Ok(number) => Number(number),
                Err(_) => panic!("Lexer failed trying to parse number"),
            }
        } else {
            Operator(cap.name("operator").unwrap().as_str().to_string())
        };

        matched_result.push(token);
    }
    matched_result
}
