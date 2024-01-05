mod lexer;

use lexer::Token;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let user_string: &String = &args[1];

    let token: Vec<Token> = lexer::tokenize(user_string);

    for token in token {
        println!("{}", token);
    }
}
