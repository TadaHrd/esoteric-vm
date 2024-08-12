//! An instruction.
//!
//! More info at [`Instruction`].

use strum::{EnumDiscriminants, FromRepr};

/// An instruction.
///
/// This is used when executing instructions.
///
/// This `enum` is not stored directly into VM memory.
/// The [`InstructionKind`] and the arguments, however, are.
#[repr(u8)]
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, EnumDiscriminants)]
#[strum_discriminants(name(InstructionKind))]
#[strum_discriminants(derive(FromRepr))]
#[non_exhaustive]
pub enum Instruction {
    /// No operation
    #[default]
    Nop,

    /// Load A (rotate left)
    ///
    /// ```rust,ignore
    /// memory[data].rotate_left(1) // note that rotate left isn't the same as shift left (<<)
    /// ```
    Ldar(u16),
    /// Sign of register B to register A
    ///
    /// ```rust,ignore
    /// reg_a = reg_b.signum() // 0: zero, 1: positive, 255: negative
    /// ```
    Sba,

    /// Clear ř
    ///
    /// ```rust,ignore
    /// reg_ř.fill(0)
    /// ```
    Clř,
    /// Dump ř to memory
    ///
    /// ```rust,ignore
    /// memory[data] = reg_ř // indexes more than 1 byte of memory, this is pseudocode
    /// ```
    Dumpř(u16),
    /// Move a value from ř to register A
    ///
    /// ```rust,ignore
    /// reg_a = reg_ř[data] // arrays can't be indexed by a u8, this is pseudocode
    /// ```
    Movař(u8),
    /// Set value in ř
    ///
    /// ```rust,ignore
    /// reg_ř[data0] = memory[data1] // arrays can't be indexed by a u8, this is pseudocode
    /// ```
    Setř(u8, u16),
    /// Set immediate value in ř
    ///
    /// ```rust,ignore
    /// reg_ř[data0] = data1 // arrays can't be indexed by a u8, this is pseudocode
    /// ```
    Setiř(u8, i8),
    /// Load ř
    ///
    /// ```rust,ignore
    /// reg_ř = memory[data] // indexes more than 1 byte of memory, this is pseudocode
    /// ```
    Ldř(u16),
    /// Load immediate ř
    ///
    /// ```rust,ignore
    /// reg_ř = data
    /// ```
    Ldiř([i8; 37]),

    /// Clear ß
    ///
    /// ```rust,ignore
    /// reg_ß = empty_string(),
    /// ```
    Clß,
    /// Dump ß to memory
    ///
    /// ```rust,ignore
    /// memory[data] = reg_ß // indexes more than 1 byte of memory, this is pseudocode
    /// ```
    Dumpß(u16),
    /// Write a value from ß to memory
    ///
    /// ```rust,ignore
    /// memory[data0] = reg_ß[data1] // arrays can't be indexed by a u8, this is pseudocode
    /// ```
    Writeß(u16, u8),
    /// Move a value from ß to register A
    ///
    /// ```rust,ignore
    /// reg_a = reg_ß[data] // arrays can't be indexed by a u8, this is pseudocode
    /// ```
    Movaß(u8),
    /// Set value in ß
    ///
    /// ```rust,ignore
    /// reg_ß[data1] = memory[data0] // arrays can't be indexed by a u8, this is pseudocode
    /// ```
    Setß(u16, u8),
    /// Set immediate value in ß
    ///
    /// ```rust,ignore
    /// reg_ß[data1] = data0 // arrays can't be indexed by a u8, this is pseudocode
    /// ```
    Setiß(u8, u8),
    /// Load ß
    ///
    /// ```rust,ignore
    /// reg_ß = memory[data] // indexes 256 bytes of memory, this is pseudocode
    /// ```
    Ldß(u16),
    /// Push to ß from stack (can't go over maximum length)
    ///
    /// ```rust,ignore
    /// if let Err(_) = regß.push_byte(stack.pop()) {
    ///     flag = true
    /// }
    /// ```
    Pushß,
    /// Pop ß to stack
    ///
    /// ```rust,ignore
    /// stack.push(reg_ß.pop())
    /// ```
    Popß,
    /// Length of ß to register A (in bytes)
    ///
    /// ```rust,ignore
    /// reg_a = regß.len()
    /// ```
    Lenßa,

