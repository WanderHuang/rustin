use super::parser::Ast;
use super::parser::AstTypeEnum;

pub fn generate(ast: &mut Ast) -> Ast {
    let mut next_ast = ast.clone();
    next_ast = next_ast.set_context(Vec::new());
    traverse_node(&mut next_ast, &mut Ast::new(AstTypeEnum::Null));

    return next_ast;
}

fn traverse_node(node: &mut Ast, parent: &mut Ast) {
    println!("[ Visit ] {:?}", node);
    match node.ast_type {
        AstTypeEnum::CallExpression => {
            let mut expression = Ast::new(AstTypeEnum::CallExpression);
            let mut callee = Ast::new(AstTypeEnum::Identifier);
            callee.name = Some(node.name.as_ref().unwrap().to_string());
            expression.callee = Some(Box::new(callee));
            expression.arguments = Some(Vec::new());

            if let Some(args) = &mut expression.arguments {
              node.context = Some(args.to_vec());
            }


            if !(AstTypeEnum::CallExpression == parent.ast_type) {
                expression = Ast::new(AstTypeEnum::ExpressionStatement).set_expression(expression);
            }

            if let Some(ctx) = &mut parent.context {
              ctx.push(expression);
              parent.context = Some(ctx.to_vec());
            }
            println!("[ transform ] expression > {:?}", parent.context);
        }
        _ => {
            let mut ctx = Vec::new();
            if let Some(x) = &mut parent.context {
                ctx = x.to_vec();
            }

            if let Some(x) = &mut node.value {
                ctx.push(Ast::new(node.ast_type.clone()).set_value(x.to_owned()));
            }
            parent.context = Some(ctx);
        }
    }

    match node.ast_type {
        AstTypeEnum::Program => {
            if let Some(vector) = &node.body {
                for mut x in vector.to_owned() {
                    traverse_node(&mut x, node);
                }
            }
        }
        AstTypeEnum::CallExpression => {
            if let Some(vector) = &node.params {
                for mut x in vector.to_owned() {
                    traverse_node(&mut x, node);
                }
            }
        }
        _ => println!("traverse on node type = {:?}", node.ast_type),
    }
}
