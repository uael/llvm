extern crate llvm;
use llvm::*;
use std::mem;

fn main() {

    let context = Context::new();
    let mut module = context.module_create_with_name("sum");
    let mut builder = context.create_builder();

    let function_type = llvm::function_type(
        context.int64_type(),
        vec![context.int64_type(), context.int64_type(), context.int64_type()],
        false);
    let mut func = module.add_function(function_type, "fname");
    let bb = context.append_basic_block(&mut func, "fname");
    builder.position_at_end(bb);

    // get the function's arguments
    let x = func.get_param(0).unwrap();
    let y = func.get_param(0).unwrap();
    let z = func.get_param(0).unwrap();

    let sum = builder.build_add(x, y, "sum");
    let sum = builder.build_add(sum, z, "sum");
    builder.build_ret(sum);

    module.dump();

    llvm::link_in_mcjit();
    llvm::initialize_native_target();
    llvm::initialize_native_asm_printer();

    let ee = llvm::ExecutionEngine::create_for_module(module).unwrap();
    let addr = ee.get_function_address("fname").unwrap();

    unsafe {
        let f: extern "C" fn(u64, u64, u64) -> u64 = mem::transmute(addr);

        let x: u64 = 1;
        let y: u64 = 1;
        let z: u64 = 1;
        let res = f(x, y, z);

        println!("{} + {} + {} = {}", x, y, z, res);
    }
}