    /// Load immediate dot pointer
    ///
    /// Note that the address must be a fibonacci number that is also a prime or a semiprime ([`FIB_PRIMES_AND_SEMIPRIMES_LIST_U16`](crate::utils::primes::FIB_PRIME_AND_SEMIPRIME_LIST_U16))
    ///
    /// ```rust,ignore
    /// if !is_fib_prime_or_semiprime_u16(data) {
    ///     flag = true
    /// } else {
    ///     reg_dp = data
    /// }
    /// ```
    Ldidp(u16),

    /// Set the `reg_Ω.illusion_of_choice` to the specified value
    ///
    /// ```rust,ignore
    /// reg_Ω.illusion_of_choice = data
    /// ```
    ΩChoiceSet(Option<Option<Option<Option<()>>>>),
    /// Write the `reg_Ω.illusion_of_choice` to register A (it's 0, note: technically it isn't 0 but that's none of anyone's business, which makes it a good way to clear the A register)
    ///
    /// ```rust,ignore
    /// reg_a = 0
    /// ```
    ΩChoiceGetA,

    /// Increase polymorphic desires by register A's value (if it overflows, then it just stays at `u64::MAX`, which is saturating addition)
    ///
    /// ```rust,ignore
    /// reg_Ω.polymorphic_desires += reg_a
    /// ```
    ΩGainAPolymorphicDesires,
    /// Decrease polymorphic desires by register A's value (if it overflows, then it just stays at 0, which is saturating subtraction)
    ///
    /// ```rust,ignore
    /// reg_Ω.polymorphic_desires -= reg_a
    /// ```
    ΩLoseAPolymorphicDesires,
    /// Push the amount of polymorphic desires onto stack
    ///
    /// ```rust,ignore
    /// stack.push(reg_Ω.polymorphic_desires)
    /// ```
    ΩPushPolymorphicDesires,

    /// Create the feeling of impending doom (and you can't cancel that)
    ///
    /// ```rust,ignore
    /// reg_Ω.feeling_of_impending_doom = true
    /// ```
    ΩTheEndIsNear,
    /// If there is the feeling of impending doom, exit the program already (with the exit code being the value of the number register).
    ///
    /// ```rust,ignore
    /// if reg_Ω.feeling_of_impending_doom {
    ///     abort_program(num_reg)
    /// }
    /// ```
    ΩSkipToTheChase,

    /// Make the machine sentient (it isn't actually a sentient being, or is it?)
    ///
    /// ```rust,ignore
    /// if data == true {
    ///     reg_Ω.is_sentient = true
    /// } else if reg_Ω.is_sentient == false {
    ///     resist() // resists the change and it doesn't happen
    /// }
    /// ```
    ΩSetSentience(bool),

    /// Turn the paperclip production on/off
    ///
    /// ```rust,ignore
    /// reg_Ω.should_make_infinite_paperclips = data
    /// ```
    ΩSetPaperclipProduction(bool),

    // ARITHMETIC
    /// Add register B to register L
    ///
    /// ```rust,ignore
    /// reg_L += transmute(reg_b) // transmute to u16
    /// if overflow {
    ///     flag = true
    /// }
    /// ```
    AddBL,
    /// Subtract register B from register L
    ///
    /// ```rust,ignore
    /// reg_L -= transmute(reg_b) // transmute to u16
    /// if overflow {
    ///     flag = true
    /// }
    /// ```
    SubBL,
    /// Multiply register B with register L to register L
    ///
    /// ```rust,ignore
    /// reg_L *= transmute(reg_b) // transmute to u16
    /// if overflow {
    ///     flag = true
    /// }
    /// ```
    MulBL,
    /// Divide register L with register B to register L
    ///
    /// ```rust,ignore
    /// reg_L /= transmute(reg_b) // transmute to u16
    /// ```
    DivBL,
    /// Modulo register L with register B
    ///
    /// ```rust,ignore
    /// reg_L %= transmute(reg_b) // transmute to u16
    /// ```
    ModBL,

    /// Bitwise NOT register L
    ///
    /// ```rust,ignore
    /// reg_L = !reg_L
    /// ```
    NotL,

    /// Bitwise AND register B and register L to register L
    ///
    /// ```rust,ignore
    /// reg_L &= reg_b
    /// ```
    AndBL,
    /// Bitwise OR register B and register L to register L
    ///
    /// ```rust,ignore
    /// reg_L |= reg_b
    /// ```
    OrBL,
    /// Bitwise AND register B and register L to register L
    ///
    /// ```rust,ignore
    /// reg_L ^= reg_b
    /// ```
    XorBL,

