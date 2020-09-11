
// pub mod token;
// pub mod parser;

mod token;
mod parser;
mod generator;
mod transformer;

use token::{Token, TokenReader};
use parser::{Ast, AstReader};
pub fn compile(input: &str) -> &str {
    let mut all = Token::read(input);
    all.reverse();
    let mut ast = Ast::transform(&mut all);

    let next_ast = transformer::generate(&mut ast);

    println!("ast ==> \n {:?}", next_ast);
    let res = generator::from(ast);
    
    println!("result ==> \n {:?}", res);

    input
}

// Ast { 
//     ast_type: Program, 
//     value: None, 
//     name: None, 
//     params: None, 
//     body: Some(
//         [Ast { 
//             ast_type: CallExpression, 
//             value: None, 
//             name: Some("add"), 
//             params: Some(
//                 [Ast { ast_type: NumberLiteral, 
//                     value: Some("123"), 
//                     name: None, 
//                     params: None, 
//                     body: None, 
//                     parent: None, 
//                     context: None, 
//                     callee: None, 
//                     arguments: None, 
//                     expresssion: None }, 
//                 Ast { ast_type: NumberLiteral, 
//                     value: Some("678"), 
//                     name: None, 
//                     params: None, 
//                     body: None, 
//                     parent: None, 
//                     context: None, 
//                     callee: None, 
//                     arguments: None, 
//                     expresssion: None }]), 
//             body: None, 
//             parent: None, 
//             context: None, 
//             callee: None, 
//             arguments: None, 
//             expresssion: None }]), 
//     parent: None, 
//     context: None, 
//     callee: None, 
//     arguments: None, 
//     expresssion: None }

