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


        // set L to 99
        8: pushi 0;
        10: pushi 99;
        12: popl;

        // set num to 99
        13: pushi 0;
        15: pushi 0;
        17: pushi 0;
        19: pushi 99;
        21: popnum;

        // set B to 1
        22: pushi 0;
        24: pushi 1;
        26: popb;

        27: Ωsetpaperclipproduction true;

        // WORK
            29: writeline 110; // wall
            33: writeline 140; // bottles

            35: Ωsetpaperclipproduction false;

            37: writeline 158; // take

            40: Ωsetpaperclipproduction true;

            // jump to fin if L == 1
            42: cmplb;
            43: pushi 0;
            45: pushi 71;

            47: zpopep;

            48: stackdealloc 2;

            // set B to 1
            51: pushi 0;
            53: pushi 1;
            55: popb;

            56: subbl; // L -= 1

            // set num to L
            57: pushi 0;
            59: pushi 0;
            61: pushl;
            62: popnum;

            63: writeline 193; // wall_end

            // jump back to work
            66: pushi 0;
            68: pushi 29;

            70: popep;

        // FIN
            71: Ωsetpaperclipproduction false;

            73: writeline 224; // no_more
            76: writeline 193; // wall_end

            79: writeline 233; // No_more
            82: writeline 110; // wall
            85: writeline 224; // no_more
            88: writeline 140; // bottles

            91: writeline 242; // store

            // set num to 99
            94: pushi 0;
            96: pushi 0;
            98: pushi 0;
            100: pushi 99;
            102: popnum;

            103: Ωsetpaperclipproduction true;

            105: writeline 193; // wall_end

        // halt machine
        108: Ωtheendisnear;
        109: Ωskiptothechase;

        // wall (30 bytes)
        110: data b"bottles of beer on the wall, \0";
        // bottles (18 bytes)
        140: data b"bottles of beer.\n\0";
        // take (35 bytes)
        158: data b"Take one down and pass it around, \0";
        // wall end (31 bytes)
        193: data b"bottles of beer on the wall.\n\n\0";

        // no more (9 bytes)
        224: data b"no more \0";
        // No more (9 bytes)
        233: data b"No more \0";

        // store
        242: data b"Go to the store and buy some more, \0";
    };

    // load machine code
    machine.load(&asm, 0);

    // run machine until it halts
    machine.run();

    // return the machine's register A (unused)
    machine
}
