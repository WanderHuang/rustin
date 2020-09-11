use super::parser::Ast;
use super::parser::AstTypeEnum;


pub fn generate(ast: &mut Ast) -> Ast {
  let mut next_ast = Ast::new(AstTypeEnum::Program).set_body(Vec::new());
  println!("generate => {:?}", ast);
  traverse_node(&mut next_ast, &mut Ast::new(AstTypeEnum::Null), |node: &mut Ast, parent: &mut Ast| {
    match node.ast_type {
      AstTypeEnum::CallExpression => {

        let mut expression = Ast::new(AstTypeEnum::CallExpression);
        let mut callee = Ast::new(AstTypeEnum::Identifier);
        callee.name = Some(node.name.clone().unwrap());
        expression.callee = Some(Box::new(callee));
        expression.arguments = Some(Vec::new());

        node.set_context(expression.arguments.clone().unwrap());

        if !(AstTypeEnum::CallExpression == parent.ast_type) {
          expression = Ast::new(AstTypeEnum::ExpressionStatement).set_expression(expression);
        }

        let mut ctx = parent.clone().context.unwrap();
        ctx.push(expression);
        parent.set_context(ctx);
      },
      _ => {
        if parent.clone().context.unwrap().len() <= 0 {
          parent.context = Some(Vec::new());
        }
        let mut ctx = parent.clone().context.unwrap();
        ctx.push(Ast::new(node.clone().ast_type).set_value(node.clone().value.unwrap()));
        parent.context = Some(ctx);
      },

    }
  });

  return next_ast;
}

fn traverse_node<F>(node: &mut Ast, parent: &mut Ast, visitor: F) where F: Fn(&mut Ast, &mut Ast) {
  visitor(node, parent);

  match node.ast_type {
    AstTypeEnum::Program => traverse_array(node.clone().body.unwrap(), node, &visitor),
    AstTypeEnum::CallExpression => traverse_array(node.clone().params.unwrap(), node, &visitor),
    _ => println!("traverse on node type = {:?}", node.ast_type),
  }
}

fn traverse_array<F>(array: Vec<Ast>, parent: &mut Ast, visitor: &F) where F: Fn(&mut Ast, &mut Ast) {
  for mut x in array {
    traverse_node(&mut x, parent, visitor);
  }
}