mod token;
mod parser;
mod generator;
mod transformer;

use token::{Token, TokenReader};
use parser::{Ast, AstReader};
pub fn compile(input: &str) -> String {
    let mut all = Token::read(input);
    all.reverse();
    let mut ast = Ast::transform(&mut all);

    let next_ast = transformer::generate(&mut ast);

    println!("ast ==> \n {:?}", next_ast);
    let res = generator::from(ast);
    
    res.unwrap_or("x".to_string()).to_string()
}

