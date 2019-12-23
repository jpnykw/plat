use inkwell::context::Context;
use super::ast;

pub fn generate(
    root: ast::Root
) -> Option<ast::Types> {
    let context = Context::create();
    let module = context.create_module("main");
    let builder = context.create_builder();
    let i32_type = context.i32_type();

    let putchar_type = i32_type.fn_type(&[i32_type.into()], false);
    module.add_function("putchar", putchar_type, None);

    let main_type = i32_type.fn_type(&[], false);
    let function = module.add_function("main", main_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    builder.position_at_end(&basic_block);

    println!("root -> \n{:#?}", root);

    // Any option (test)
    Some(ast::Types::Exp(ast::Expression::new(0)))
}
