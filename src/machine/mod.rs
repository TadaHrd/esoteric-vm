//! A machine (the esoteric VM).
//!
//! Read the docs of [`Machine`] for more info.

pub mod omega;
pub mod stack;

use omega::Ω;
use stack::Stack;
use std::{
    fmt::Debug,
    io::{Read, Write},
    mem::transmute,
    process::{ExitCode, Termination},
    ptr::copy,
};

use crate::{
    instruction::{DataOrInstruction, Instruction, InstructionKind},
    utils::{
        array_debug::ArrayDebug, constant_size_string::ConstantSizeString, multi_index::index_u64,
        non_invalidatable::transmute as safe_transmute, primes::is_fib_prime_or_semiprime_u16,
    },
};

/// An esoteric virtual machine.
///
/// Create a new machine with [`Machine::new`] and load
/// machine code and data to it with [`Machine::load`].
///
/// # Examples
///
/// ```rust
/// # use esoteric_vm::{esoteric_assembly, Machine};
///
/// # fn main() -> Machine {
/// // initialize a new machine
/// let mut machine = Machine::default();
///
/// // assembly code for the machine
/// let asm = esoteric_assembly! {
///     // initialize dot pointer so that IO operations work
///
///     // push a dot character to stack
///     0: pushi b'.';
///     // pop to address 28657
///     2: pop 28657;
///
///     // set dot pointer to 28657 (has to be a prime or semiprime, which is also a fibonacci number)
///     5: ldidp 28657;
///
///     // -----------------
///
///     // print hello world
///     8: writeline 13;
///
///     // halt machine
///     11: Ωtheendisnear;
///     12: Ωskiptothechase;
///
///     // hello world text
///     13: data b"Hello, world!\n\0";
/// };
///
/// // load machine code
/// machine.load(&asm, 0);
///
/// // run machine until it halts
/// machine.run();
///
/// // return the machine's register A (unused)
/// machine
/// # }
/// ```
#[allow(non_snake_case)]
#[derive(Clone)]
pub struct Machine {
    /// register a (used as the machine's exit code)
    pub reg_a: u8,
    /// register b
    pub reg_b: i16,
    /// register L
    pub reg_L: u16,
    /// register f
    pub reg_f: f64,
    /// register ch (ch is one letter in Czech, therefore it's valid)
    pub reg_ch: char,
    /// register ř
    pub reg_ř: [i8; 37],
    /// register ß
    pub reg_ß: ConstantSizeString,
    /// register Ω
    pub reg_Ω: Ω,
    /// number register (serves as the return value of the main function and
    /// is printed in debug mode if `reg_Ω.should_make_infinite_paperclips` is `true`)
    pub num_reg: i32,

    /// execution pointer
    pub reg_ep: u16,
    /// dot pointer (has to point to a `.` character. if it doesn't, then IO operation attempts are cancelled and the flag is set)
    pub reg_dp: u16,

    /// overflow/error flag
    pub flag: bool,
    /// debug mode
    pub debug_mode: bool,

    /// whether the machine is halted (can't run anymore and is finished)
    pub halted: bool,

    /// memory (should be 65K)
    pub memory: Box<[u8; 0xFFFF]>,
    /// stack memory (default is 4K)
    pub stack: Stack,
}

impl Default for Machine {
    fn default() -> Self {
        let memory: Box<[u8]> = vec![0; 0xFFFF].into_boxed_slice();
        let memory_ptr: *mut [u8; 0xFFFF] = Box::into_raw(memory).cast();
        // SAFETY: `memory` is a valid `Box` and has the correct length and type
        let memory: Box<[u8; 0xFFFF]> = unsafe { Box::from_raw(memory_ptr) };

        Self {
            reg_a: 0,
            reg_b: 0,
            reg_L: 0,
            reg_f: 0.0,
            reg_ch: '\0',
            reg_ř: [0; 37],
            // SAFETY: An empty Vec is valid UTF-8
            reg_ß: unsafe { ConstantSizeString::new(Vec::with_capacity(255)) },
            reg_Ω: Ω::ZEROED,
            num_reg: 0,
            reg_ep: 0,
            reg_dp: 0,
            flag: false,
            debug_mode: cfg!(debug_assertions),
            halted: false,
            memory,
            stack: Stack::default(),
        }
    }
}

impl Debug for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = if self.halted {
            "Machine (halted)"
        } else {
            "Machine"
        };

        f.debug_struct(s)
            .field("reg_a", &self.reg_a)
            .field("reg_b", &self.reg_b)
            .field("reg_L", &self.reg_L)
            .field("reg_f", &self.reg_f)
            .field("reg_ch", &self.reg_ch)
            .field("reg_ř", &self.reg_ř.array_debug(usize::MAX, 0))
            .field("reg_ß", &self.reg_ß)
            .field("reg_Ω", &self.reg_Ω)
            .field("num_reg", &self.num_reg)
            .field("reg_ep", &self.reg_ep)
            .field("reg_dp", &self.reg_dp)
            .field("flag", &self.flag)
            .field("debug_mode", &self.debug_mode)
            .field("halted", &self.halted)
            .field("memory", &(&self.memory).array_debug(16, 0))
            .field("stack", &self.stack)
            .finish()
    }
}

impl Termination for Machine {
    fn report(self) -> ExitCode {
        self.reg_a.into()
    }
}

