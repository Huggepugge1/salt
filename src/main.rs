use inkwell::context::Context;
use inkwell::targets::{InitializationConfig, Target};

fn main() {
    Target::initialize_x86(&InitializationConfig::default());

    let context = Context::create();
    let module = context.create_module("main");
    let builder = context.create_builder();

    let void_type = context.void_type();
    let func_type = void_type.fn_type(&[], false);
    let function = module.add_function("main", func_type, None);
    let entry = context.append_basic_block(function, "entry");
    builder.position_at_end(entry);

    let asm_fn_type = context.i64_type().fn_type(&[], false);
    let asm = context.create_inline_asm(
        asm_fn_type,               // function type
        "mov rax, $0".to_string(), // asm string
        "r".to_string(),           // constraints
        true,                      // side_effects
        false,                     // align_stack
        None,                      // Dialect
        false,                     // can_throw
    );

    builder
        .build_indirect_call(asm_fn_type, asm, &[], "")
        .unwrap();

    builder.build_return(None).unwrap();

    module.print_to_stderr();
}
