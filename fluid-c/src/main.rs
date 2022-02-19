use fluid_vm::*;

fn main() {
    println!("Welcome to Fluid, the flowing VM!");

    let bytecode: Vec<u8> = vec![1, 1, 0, 255, 1, 0, 0, 17];
    let mut vm = VM::new(bytecode);
    vm.execute();
}