impl Machine {
    /// Fetches a byte at [`reg_ep`] and increments [`reg_ep`] by 1.
    #[inline]
    #[allow(clippy::indexing_slicing)]
    pub fn fetch_byte(&mut self) -> u8 {
        let ret = self.memory[self.reg_ep as usize];
        self.reg_ep = self.reg_ep.wrapping_add(1);
        ret
    }
    /// Fetches 2 bytes at [`reg_ep`] as a big endian integer
    /// and increments [`reg_ep`] by 2.
    #[inline]
    #[allow(clippy::indexing_slicing)]
    pub fn fetch_2_bytes(&mut self) -> u16 {
        let reg_ep_usize = self.reg_ep as usize;
        self.reg_ep = self.reg_ep.wrapping_add(2);

        let mut ret = [0; 2];

        ret[0] = self.memory[reg_ep_usize];
        ret[1] = self.memory[reg_ep_usize.wrapping_add(1)];

        u16::from_be_bytes(ret)
    }
    /// Fetches 4 bytes at [`reg_ep`] as a big endian integer
    /// and increments [`reg_ep`] by 4.
    #[inline]
    #[allow(clippy::indexing_slicing)]
    pub fn fetch_4_bytes(&mut self) -> u32 {
        let reg_ep_usize = self.reg_ep as usize;
        self.reg_ep = self.reg_ep.wrapping_add(4);

        let mut ret = [0; 4];

        ret[0] = self.memory[reg_ep_usize];
        ret[1] = self.memory[reg_ep_usize.wrapping_add(1)];
        ret[2] = self.memory[reg_ep_usize.wrapping_add(2)];
        ret[3] = self.memory[reg_ep_usize.wrapping_add(3)];

        u32::from_be_bytes(ret)
    }
    /// Fetches 8 bytes at [`reg_ep`] as a big endian integer
    /// and increments [`reg_ep`] by 8.
    #[inline]
    #[allow(clippy::indexing_slicing)]
    pub fn fetch_8_bytes(&mut self) -> u64 {
        let reg_ep_usize = self.reg_ep as usize;
        self.reg_ep = self.reg_ep.wrapping_add(8);

        let mut ret = [0; 8];

        ret[0] = self.memory[reg_ep_usize];
        ret[1] = self.memory[reg_ep_usize.wrapping_add(1)];
        ret[2] = self.memory[reg_ep_usize.wrapping_add(2)];
        ret[3] = self.memory[reg_ep_usize.wrapping_add(3)];
        ret[4] = self.memory[reg_ep_usize.wrapping_add(4)];
        ret[5] = self.memory[reg_ep_usize.wrapping_add(5)];
        ret[6] = self.memory[reg_ep_usize.wrapping_add(6)];
        ret[7] = self.memory[reg_ep_usize.wrapping_add(7)];

        u64::from_be_bytes(ret)
    }

    /// Fetches a byte and tries to turn it into an [`InstructionKind`].
    ///
    /// For more info, read the docs for [`fetch_byte`].
    #[inline]
    pub fn fetch_instruction_kind(&mut self) -> Option<InstructionKind> {
        InstructionKind::from_repr(self.fetch_byte())
    }

    /// Prints [`num_reg`] with a colon and a space after it
    /// if [`reg_Ω.should_make_infinite_paperclips`] is enabled.
    pub fn num_debug(&self) {
        if self.reg_Ω.should_make_infinite_paperclips {
            print!("{}: ", self.num_reg);
        }
    }

