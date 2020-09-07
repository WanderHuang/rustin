
pub mod token;
pub mod parser;

use token::{Token, TokenReader};
pub fn compile(input: &str) -> &str {
    let all = Token::read(input);
    for t in all {
        Token::print(t);
    }

    input
}

