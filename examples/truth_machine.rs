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
        4: ldidp 28657;

        // ------------------

        // get a single character to register ch
        7: getchar;

        // push register ch to stack (4 bytes)
        8: pushch;

        // pop character to register b
        9: popb;

        // pop the other 2 bytes
        10: stackdealloc 2;

        // put 49 (ascii for '1') in register L
        13: pushi 0;
        15: pushi 49;
        17: popl;

        // compare registers L and b
        18: cmplb;

        // push 30 (input == '1' logic)
        19: pushi 0;
        21: pushi 30;

        // pop to execution pointer (jump to value on stack) if b is 0 (input is '1')
        23: zpopep;

        // empty the stack
        24: stackdealloc 2;

        // IF INPUT IS '0' (repeat it)
        // fallback to the input == '1' logic, ran without jumps

        // write the input value ('0')
        27: writechar;

        // halt machine
        28: Ωtheendisnear;
        29: Ωskiptothechase;

        // IF INPUT IS '1' (repeat indefinitely)

        // write the input value ('1')
        30: writechar;

        // push 30 (input == '1' logic)
        31: pushi 0;
        33: pushi 30;

        // pop to execution pointer (jump to value on stack)
        //
        // this repeats the input == '1' logic indefinitely
        35: popep;

    };

    // load machine code
    machine.load(&asm, 0);

    // run machine until it halts
    machine.run();

    // return the machine's register A (unused)
    machine
}
