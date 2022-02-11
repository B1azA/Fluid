use fluid_vm::*;

fn main() {
    println!("Welcome to Fluid, the flowing VM!");

    let bytecode: Vec<u8> = vec![1, 11, 0, 0, 0, 255, 0, 0, 0, 0, 0, 0];
    let mut vm = VM::<u32>::new(bytecode);
    vm.execute();
}
