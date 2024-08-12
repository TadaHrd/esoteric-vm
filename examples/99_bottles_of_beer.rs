use esoteric_vm::{esoteric_assembly, machine::Machine};

fn main() -> Machine {
    // initialize a new machine
    let mut machine = Machine::default();

    // assembly code for the machine
    let asm = esoteric_assembly! {};

    // load machine code
    machine.load(&asm, 0);

    // run machine until it halts
    machine.run();

    // return the machine's register A (unused)
    machine
}
