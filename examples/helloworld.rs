use esoteric_vm::{esoteric_assembly, Machine};

fn main() -> Machine {
    // initialize a new machine
    let mut machine = Machine::default();

    // assembly code for the machine
    let asm = esoteric_assembly! {
        // initialize dot pointer so that IO operations work

        // push a dot character to stack
        0: pushi b'.';
        // pop to address 28657
        2: pop 28657;

        // set dot pointer to 28657 (has to be a prime or semiprime, which is also a fibonacci number)
        5: ldidp 28657;

        // -----------------

        // print hello world
        8: writeline 13;

        // halt machine
        11: Ωtheendisnear;
        12: Ωskiptothechase;

        // hello world text
        13: data b"Hello, world!\n\0";
    };

    // load machine code
    machine.load(&asm, 0);

    // run machine until it halts
    machine.run();

    // return the machine's register A (unused)
    machine
}