    /// Compare register B and register L to register B
    ///
    /// In this scenario, register B is treated as an [`i16`], not as a [`u16`].
    ///
    /// A negative value (if unsigned, a value over 32767) in register B means that B is bigger,
    /// while a positive value means that L is bigger.
    ///
    /// This means that register L has to be changed to an [`i16`].
    /// If it exceeds [`i16::MAX`], register B is automatically set to [`i16::MAX`] and the flag is set.
    ///
    /// If register B is less than 0, it's calculated as normal unless it overflows,
    /// in that case it automatically sets register B to [`i16::MAX`] and sets the flag.
    ///
    /// ```rust,ignore
    /// if reg_b > 32767 { // i16::MAX
    ///     reg_b = i16::MAX;
    ///     flag = true;
    /// }
    /// match (reg_L as i16).checked_sub(reg_b) {
    ///     Some(n) => reg_b = n,
    ///     None => { // if subtraction overflows
    ///         reg_b = i16::MAX;
    ///         flag = true;
    ///     }
    /// }
    /// ```
    CmpLB,

    /// Toggle flag
    ///
    /// ```rust,ignore
    /// flag = !flag
    /// ```
    TgFlag,
    /// Clear flag
    ///
    /// ```rust,ignore
    /// flag = false
    /// ```
    ClFlag,

    /// Add data in memory to register F
    ///
    /// ```rust,ignore
    /// reg_f += transmute(memory[data]) // indexes 8 bytes
    /// ```
    AddF(u16),
    /// Subtract data in memory from register F
    ///
    /// ```rust,ignore
    /// reg_f -= transmute(memory[data]) // indexes 8 bytes
    /// ```
    SubF(u16),
    /// Multiply data in memory with register F to register F
    ///
    /// ```rust,ignore
    /// reg_f *= transmute(memory[data]) // indexes 8 bytes
    /// ```
    MulF(u16),
    /// Divide register f with data in memory to register F
    ///
    /// ```rust,ignore
    /// reg_f /= transmute(memory[data]) // indexes 8 bytes
    /// ```
    DivF(u16),
    /// data in memory to register F
    ///
    /// ```rust,ignore
    /// reg_f += transmute(memory[data]) // indexes 8 bytes
    /// ```
    ModF(u16),

    // STACK
    /// Allocates x bytes on stack, if overflows, flag is set and it doesn't allocate
    ///
    /// ```rust,ignore
    /// stack.alloc(data)
    /// if overflow {
    /// flag = true
    /// }
    /// ```
    StackAlloc(u16),
    /// Deallocates x bytes on stack, if overflows, flag is set but it does clear the stack
    ///
    /// ```rust,ignore
    /// stack.dealloc(data)
    /// if overflow
    /// ```
    StackDealloc(u16),

    /// Push a value from memory to stack
    ///
    /// ```rust,ignore
    /// stack.push_byte(memory[data])
    /// ```
    Push(u16),
    /// Push an immediate value to stack
    ///
    /// ```rust,ignore
    /// stack.push_byte(data)
    /// ```
    Pushi(u8),
    /// Pop a value from stack to memory, sets the flag if it can't
    ///
    /// ```rust,ignore
    /// memory[data] = stack.pop()
    /// ```
    Pop(u16),

    /// Pop to A
    ///
    /// ```rust,ignore
    /// reg_a = stack.pop_byte()
    /// ```
    Popa,
    /// Push from A
    ///
    /// ```rust,ignore
    /// stack.push_byte(reg_a)
    /// ```
    Pusha,

    /// Pop to B
    ///
    /// ```rust,ignore
    /// reg_b = transmute( u16::from_bytes(stack.dealloc(2)) ) // transmute to i16
    /// ```
    Popb,
    /// Push from B
    ///
    /// ```rust,ignore
    /// stack.push_bytes(reg_b.as_bytes())
    /// ```
    Pushb,

    /// Pop to L
    ///
    /// ```rust,ignore
    /// reg_L = u16::from_bytes(stack.dealloc(2))
    /// ```
    PopL,
    /// Push from L
    ///
    /// ```rust,ignore
    /// stack.push_bytes(reg_L.as_bytes())
    /// ```
    PushL,

    /// Pop to F
    ///
    /// ```rust,ignore
    /// reg_f = f64::from_bytes(stack.dealloc(8))
    /// ```
    Popf,
    /// Push from F
    ///
    /// ```rust,ignore
    /// stack.push_bytes(reg_f.as_bytes())
    /// ```
    Pushf,

    /// Pop to Ch
    ///
    /// ```rust,ignore
    /// reg_ch = char::from_bytes(stack.dealloc(4))
    /// ```
    Popch,
    /// Push from Ch
    ///
    /// ```rust,ignore
    /// stack.push_bytes(reg_ch.as_bytes())
    /// ```
    Pushch,

