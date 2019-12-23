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

    // println!("root -> \n{:#?}", root);
    for node in root.node {
        // println!("node -> {:#?}", node);
        match node {
            ast::Types::Exp(val) => {
                println!("Matched Type::Exp");

                if val.token == -6 {
                    let text =  match val.value {
                        ast::ValueTypes::Str(val) => val,
                        _ => String::new()
                    };

                    let fun = module.get_function("putchar");
                    for c in text.chars() {
                        let ascii = c.to_string().as_bytes()[0] as u64;
                        builder.build_call(fun.unwrap(), &[i32_type.const_int(ascii, false).into()], "putchar");
                    }
                }
            },
            _ => println!("Unknown type matched")
        }
    }

    builder.build_return(Some(&i32_type.const_int(0, false)));
    module.print_to_stderr();

    // Any option (test)
    Some(ast::Types::Exp(ast::Expression::new(0)))
}