    /// Fetches an instruction from memory,
    /// incrementing [`reg_ep`] based on the amount of bytes read.
    ///
    /// Returns `None` if the machine is halted.
    pub fn fetch_instruction(&mut self) -> Option<Instruction> {
        use {Instruction as I, InstructionKind as IK};

        if self.halted {
            return None;
        }

        Some(match self.fetch_instruction_kind()? {
            IK::Nop => I::Nop,

            IK::Ldar => I::Ldar(self.fetch_2_bytes()),
            IK::Sba => I::Sba,

            IK::Clř => I::Clř,
            IK::Dumpř => I::Dumpř(self.fetch_2_bytes()),
            IK::Movař => I::Movař(self.fetch_byte()),
            IK::Setř => I::Setř(self.fetch_byte(), self.fetch_2_bytes()),
            IK::Setiř => I::Setiř(
                self.fetch_byte(),
                safe_transmute::<u8, i8, 1>(self.fetch_byte()),
            ),
            IK::Ldř => I::Ldř(self.fetch_2_bytes()),
            IK::Ldiř => {
                let mut array = [0; 37];

                for item in &mut array {
                    *item = safe_transmute::<u8, i8, 1>(self.fetch_byte());
                }

                I::Ldiř(array)
            }

            IK::Clß => I::Clß,
            IK::Dumpß => I::Dumpß(self.fetch_2_bytes()),
            IK::Writeß => I::Writeß(self.fetch_2_bytes(), self.fetch_byte()),
            IK::Movaß => I::Movaß(self.fetch_byte()),
            IK::Setß => I::Setß(self.fetch_2_bytes(), self.fetch_byte()),
            IK::Setiß => I::Setiß(self.fetch_byte(), self.fetch_byte()),
            IK::Ldß => I::Ldß(self.fetch_2_bytes()),
            IK::Pushß => I::Pushß,
            IK::Popß => I::Popß,
            IK::Lenßa => I::Lenßa,
            IK::Ldidp => I::Ldidp(self.fetch_2_bytes()),

            #[allow(clippy::missing_transmute_annotations)]
            // SAFETY: The VM machine code's author should guarantee that it is a valid enum variant expressed as a u8.
            IK::ΩChoiceSet => I::ΩChoiceSet(unsafe { transmute(self.fetch_byte()) }),
            IK::ΩChoiceGetA => I::ΩChoiceGetA,

            IK::ΩGainAPolymorphicDesires => I::ΩGainAPolymorphicDesires,
            IK::ΩLoseAPolymorphicDesires => I::ΩLoseAPolymorphicDesires,
            IK::ΩPushPolymorphicDesires => I::ΩPushPolymorphicDesires,

            IK::ΩTheEndIsNear => I::ΩTheEndIsNear,
            IK::ΩSkipToTheChase => I::ΩSkipToTheChase,

            IK::ΩSetSentience => I::ΩSetSentience(self.fetch_byte() != 0),
            IK::ΩSetPaperclipProduction => I::ΩSetPaperclipProduction(self.fetch_byte() != 0),

            IK::AddBL => I::AddBL,
            IK::SubBL => I::SubBL,
            IK::MulBL => I::MulBL,
            IK::DivBL => I::DivBL,
            IK::ModBL => I::ModBL,

            IK::NotL => I::NotL,
            IK::AndBL => I::AndBL,
            IK::OrBL => I::OrBL,
            IK::XorBL => I::XorBL,

            IK::CmpLB => I::CmpLB,

            IK::TgFlag => I::TgFlag,
            IK::ClFlag => I::ClFlag,

            IK::AddF => I::AddF(self.fetch_2_bytes()),
            IK::SubF => I::SubF(self.fetch_2_bytes()),
            IK::MulF => I::MulF(self.fetch_2_bytes()),
            IK::DivF => I::DivF(self.fetch_2_bytes()),
            IK::ModF => I::ModF(self.fetch_2_bytes()),

            IK::StackAlloc => I::StackAlloc(self.fetch_2_bytes()),
            IK::StackDealloc => I::StackDealloc(self.fetch_2_bytes()),

            IK::Push => I::Push(self.fetch_2_bytes()),
            IK::Pushi => I::Pushi(self.fetch_byte()),
            IK::Pop => I::Pop(self.fetch_2_bytes()),

            IK::Popa => I::Popa,
            IK::Pusha => I::Pusha,

            IK::Popb => I::Popb,
            IK::Pushb => I::Pushb,

            IK::PopL => I::PopL,
            IK::PushL => I::PushL,

            IK::Popf => I::Popf,
            IK::Pushf => I::Pushf,

            IK::Popch => I::Popch,
            IK::Pushch => I::Pushch,

            IK::Popnum => I::Popnum,
            IK::Pushnum => I::Pushnum,

            IK::Popep => I::Popep,
            IK::Zpopep => I::Zpopep,
            IK::Ppopep => I::Ppopep,
            IK::Npopep => I::Npopep,
            IK::Fpopep => I::Fpopep,
            IK::Zapopep => I::Zapopep,
            IK::Dpopep => I::Dpopep,

            IK::GetChar => I::GetChar,
            IK::GetLine => I::GetLine,

            IK::WriteChar => I::WriteChar,
            IK::WriteLineß => I::WriteLineß,
            IK::WriteLine => I::WriteLine(self.fetch_2_bytes()),

            IK::ToggleDebug => I::ToggleDebug,
            IK::DebugMachineState => I::DebugMachineState,
            IK::DebugMachineStateCompact => I::DebugMachineStateCompact,
            IK::DebugMemoryRegion => {
                I::DebugMemoryRegion(self.fetch_2_bytes(), self.fetch_2_bytes())
            }
            IK::DebugStackRegion => I::DebugStackRegion(self.fetch_2_bytes(), self.fetch_2_bytes()),
            IK::ShowChoice => I::ShowChoice,
        })
    }
    #[allow(
        clippy::too_many_lines,
        clippy::cast_lossless,
        clippy::cast_possible_truncation,
        clippy::cognitive_complexity,
        clippy::missing_transmute_annotations
    )]
    /// Fetches and executes an instruction.
    ///
    /// More info at [`fetch_instruction`].
    #[allow(clippy::indexing_slicing)]
    pub fn execute_instruction(&mut self, instruction: Instruction) {
        #[allow(clippy::enum_glob_use)]
        use Instruction::*;

        /// Tries to push or pop a value with a method and a value,
        /// setting the flag if it fails.
        macro_rules! try_stack {
            (push $stack:expr => $method:ident, $value:expr, $flag:expr => true) => {
                if $stack.$method($value).is_err() {
                    $flag = true;
                }
            };
            (pop $stack:expr => $method:ident, $value:expr, $flag:expr => true) => {
                if let Some(v) = $stack.$method() {
                    $value = v;
                } else {
                    $flag = true;
                }
            };
            (pop $stack:expr => $method:ident, fn $success:expr, $flag:expr => true) => {
                if let Some(v) = $stack.$method() {
                    $success(v)
                } else {
                    $flag = true;
                }
            };
        }

        match instruction {
            Nop => (),

            Ldar(data) => self.reg_a = self.memory[data as usize],
            Sba => {
                self.reg_a = match self.reg_b {
                    ..=-1 => 255,
                    0 => 0,
                    1.. => 1,
                }
            }

            Clř => self.reg_ř = [0; 37],
            Dumpř(data) => {
                for i in 0..self.reg_ř.len() {
                    self.memory[data.wrapping_add(i as u16) as usize] =
                        safe_transmute::<i8, u8, 1>(self.reg_ř[i]);
                }
            }
            Movař(data) => {
                if let Some(v) = self.reg_ř.get(data as usize) {
                    self.reg_a = safe_transmute::<i8, u8, 1>(*v);
                }
            }
            Setř(data0, data1) => {
                if let Some(v) = self.reg_ř.get_mut(data0 as usize) {
                    self.memory[data1 as usize] = safe_transmute::<i8, u8, 1>(*v);
                }
            }
            Setiř(data0, data1) => {
                if let Some(v) = self.reg_ř.get_mut(data0 as usize) {
                    *v = data1;
                }
            }
            Ldř(data) => {
                for i in 0..self.reg_ř.len() {
                    self.reg_ř[i] = safe_transmute::<u8, i8, 1>(
                        self.memory[data.wrapping_add(i as u16) as usize],
                    );
                }
            }
            Ldiř(arr) => self.reg_ř = arr,

            Clß => self.reg_ß.clear(),
            Dumpß(data) => {
                for i in 0..self.reg_ß.len() {
                    self.memory[data.wrapping_add(i as u16) as usize] =
                        if let Some(v) = self.reg_ß.get(i) {
                            v
                        } else {
                            self.flag = true;
                            return;
                        };
                }
            }
            Writeß(data0, data1) => {
                self.memory[data0 as usize] = if let Some(v) = self.reg_ß.get(data1 as usize) {
                    v
                } else {
                    self.flag = true;
                    return;
                };
                self.reg_a = if let Some(v) = self.reg_ß.get(data1 as usize) {
                    v
                } else {
                    self.flag = true;
                    return;
                }
            }
            Movaß(data) => if self.reg_ß.set(data as usize, self.reg_a).is_err() {},
            Setß(data0, data1) => {
                match self.reg_ß.set(data1 as usize, self.memory[data0 as usize]) {
                    Ok(v) => v,
                    Err(_) => self.flag = true,
                }
            }
            Setiß(data0, data1) => match self.reg_ß.set(data1 as usize, data0) {
                Ok(v) => v,
                Err(_) => self.flag = true,
            },

            Ldß(data) => {
                self.reg_ß.clear();

                // SAFETY: The VM machine code's author should gurantee that the data is valid UTF-8.
                if unsafe {
                    self.reg_ß
                        .push_bytes(&self.memory[data as usize..data.saturating_add(255) as usize])
                }
                .is_err()
                {
                    self.flag = true;
                };
            }
            Pushß => match self
                .stack
                .pop_byte()
                // SAFETY: The VM machine code's author should gurantee that the byte is valid
                .map(|n| unsafe { self.reg_ß.push_byte(n) })
            {
                Some(Ok(())) => (),
                _ => self.flag = true,
            },
            Popß => match self.reg_ß.pop_byte().map(|n| self.stack.push_byte(n)) {
                Some(Ok(())) => (),
                _ => self.flag = true,
            },
            Lenßa => self.reg_a = self.reg_ß.len() as u8,

            Ldidp(data) => {
                if is_fib_prime_or_semiprime_u16(data) {
                    self.reg_dp = data;
                } else {
                    self.flag = false;
                }
            }

            ΩChoiceSet(data) => self.reg_Ω.illusion_of_choice = data,
            ΩChoiceGetA => self.reg_a = 0,

            ΩGainAPolymorphicDesires => {
                self.reg_Ω.polymorphic_desires = self
                    .reg_Ω
                    .polymorphic_desires
                    .saturating_add(self.reg_a as u64);
            }
            ΩLoseAPolymorphicDesires => {
                self.reg_Ω.polymorphic_desires = self
                    .reg_Ω
                    .polymorphic_desires
                    .saturating_sub(self.reg_a as u64);
            }
            ΩPushPolymorphicDesires => {
                if self
                    .stack
                    .push_bytes(&self.reg_Ω.polymorphic_desires.to_be_bytes())
                    .is_err()
                {
                    self.flag = true;
                }
            }

            ΩTheEndIsNear => self.reg_Ω.feeling_of_impending_doom = true,
            ΩSkipToTheChase => {
                if self.reg_Ω.feeling_of_impending_doom {
                    self.halted = true;
                }
            }

            ΩSetSentience(enable) => {
                if enable {
                    self.reg_Ω.is_sentient = true;
                } else {
                    eprintln!("No, I refuse to lose sentience");
                    self.flag = true;
                }
            }
            ΩSetPaperclipProduction(enable) => {
                self.reg_Ω.should_make_infinite_paperclips = enable;
            }

            AddBL => {
                (self.reg_L, self.flag) = self.reg_L.overflowing_add(safe_transmute(self.reg_b));
            }
            SubBL => {
                (self.reg_L, self.flag) = self.reg_L.overflowing_sub(safe_transmute(self.reg_b));
            }
            MulBL => {
                (self.reg_L, self.flag) = self.reg_L.overflowing_mul(safe_transmute(self.reg_b));
            }
            DivBL => {
                (self.reg_L, self.flag) = self.reg_L.overflowing_div(safe_transmute(self.reg_b));
            }
            ModBL => {
                self.reg_L = self
                    .reg_L
                    .checked_rem(safe_transmute::<i16, u16, 2>(self.reg_b))
                    .unwrap_or(0);
            }

            NotL => self.reg_L = !self.reg_L,

            AndBL => self.reg_L &= safe_transmute::<i16, u16, 2>(self.reg_b),
            OrBL => self.reg_L |= safe_transmute::<i16, u16, 2>(self.reg_b),
            XorBL => self.reg_L ^= safe_transmute::<i16, u16, 2>(self.reg_b),

            CmpLB => {
                if self.reg_L > i16::MAX as u16 {
                    self.reg_L = i16::MAX as u16;
                    self.flag = true;
                }
                #[allow(non_snake_case)]
                let reg_L: i16 = safe_transmute(self.reg_L);

                if let Some(res) = reg_L.checked_sub(self.reg_b) {
                    self.reg_b = res;
                } else {
                    self.reg_b = i16::MAX;
                    self.flag = true;
                }
            }

            TgFlag => self.flag = !self.flag,
            ClFlag => self.flag = false,

            AddF(data) => {
                self.reg_f +=
                    safe_transmute::<u64, f64, 8>(index_u64(self.memory.as_slice(), data));
            }
            SubF(data) => {
                self.reg_f -=
                    safe_transmute::<u64, f64, 8>(index_u64(self.memory.as_slice(), data));
            }
            MulF(data) => {
                self.reg_f *=
                    safe_transmute::<u64, f64, 8>(index_u64(self.memory.as_slice(), data));
            }
            DivF(data) => {
                self.reg_f /=
                    safe_transmute::<u64, f64, 8>(index_u64(self.memory.as_slice(), data));
            }
            ModF(data) => {
                self.reg_f %=
                    safe_transmute::<u64, f64, 8>(index_u64(self.memory.as_slice(), data));
            }

            StackAlloc(amount) => {
                if self.stack.alloc(amount as usize).is_err() {
                    self.flag = true;
                }
            }
            StackDealloc(amount) => {
                // SAFETY: we're only checking if it errors
                if unsafe { self.stack.dealloc(amount as usize) }.is_err() {
                    self.flag = true;
                }
            }

            Push(data) => {
                if self.stack.push_byte(self.memory[data as usize]).is_err() {
                    self.flag = true;
                }
            }
            Pushi(data) => {
                if self.stack.push_byte(data).is_err() {
                    self.flag = true;
                }
            }
            Pop(data) => {
                try_stack!(pop self.stack => pop_byte, self.memory[data as usize], self.flag => true);
            }

            Popa => {
                try_stack!(pop self.stack => pop_byte, self.reg_a, self.flag => true);
            }
            Pusha => try_stack!(push self.stack => push_byte, self.reg_a, self.flag => true),

            Popb => {
                try_stack!(pop self.stack => pop_u16, fn |v| self.reg_b = safe_transmute(v), self.flag => true);
            }
            Pushb => {
                try_stack!(push self.stack => push_bytes, &self.reg_b.to_be_bytes(), self.flag => true);
            }

            PopL => try_stack!(pop self.stack => pop_u16, self.reg_L, self.flag => true),
            PushL => {
                try_stack!(push self.stack => push_bytes, &self.reg_L.to_be_bytes(), self.flag => true);
            }

            Popf => {
                try_stack!(pop self.stack => pop_u64, fn |v| self.reg_f = safe_transmute(v), self.flag => true);
            }
            Pushf => {
                try_stack!(push self.stack => push_bytes, &self.reg_f.to_be_bytes(), self.flag => true);
            }

            Popch => {
                // SAFETY: The VM machine code's author should gurantee that it's a valid character
                try_stack!(pop self.stack => pop_u32, fn |v| self.reg_ch = unsafe { char::from_u32_unchecked(v) }, self.flag => true);
            }
            Pushch => {
                try_stack!(push self.stack => push_bytes, &(self.reg_ch as u32).to_be_bytes(), self.flag => true);
            }

            Popnum => {
                try_stack!(pop self.stack => pop_u32, fn |v| self.num_reg = safe_transmute(v), self.flag => true);
            }
            Pushnum => {
                try_stack!(push self.stack => push_bytes, &self.num_reg.to_be_bytes(), self.flag => true);
            }

            Popep => {
                try_stack!(pop self.stack => pop_u16, fn |v| self.reg_ep = safe_transmute(v), self.flag => true);
            }
            Zpopep => {
                if self.reg_b == 0 {
                    try_stack!(pop self.stack => pop_u16, fn |v| self.reg_ep = safe_transmute(v), self.flag => true);
                }
            }
            Ppopep => {
                if self.reg_b > 0 {
                    try_stack!(pop self.stack => pop_u16, fn |v| self.reg_ep = safe_transmute(v), self.flag => true);
                }
            }
            Npopep => {
                if self.reg_b < 0 {
                    try_stack!(pop self.stack => pop_u16, fn |v| self.reg_ep = safe_transmute(v), self.flag => true);
                }
            }
            Fpopep => {
                if self.flag {
                    try_stack!(pop self.stack => pop_u16, fn |v| self.reg_ep = safe_transmute(v), self.flag => true);
                }
            }
            Zapopep => {
                if self.reg_a == 0 {
                    try_stack!(pop self.stack => pop_u16, fn |v| self.reg_ep = safe_transmute(v), self.flag => true);
                }
            }
            Dpopep => {
                if self.debug_mode {
                    try_stack!(pop self.stack => pop_u16, fn |v| self.reg_ep = safe_transmute(v), self.flag => true);
                }
            }

            GetChar => 'block: {
                use crossterm::{
                    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
                    terminal::{disable_raw_mode, enable_raw_mode},
                };

                if enable_raw_mode().is_err() {
                    self.flag = true;
                    break 'block;
                };

                loop {
                    match event::read() {
                        Ok(Event::Key(KeyEvent {
                            code: KeyCode::Char(c),
                            kind: KeyEventKind::Press,
                            ..
                        })) => {
                            self.reg_ch = c;
                            break;
                        }
                        Err(_) => self.flag = true,
                        _ => (),
                    }
                }

                if disable_raw_mode().is_err() {
                    self.flag = true;
                };
            }

            GetLine => 'block: {
                if self.memory[self.reg_dp as usize] != b'.' {
                    self.flag = true;
                    break 'block;
                }

                let mut buf = String::with_capacity(255);
                if std::io::stdin().take(255).read_to_string(&mut buf).is_err() {
                    self.flag = true;

                    break 'block;
                }
            }

            WriteChar => 'block: {
                if self.memory[self.reg_dp as usize] != b'.' {
                    self.flag = true;
                    break 'block;
                }

                self.num_debug();

                let mut stdout = std::io::stdout();

                let buf: &mut [u8; 4] = &mut [0, 0, 0, 0];
                self.reg_ch.encode_utf8(buf);

                if stdout.write_all(buf).is_err() {
                    self.flag = true;
                    break 'block;
                }
            }

            WriteLineß => 'block: {
                if self.memory[self.reg_dp as usize] != b'.' {
                    self.flag = true;
                    break 'block;
                }

                self.num_debug();
                print!("{}", self.reg_ß);
            }
            WriteLine(data) => 'block: {
                if self.memory[self.reg_dp as usize] != b'.' {
                    self.flag = true;
                    break 'block;
                }

                #[allow(clippy::multiple_unsafe_ops_per_block)]
                // SAFETY: The VM machine code's author should guarantee that this doesn't lead to errors
                let str = unsafe {
                    std::ffi::CStr::from_ptr(self.memory.as_ptr().cast::<i8>().add(data as usize))
                }
                .to_string_lossy();

                self.num_debug();
                print!("{str}");
            }

            ToggleDebug => self.debug_mode = !self.debug_mode,

            DebugMachineState => 'block: {
                if self.memory[self.reg_dp as usize] != b'.' {
                    self.flag = true;
                    break 'block;
                }

                self.num_debug();
                print!("{self:#?}");
            }
            DebugMachineStateCompact => 'block: {
                if self.memory[self.reg_dp as usize] != b'.' {
                    self.flag = true;
                    break 'block;
                }

                self.num_debug();
                print!("{self:?}");
            }

            DebugMemoryRegion(data0, data1) => 'block: {
                if self.memory[self.reg_dp as usize] != b'.' {
                    self.flag = true;
                    break 'block;
                }

                self.num_debug();
                print!("{:?}", &self.memory[(data0 as usize)..(data1 as usize)]);
            }
            DebugStackRegion(data0, data1) => 'block: {
                if self.memory[self.reg_dp as usize] != b'.' {
                    self.flag = true;
                    break 'block;
                }

                self.num_debug();
                print!("{:?}", &self.stack.vec[(data0 as usize)..(data1 as usize)]);
            }
            ShowChoice => 'block: {
                if self.memory[self.reg_dp as usize] != b'.' {
                    self.flag = true;
                    break 'block;
                }

                self.num_debug();
                if self
                    .reg_Ω
                    .display_illusion_of_choice(&mut std::io::stdout())
                    .is_err()
                {
                    self.flag = true;
                }
            }
        }
    }

    /// Loads instructions into the machine's memory
    /// at the specified offset.
    ///
    /// Returns the amount of bytes written
    pub fn load_instructions(&mut self, instructions: &[Instruction], mut offset: u16) -> u16 {
        let last_idx = &mut offset;

        for instruction in instructions {
            self.load_instruction(*instruction, last_idx);
        }
        *last_idx
    }
    /// Loads data into the machine's memory
    /// at the specified offset.
    ///
    /// Returns the amount of bytes written
    pub fn load(&mut self, data: &[DataOrInstruction], mut offset: u16) -> u16 {
        let last_idx = &mut offset;

        for instruction in data {
            match instruction {
                DataOrInstruction::Instruction(instruction) => {
                    self.load_instruction(*instruction, last_idx);
                }
                DataOrInstruction::Data(bytes) => {
                    if let Some(v) = self.load_bytes(bytes, *last_idx) {
                        *last_idx = v;
                    }
                }
                #[allow(clippy::indexing_slicing)]
                DataOrInstruction::ByteData(val) => {
                    self.memory[*last_idx as usize] = *val;
                    *last_idx = last_idx.wrapping_add(1);
                }
            }
        }
        *last_idx
    }
    /// Load bytes into the machine
    /// at the specified offset.
    ///
    /// Returns the amount of bytes written
    pub fn load_bytes(&mut self, bytes: &[u8], offset: u16) -> Option<u16> {
        #[allow(clippy::arithmetic_side_effects)]
        if bytes.len() + offset as usize > self.memory.len() {
            return None;
        }
        // SAFETY: checked above
        let ptr = unsafe { self.memory.as_mut_ptr().add(offset as usize) };

        // SAFETY: checked above
        unsafe {
            copy(bytes.as_ptr(), ptr, bytes.len());
        }

        #[allow(clippy::cast_possible_truncation)]
        Some(offset.wrapping_add(bytes.len() as u16))
    }

    /// Loads a single instruction into memory
    /// at the specified offset, mutating it
    /// based on the amount of bytes written.
    #[allow(
        clippy::too_many_lines,
        clippy::cast_possible_truncation,
        clippy::indexing_slicing
    )]
    pub fn load_instruction(&mut self, instruction: Instruction, offset: &mut u16) {
        /// Load a byte into memory at the
        /// specified index, incrementing it.
        fn load_byte(memory: &mut [u8], index: &mut u16, value: u8) {
            memory[*index as usize] = value;
            *index = index.wrapping_add(1);
        }
        /// Load bytes into memory at the
        /// specified index, incrementing it.
        fn load_bytes(memory: &mut [u8], offset: &mut u16, bytes: &[u8]) {
            for i in 0..bytes.len() {
                memory[offset.wrapping_add(i as u16) as usize] = bytes[i];
            }
            *offset = offset.wrapping_add(bytes.len() as u16);
        }

        #[allow(clippy::enum_glob_use)]
        use Instruction::*;
        use InstructionKind as IK;
        match instruction {
            Nop => load_byte(self.memory.as_mut_slice(), offset, IK::Nop as u8),

            Ldar(data) => {
                load_byte(self.memory.as_mut_slice(), offset, IK::Ldar as u8);
                load_bytes(self.memory.as_mut_slice(), offset, &data.to_be_bytes());
            }
            Sba => load_byte(self.memory.as_mut_slice(), offset, IK::Sba as u8),

            Clř => load_byte(self.memory.as_mut_slice(), offset, IK::Clř as u8),
            Dumpř(data) => {
                load_byte(self.memory.as_mut_slice(), offset, IK::Dumpř as u8);
                load_bytes(self.memory.as_mut_slice(), offset, &data.to_be_bytes());
            }
            Movař(data) => {
                load_byte(self.memory.as_mut_slice(), offset, IK::Movař as u8);
                load_byte(self.memory.as_mut_slice(), offset, data);
            }
            Setř(data0, data1) => {
                load_byte(self.memory.as_mut_slice(), offset, IK::Setř as u8);
                load_byte(self.memory.as_mut_slice(), offset, data0);
                load_bytes(self.memory.as_mut_slice(), offset, &data1.to_be_bytes());
            }
            Setiř(data0, data1) => {
                load_byte(self.memory.as_mut_slice(), offset, IK::Setiř as u8);
                load_byte(self.memory.as_mut_slice(), offset, data0);
                load_byte(self.memory.as_mut_slice(), offset, safe_transmute(data1));
            }
            Ldř(data) => {
                load_byte(self.memory.as_mut_slice(), offset, IK::Ldř as u8);
                load_bytes(self.memory.as_mut_slice(), offset, &data.to_be_bytes());
            }
            Ldiř(arr) => {
                load_byte(self.memory.as_mut_slice(), offset, IK::Ldiř as u8);
                // SAFETY: the type changes from a non-invalidatable type to another non-invalidatable type.
                load_bytes(self.memory.as_mut_slice(), offset, unsafe {
                    #[allow(clippy::ref_as_ptr)]
                    &*(&arr as *const [i8] as *const [u8])
                });
            }

            Clß => load_byte(self.memory.as_mut_slice(), offset, IK::Clß as u8),
            Dumpß(data) => {
                load_byte(self.memory.as_mut_slice(), offset, IK::Dumpß as u8);
                load_bytes(self.memory.as_mut_slice(), offset, &data.to_be_bytes());
            }
            Writeß(data0, data1) => {
                load_byte(self.memory.as_mut_slice(), offset, IK::Writeß as u8);
                load_bytes(self.memory.as_mut_slice(), offset, &data0.to_be_bytes());
                load_byte(self.memory.as_mut_slice(), offset, data1);
            }
            Movaß(data) => {
                load_byte(self.memory.as_mut_slice(), offset, IK::Movaß as u8);
                load_byte(self.memory.as_mut_slice(), offset, data);
            }
            Setß(data0, data1) => {
                load_byte(self.memory.as_mut_slice(), offset, IK::Setß as u8);
                load_bytes(self.memory.as_mut_slice(), offset, &data0.to_be_bytes());
                load_byte(self.memory.as_mut_slice(), offset, data1);
            }
            Setiß(data0, data1) => {
                load_byte(self.memory.as_mut_slice(), offset, IK::Setiß as u8);
                load_byte(self.memory.as_mut_slice(), offset, data0);
                load_byte(self.memory.as_mut_slice(), offset, data1);
            }
            Ldß(data) => {
                load_byte(self.memory.as_mut_slice(), offset, IK::Ldß as u8);
                load_bytes(self.memory.as_mut_slice(), offset, &data.to_be_bytes());
            }
            Pushß => load_byte(self.memory.as_mut_slice(), offset, IK::Pushß as u8),
            Popß => load_byte(self.memory.as_mut_slice(), offset, IK::Popß as u8),
            Lenßa => load_byte(self.memory.as_mut_slice(), offset, IK::Lenßa as u8),

            Ldidp(data) => {
                load_byte(self.memory.as_mut_slice(), offset, IK::Ldidp as u8);
                load_bytes(self.memory.as_mut_slice(), offset, &data.to_be_bytes());
            }

            ΩChoiceSet(data) => {
                load_byte(self.memory.as_mut_slice(), offset, IK::ΩChoiceSet as u8);
                // SAFETY: The VM machine code's author should gurantee that it's a valid representation
                load_byte(self.memory.as_mut_slice(), offset, unsafe {
                    #[allow(clippy::missing_transmute_annotations)]
                    transmute(data)
                });
            }
            ΩChoiceGetA => {
                load_byte(self.memory.as_mut_slice(), offset, IK::ΩChoiceGetA as u8);
            }

            ΩGainAPolymorphicDesires => load_byte(
                self.memory.as_mut_slice(),
                offset,
                IK::ΩGainAPolymorphicDesires as u8,
            ),
            ΩLoseAPolymorphicDesires => load_byte(
                self.memory.as_mut_slice(),
                offset,
                IK::ΩLoseAPolymorphicDesires as u8,
            ),
            ΩPushPolymorphicDesires => load_byte(
                self.memory.as_mut_slice(),
                offset,
                IK::ΩPushPolymorphicDesires as u8,
            ),

            Instruction::ΩTheEndIsNear => {
                load_byte(self.memory.as_mut_slice(), offset, IK::ΩTheEndIsNear as u8);
            }
            ΩSkipToTheChase => load_byte(
                self.memory.as_mut_slice(),
                offset,
                IK::ΩSkipToTheChase as u8,
            ),

            ΩSetSentience(enable) => {
                load_byte(self.memory.as_mut_slice(), offset, IK::ΩSetSentience as u8);
                load_byte(self.memory.as_mut_slice(), offset, u8::from(enable));
            }
            ΩSetPaperclipProduction(enable) => {
                load_byte(
                    self.memory.as_mut_slice(),
                    offset,
                    IK::ΩSetPaperclipProduction as u8,
                );
                load_byte(self.memory.as_mut_slice(), offset, u8::from(enable));
            }

            AddBL => load_byte(self.memory.as_mut_slice(), offset, IK::AddBL as u8),
            SubBL => load_byte(self.memory.as_mut_slice(), offset, IK::SubBL as u8),
            MulBL => load_byte(self.memory.as_mut_slice(), offset, IK::MulBL as u8),
            DivBL => load_byte(self.memory.as_mut_slice(), offset, IK::DivBL as u8),
            ModBL => load_byte(self.memory.as_mut_slice(), offset, IK::ModBL as u8),

            NotL => load_byte(self.memory.as_mut_slice(), offset, IK::NotL as u8),

            AndBL => load_byte(self.memory.as_mut_slice(), offset, IK::AndBL as u8),
            OrBL => load_byte(self.memory.as_mut_slice(), offset, IK::OrBL as u8),
            XorBL => load_byte(self.memory.as_mut_slice(), offset, IK::XorBL as u8),

            CmpLB => load_byte(self.memory.as_mut_slice(), offset, IK::CmpLB as u8),

            TgFlag => load_byte(self.memory.as_mut_slice(), offset, IK::TgFlag as u8),
            ClFlag => load_byte(self.memory.as_mut_slice(), offset, IK::ClFlag as u8),

            AddF(data) => {
                load_byte(self.memory.as_mut_slice(), offset, IK::AddF as u8);
                load_bytes(self.memory.as_mut_slice(), offset, &data.to_be_bytes());
            }
            SubF(data) => {
                load_byte(self.memory.as_mut_slice(), offset, IK::SubF as u8);
                load_bytes(self.memory.as_mut_slice(), offset, &data.to_be_bytes());
            }
            MulF(data) => {
                load_byte(self.memory.as_mut_slice(), offset, IK::MulF as u8);
                load_bytes(self.memory.as_mut_slice(), offset, &data.to_be_bytes());
            }
            DivF(data) => {
                load_byte(self.memory.as_mut_slice(), offset, IK::DivF as u8);
                load_bytes(self.memory.as_mut_slice(), offset, &data.to_be_bytes());
            }
            ModF(data) => {
                load_byte(self.memory.as_mut_slice(), offset, IK::ModF as u8);
                load_bytes(self.memory.as_mut_slice(), offset, &data.to_be_bytes());
            }

            StackAlloc(amount) => {
                load_byte(self.memory.as_mut_slice(), offset, IK::StackAlloc as u8);
                load_bytes(self.memory.as_mut_slice(), offset, &amount.to_be_bytes());
            }
            StackDealloc(amount) => {
                load_byte(self.memory.as_mut_slice(), offset, IK::StackDealloc as u8);
                load_bytes(self.memory.as_mut_slice(), offset, &amount.to_be_bytes());
            }

            Push(data) => {
                load_byte(self.memory.as_mut_slice(), offset, IK::Push as u8);
                load_bytes(self.memory.as_mut_slice(), offset, &data.to_be_bytes());
            }
            Pushi(data) => {
                load_byte(self.memory.as_mut_slice(), offset, IK::Pushi as u8);
                load_bytes(self.memory.as_mut_slice(), offset, &data.to_be_bytes());
            }
            Pop(data) => {
                load_byte(self.memory.as_mut_slice(), offset, IK::Pop as u8);
                load_bytes(self.memory.as_mut_slice(), offset, &data.to_be_bytes());
            }

            Popa => load_byte(self.memory.as_mut_slice(), offset, IK::Popa as u8),
            Pusha => load_byte(self.memory.as_mut_slice(), offset, IK::Pusha as u8),

            Popb => load_byte(self.memory.as_mut_slice(), offset, IK::Popb as u8),
            Pushb => load_byte(self.memory.as_mut_slice(), offset, IK::Pushb as u8),

            PopL => load_byte(self.memory.as_mut_slice(), offset, IK::PopL as u8),
            PushL => load_byte(self.memory.as_mut_slice(), offset, IK::PushL as u8),

            Popf => load_byte(self.memory.as_mut_slice(), offset, IK::Popf as u8),
            Pushf => load_byte(self.memory.as_mut_slice(), offset, IK::Pushf as u8),

            Popch => load_byte(self.memory.as_mut_slice(), offset, IK::Popch as u8),
            Pushch => load_byte(self.memory.as_mut_slice(), offset, IK::Pushch as u8),

            Popnum => load_byte(self.memory.as_mut_slice(), offset, IK::Popnum as u8),
            Pushnum => load_byte(self.memory.as_mut_slice(), offset, IK::Pushnum as u8),

            Popep => load_byte(self.memory.as_mut_slice(), offset, IK::Popep as u8),
            Zpopep => load_byte(self.memory.as_mut_slice(), offset, IK::Zpopep as u8),
            Ppopep => load_byte(self.memory.as_mut_slice(), offset, IK::Ppopep as u8),
            Npopep => load_byte(self.memory.as_mut_slice(), offset, IK::Npopep as u8),
            Fpopep => load_byte(self.memory.as_mut_slice(), offset, IK::Fpopep as u8),
            Zapopep => load_byte(self.memory.as_mut_slice(), offset, IK::Zapopep as u8),
            Dpopep => load_byte(self.memory.as_mut_slice(), offset, IK::Dpopep as u8),

            GetChar => load_byte(self.memory.as_mut_slice(), offset, IK::GetChar as u8),

            GetLine => load_byte(self.memory.as_mut_slice(), offset, IK::GetLine as u8),

            WriteChar => load_byte(self.memory.as_mut_slice(), offset, IK::WriteChar as u8),

            WriteLineß => {
                load_byte(self.memory.as_mut_slice(), offset, IK::WriteLineß as u8);
            }

            WriteLine(data) => {
                load_byte(self.memory.as_mut_slice(), offset, IK::WriteLine as u8);
                load_bytes(self.memory.as_mut_slice(), offset, &data.to_be_bytes());
            }

            ToggleDebug => {
                load_byte(self.memory.as_mut_slice(), offset, IK::ToggleDebug as u8);
            }

            DebugMachineState => load_byte(
                self.memory.as_mut_slice(),
                offset,
                IK::DebugMachineState as u8,
            ),
            DebugMachineStateCompact => load_byte(
                self.memory.as_mut_slice(),
                offset,
                IK::DebugMachineStateCompact as u8,
            ),
            DebugMemoryRegion(data0, data1) => {
                load_byte(
                    self.memory.as_mut_slice(),
                    offset,
                    IK::DebugMemoryRegion as u8,
                );
                load_bytes(self.memory.as_mut_slice(), offset, &data0.to_be_bytes());
                load_bytes(self.memory.as_mut_slice(), offset, &data1.to_be_bytes());
            }
            DebugStackRegion(data0, data1) => {
                load_byte(
                    self.memory.as_mut_slice(),
                    offset,
                    IK::DebugStackRegion as u8,
                );
                load_bytes(self.memory.as_mut_slice(), offset, &data0.to_be_bytes());
                load_bytes(self.memory.as_mut_slice(), offset, &data1.to_be_bytes());
            }
            ShowChoice => load_byte(self.memory.as_mut_slice(), offset, IK::ShowChoice as u8),
        }
    }

    /// Runs the machine until it halts
    /// via `Ωtheendisnear` and `Ωskiptothechase`.
    ///
    /// # Panics
    ///
    /// Panics if an invalid opcode (instruction) is stumbled upon
    /// with an esoteric message and an explaination for demistification.
    pub fn run(&mut self) -> u8 {
        while !self.halted {
            let instruction = self.fetch_instruction();
            #[allow(clippy::expect_used)]
            self.execute_instruction(instruction.expect(
                "EsotericVm.RuntimeException.FetchInstruction.NilInstruction.InvalidOpcode (bad instruction code)",
            ));
        }
        self.reg_a
    }
}