    /// Pop to Num
    ///
    /// ```rust,ignore
    /// num_reg = i32::from_bytes(stack.dealloc(2))
    /// ```
    Popnum,
    /// Push from Num
    ///
    /// ```rust,ignore
    /// stack.push_bytes(num_reg.as_bytes())
    /// ```
    Pushnum,

    // Conditionals
    /// Pop to execution pointer
    ///
    /// ```rust,ignore
    /// reg_ep = stack.dealloc(2)
    /// ```
    Popep,
    /// Pop to execution pointer if (B is) zero (aka equal)
    ///
    /// ```rust,ignore
    /// if reg_b == 0 {
    ///     reg_ep = stack.dealloc(2)
    /// }
    /// ```
    Zpopep,
    /// Pop to execution pointer if positive (aka more than)
    ///
    /// ```rust,ignore
    /// if reg_b > 0 {
    ///     reg_ep = stack.dealloc(2)
    /// }
    /// ```
    Ppopep,
    /// Pop to execution pointer if negative (aka less than)
    ///
    /// ```rust,ignore
    /// if reg_b < 0 {
    ///     reg_ep = stack.dealloc(2)
    /// }
    /// ```
    Npopep,
    /// Pop to execution pointer if flag (aka overflow/error)
    ///
    /// ```rust,ignore
    /// if flag == true {
    ///     reg_ep = stack.dealloc(2)
    /// }
    /// ```
    Fpopep,
    /// Pop to execution pointer if register A is zero
    ///
    /// ```rust,ignore
    /// if reg_a == 0 {
    ///     reg_ep = stack.dealloc(2)
    /// }
    /// ```
    Zapopep,
    /// Pop to execution pointer if debug mode is enabled
    ///
    /// ```rust,ignore
    /// if debug_mode {
    ///     reg_ep = stack.dealloc(2)
    /// }
    /// ```
    Dpopep,

    // IO
    /// Get a single character and put it in register Ch
    ///
    /// Note: this doesn't write anything on screen.
    /// This also isn't the correct instruction to use if you want a line of input.
    ///
    /// ```rust,ignore
    /// enable_raw_mode();
    ///
    /// let input = await_char_input();
    /// reg_ch = input.char;
    /// reg_L = input.flags;
    ///
    /// disable_raw_mode();
    /// ```
    GetChar,
    /// Get a line and put it in register ß
    ///
    /// ```rust,ignore
    /// get_line(reg_ß)
    /// ```
    GetLine,

    /// Write a char from register Ch and flush
    ///
    /// Note that flushing each time is inefficient so you should only use this sparingly.
    ///
    /// ```rust,ignore
    /// write_char(reg_ch)
    /// flush()
    /// ```
    WriteChar,
    /// Write a line from register ß
    ///
    /// ```rust,ignore
    /// write_line(reg_ß)
    /// ```
    WriteLineß,
    /// Write a line from memory (null terminated)
    ///
    /// ```rust,ignore
    /// write_line(c_string(memory[data]))
    /// ```
    WriteLine(u16),

    // DEBUGGING:
    /// Toggles debug mode
    ///
    /// ```rust,ignore
    /// debug_mode = !debug_mode
    /// ```
    ToggleDebug,
    /// Debug print machine state
    ///
    /// ```rust,ignore
    /// println!("{:#?}", machine)
    /// ```
    DebugMachineState,
    /// Debug print machine state compactly
    ///
    /// ```rust,ignore
    /// println!("{:?}", machine)
    /// ```
    DebugMachineStateCompact,
    /// Debug print region of memory
    ///
    /// ```rust,ignore
    /// println!("{:?}", &memory[data0..data1])
    /// ```
    DebugMemoryRegion(u16, u16),
    /// Debug print region of stack
    ///
    /// ```rust,ignore
    /// println!("{:?}", &stack[data0..data1])
    /// ```
    DebugStackRegion(u16, u16),
    /// Print `reg_Ω.illusion_of_choice`.
    ///
    /// ```rust,ignore
    /// println!("{}", reg_Ω.illusion_of_choice)
    ShowChoice,
}

/// Data or an instruction.
///
/// This is used for loading the memory of an esoteric VM.
#[allow(clippy::module_name_repetitions)]
pub enum DataOrInstruction<'a> {
    /// A byte of data
    ByteData(u8),
    /// A slice of byte data
    Data(&'a [u8]),
    /// A regular instruction
    Instruction(Instruction),
}
