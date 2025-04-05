use inkwell::context::Context;
use inkwell::OptimizationLevel;

fn main() {
    let context = Context::create();
    let module = context.create_module("my_module");
    let builder = context.create_builder();
    let i32_type = context.i32_type();

    let function_type = i32_type.fn_type(&[], false);
    let function = module.add_function("main", function_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    builder.position_at_end(basic_block);

    // Declare a variable and assign it the value 10
    let var_alloca = builder.build_alloca(i32_type, "my_var").unwrap();
    let const_val = i32_type.const_int(10, false);
    builder.build_store(var_alloca, const_val).unwrap();

    // Load the variable (simulate "getting" its runtime value)
    let loaded_val = builder.build_load(i32_type, var_alloca, "loaded_val").unwrap();

    // Return the loaded value
    builder.build_return(Some(&loaded_val)).unwrap();

    // Print the LLVM IR
    module.print_to_stderr();

    // Note: If you want to actually *execute* and see the value 10 at runtime,
    // you’d need to JIT it using inkwell's execution engine:
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::None)
        .unwrap();
    let compiled_fn = unsafe {
        execution_engine.get_function::<unsafe extern "C" fn() -> i32>("main")
    }.unwrap();

    let result = unsafe { compiled_fn.call() };
    println!("Runtime result: {}", result); // <- This will print 10
}
