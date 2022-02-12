use fluid_vm::*;

fn main() {
    println!("Welcome to Fluid, the flowing VM!");

    let bytecode: Vec<u8> = vec![1, 0, 218, 3, 11, 10, 0, 123, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11, 0];
    let mut vm = VM::new(bytecode);
    vm.execute();
}
