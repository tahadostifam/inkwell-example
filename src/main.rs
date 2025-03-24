use inkwell::context::Context;
use inkwell::execution_engine::{FunctionLookupError, JitFunction};
use inkwell::module::Linkage;
use inkwell::targets::{InitializationConfig, Target, TargetMachine, RelocMode, CodeModel};
use inkwell::OptimizationLevel;
use inkwell::passes::PassManager;
use std::ffi::c_void;
use std::path::Path;

type MainFunc = unsafe extern "C" fn() -> c_void;

fn main() {
    let context = Context::create();
    let module = context.create_module("my_module");
    let builder = context.create_builder();

    // Define the main function
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = module.add_function("main", fn_type, Some(Linkage::External));
    let basic_block = context.append_basic_block(function, "entry");
    builder.position_at_end(basic_block);

    // Simple return value (42)
    let return_value = i32_type.const_int(42, false);
    let _ = builder.build_return(Some(&return_value));

    // Target initialization
    Target::initialize_native(&InitializationConfig::default()).unwrap();

    let target_triple = TargetMachine::get_default_triple();
    let cpu = TargetMachine::get_host_cpu_name().to_string();
    let features = TargetMachine::get_host_cpu_features().to_string();
    let target = Target::from_triple(&target_triple).unwrap();
    let target_machine = target
        .create_target_machine(
            &target_triple,
            &cpu,
            &features,
            OptimizationLevel::Default,
            RelocMode::Default,
            CodeModel::Default,
        )
        .unwrap();
    module.set_triple(&target_triple);
    module.set_data_layout(&target_machine.get_target_data().get_data_layout());

    module.print_to_stderr();

    let ee = module.create_jit_execution_engine(OptimizationLevel::None).unwrap();
    let func: Result<JitFunction<'_, MainFunc>, FunctionLookupError> = unsafe { ee.get_function("main") };

    dbg!(func);
    
    // // Optimization passes
    // let fpm = PassManager::create(&module);
    // target_machine.add_analysis_passes(&fpm);
    // fpm.initialize();

    // // Write object file
    // let object_file_path = Path::new("my_module.o");
    // target_machine
    //     .write_to_file(&module, inkwell::targets::FileType::Object, &object_file_path)
    //     .unwrap();

    // // Link object file into executable (using system linker)
    // let output_executable_path = Path::new("my_executable");
    // let linker_output = std::process::Command::new("cc")
    //     .arg("my_module.o")
    //     .arg("-o")
    //     .arg("my_executable")
    //     .output()
    //     .expect("failed to execute linker");

    // if !linker_output.status.success() {
    //     eprintln!("Linker error: {}", String::from_utf8_lossy(&linker_output.stderr));
    //     std::fs::remove_file("my_module.o").unwrap();
    //     return;
    // }
}