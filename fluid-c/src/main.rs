use fluid_vm::*;

fn main() {
    println!("Welcome to Fluid, the flowing VM!");

    let bytecode: Vec<u8> = vec![1, 0, 255, 1, 0, 123, 5, 6];
    let mut vm = VM::<u32>::new(bytecode);
    vm.execute();
}
