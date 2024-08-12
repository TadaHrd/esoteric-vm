//! Assembly compiler for Esoteric VM.
//!
//! More info at [`esoteric_assembly`].

#[doc(hidden)]
#[allow(non_upper_case_globals)]
pub mod __instructions {
    #[allow(non_camel_case_types)]
    pub struct instruction;

    pub const data: instruction = instruction;
    pub const DATA: instruction = instruction;

    pub const byte: instruction = instruction;
    pub const BYTE: instruction = instruction;

    pub const nop: instruction = instruction;
    pub const NOP: instruction = instruction;

    pub const ldar: instruction = instruction;
    pub const LDAR: instruction = instruction;

    pub const sba: instruction = instruction;
    pub const SBA: instruction = instruction;

    pub const clř: instruction = instruction;
    pub const CLŘ: instruction = instruction;

    pub const dumpř: instruction = instruction;
    pub const DUMPŘ: instruction = instruction;

    pub const movař: instruction = instruction;
    pub const MOVAŘ: instruction = instruction;

    pub const setř: instruction = instruction;
    pub const SETŘ: instruction = instruction;

    pub const setiř: instruction = instruction;
    pub const SETIŘ: instruction = instruction;

    pub const ldř: instruction = instruction;
    pub const LDŘ: instruction = instruction;

    pub const ldiř: instruction = instruction;
    pub const LDIŘ: instruction = instruction;

    pub const clß: instruction = instruction;
    pub const CLß: instruction = instruction;

    pub const dumpß: instruction = instruction;
    pub const DUMPß: instruction = instruction;

    pub const writeß: instruction = instruction;
    pub const WRITEß: instruction = instruction;

    pub const movaß: instruction = instruction;
    pub const MOVAß: instruction = instruction;

    pub const setß: instruction = instruction;
    pub const SETß: instruction = instruction;

    pub const setiß: instruction = instruction;
    pub const SETIß: instruction = instruction;

    pub const ldß: instruction = instruction;
    pub const LDß: instruction = instruction;

    pub const pushß: instruction = instruction;
    pub const PUSHß: instruction = instruction;

    pub const popß: instruction = instruction;
    pub const POPß: instruction = instruction;

    pub const lenßa: instruction = instruction;
    pub const LENßA: instruction = instruction;

    pub const ldidp: instruction = instruction;
    pub const LDIDP: instruction = instruction;

    pub const Ωchoiceset: instruction = instruction;
    pub const ΩCHOICESET: instruction = instruction;

    pub const Ωchoicegeta: instruction = instruction;
    pub const ΩCHOICEGETA: instruction = instruction;

    pub const Ωgainapolymorphicdesires: instruction = instruction;
    pub const ΩGAINAPOLYMORPHICDESIRES: instruction = instruction;

    pub const Ωloseapolymorphicdesires: instruction = instruction;
    pub const ΩLOSEAPOLYMORPHICDESIRES: instruction = instruction;

    pub const Ωpushpolymorphicdesires: instruction = instruction;
    pub const ΩPUSHPOLYMORPHICDESIRES: instruction = instruction;

    pub const Ωtheendisnear: instruction = instruction;
    pub const ΩTHEENDISNEAR: instruction = instruction;

    pub const Ωskiptothechase: instruction = instruction;
    pub const ΩSKIPTOTHECHASE: instruction = instruction;

    pub const Ωsetsentience: instruction = instruction;
    pub const ΩSETSENTIENCE: instruction = instruction;

    pub const Ωsetpaperclipproduction: instruction = instruction;
    pub const ΩSETPAPERCLIPPRODUCTION: instruction = instruction;

    pub const addbl: instruction = instruction;
    pub const ADDBL: instruction = instruction;

    pub const subbl: instruction = instruction;
    pub const SUBBL: instruction = instruction;

    pub const mulbl: instruction = instruction;
    pub const MULBL: instruction = instruction;

    pub const divbl: instruction = instruction;
    pub const DIVBL: instruction = instruction;

    pub const modbl: instruction = instruction;
    pub const MODBL: instruction = instruction;

    pub const notl: instruction = instruction;
    pub const NOTL: instruction = instruction;

    pub const andbl: instruction = instruction;
    pub const ANDBL: instruction = instruction;

    pub const orbl: instruction = instruction;
    pub const ORBL: instruction = instruction;

    pub const xorbl: instruction = instruction;
    pub const XORBL: instruction = instruction;

    pub const cmplb: instruction = instruction;
    pub const CMPLB: instruction = instruction;

    pub const tgflag: instruction = instruction;
    pub const TGFLAG: instruction = instruction;

    pub const clflag: instruction = instruction;
    pub const CLFLAG: instruction = instruction;

    pub const addf: instruction = instruction;
    pub const ADDF: instruction = instruction;

    pub const subf: instruction = instruction;
    pub const SUBF: instruction = instruction;

    pub const mulf: instruction = instruction;
    pub const MULF: instruction = instruction;

    pub const divf: instruction = instruction;
    pub const DIVF: instruction = instruction;

    pub const modf: instruction = instruction;
    pub const MODF: instruction = instruction;

    pub const stackalloc: instruction = instruction;
    pub const STACKALLOC: instruction = instruction;

    pub const stackdealloc: instruction = instruction;
    pub const STACKDEALLOC: instruction = instruction;

    pub const push: instruction = instruction;
    pub const PUSH: instruction = instruction;

    pub const pushi: instruction = instruction;
    pub const PUSHI: instruction = instruction;

    pub const pop: instruction = instruction;
    pub const POP: instruction = instruction;

    pub const popa: instruction = instruction;
    pub const POPA: instruction = instruction;

    pub const pusha: instruction = instruction;
    pub const PUSHA: instruction = instruction;

    pub const popb: instruction = instruction;
    pub const POPB: instruction = instruction;

    pub const pushb: instruction = instruction;
    pub const PUSHB: instruction = instruction;

    pub const popl: instruction = instruction;
    pub const POPL: instruction = instruction;

    pub const pushl: instruction = instruction;
    pub const PUSHL: instruction = instruction;

    pub const popf: instruction = instruction;
    pub const POPF: instruction = instruction;

    pub const pushf: instruction = instruction;
    pub const PUSHF: instruction = instruction;

    pub const popch: instruction = instruction;
    pub const POPCH: instruction = instruction;

    pub const pushch: instruction = instruction;
    pub const PUSHCH: instruction = instruction;

    pub const popnum: instruction = instruction;
    pub const POPNUM: instruction = instruction;

    pub const pushnum: instruction = instruction;
    pub const PUSHNUM: instruction = instruction;

    pub const popep: instruction = instruction;
    pub const POPEP: instruction = instruction;

    pub const zpopep: instruction = instruction;
    pub const ZPOPEP: instruction = instruction;

    pub const ppopep: instruction = instruction;
    pub const PPOPEP: instruction = instruction;

    pub const npopep: instruction = instruction;
    pub const NPOPEP: instruction = instruction;

    pub const fpopep: instruction = instruction;
    pub const FPOPEP: instruction = instruction;

    pub const dpopep: instruction = instruction;
    pub const DPOPEP: instruction = instruction;

    pub const getchar: instruction = instruction;
    pub const GETCHAR: instruction = instruction;

    pub const getline: instruction = instruction;
    pub const GETLINE: instruction = instruction;

    pub const writechar: instruction = instruction;
    pub const WRITECHAR: instruction = instruction;

    pub const writelineß: instruction = instruction;
    pub const WRITELINEß: instruction = instruction;

    pub const writeline: instruction = instruction;
    pub const WRITELINE: instruction = instruction;

    pub const toggledebug: instruction = instruction;
    pub const TOGGLEDEBUG: instruction = instruction;

    pub const debugmachinestate: instruction = instruction;
    pub const DEBUGMACHINESTATE: instruction = instruction;

    pub const debugmachinestatecompact: instruction = instruction;
    pub const DEBUGMACHINESTATECOMPACT: instruction = instruction;

    pub const debugmemoryregion: instruction = instruction;
    pub const DEBUGMEMORYREGION: instruction = instruction;

    pub const debugstackregion: instruction = instruction;
    pub const DEBUGSTACKREGION: instruction = instruction;
}

/// Assembly compiler for esoteric VM.
///
/// Input format: `<n>: <inst> <arg1?>, <arg2?>;`, with:
/// - `<n>:` being an optional helper prefix (usually an integer
///     to denote he instruction's location in memory),\
/// - `<inst>` being the instruction, and \
/// - `<argx>` being the argument (usually a number).
///
/// # Examples
///
/// ```rust
/// # use esoteric_vm::{esoteric_assembly, Machine};
///
/// let mut machine = Machine::default();
///
/// let assembly = esoteric_assembly!{
///     // you can prefix the instruction with an integer
///     // and use it to know where instructions are in memory,
///     // but it's not checked so be careful.
///     0: pushi 10;
///     2: pushi 25;
///
///     // you can omit the prefix
///     popl;
///
///     // you can set the prefix to an arbitrary amount
///     1_000_000: Ωtheendisnear;
///     // you can use any literal as the prefix
///     "hello": ΩSKIPTOTHECHASE;
/// };
///
/// machine.load(&assembly, 0);
/// ```
#[macro_export]
#[allow(clippy::module_name_repetitions)]
macro_rules! esoteric_assembly {
    () => { [] as [$crate::instruction::DataOrInstruction; 0] };

    ({} data $data:expr) => { $crate::instruction::DataOrInstruction::Data($data as &[u8]) };
    ({} DATA $data:expr) => { $crate::instruction::DataOrInstruction::Data($data as &[u8]) };

    ({} byte $data:expr) => { $crate::instruction::DataOrInstruction::ByteData($data as u8) };
    ({} BYTE $data:expr) => { $crate::instruction::DataOrInstruction::ByteData($data as u8) };

    ({} nop) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Nop) };
    ({} NOP) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Nop) };

    ({} ldar $data:expr) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Ldar($data)) };
    ({} LDAR $data:expr) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Ldar($data)) };

    ({} ldar) => { compile_error!("missing argument for `ldar` instruction."); };
    ({} LDAR) => { compile_error!("missing argument for `ldar` instruction."); };

    ({} sba) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Sba) };
    ({} SBA) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Sba) };

    ({} clř) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Clř) };
    ({} CLŘ) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Clř) };

    ({} dumpř $data:expr) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Dumpř($data)) };
    ({} DUMPŘ $data:expr) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Dumpř($data)) };

    ({} dumpř) => { compile_error!("missing argument for `dumpř` instruction."); };
    ({} DUMPŘ) => { compile_error!("missing argument for `dumpř` instruction."); };

    ({} movař $data:expr) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Movař($data)) };
    ({} MOVAŘ $data:expr) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Movař($data)) };

    ({} movař) => { compile_error!("missing argument for `movař` instruction."); };
    ({} MOVAŘ) => { compile_error!("missing argument for `movař` instruction."); };

    ({} setř $data0:expr, $data1:expr) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Setř($data0, $data1)) };
    ({} SETŘ $data0:expr, $data1:expr) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Setř($data0, $data1)) };

    ({} setř) => { compile_error!("missing arguments for `setř` instruction."); };
    ({} SETŘ) => { compile_error!("missing arguments for `setř` instruction."); };
    ({} setř $data:expr) => { compile_error!("missing argument for `setř` instruction."); };
    ({} SETŘ $data:expr) => { compile_error!("missing argument for `setř` instruction."); };

    ({} setiř $data0:expr, $data1:expr) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Setiř($data0, $data1)) };
    ({} SETIŘ $data0:expr, $data1:expr) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Setiř($data0, $data1)) };

    ({} setiř) => { compile_error!("missing arguments for `setiř` instruction."); };
    ({} SETIŘ) => { compile_error!("missing arguments for `setiř` instruction."); };
    ({} setiř $data:expr) => { compile_error!("missing argument for `setiř` instruction."); };
    ({} SETIŘ $data:expr) => { compile_error!("missing argument for `setiř` instruction."); };

    ({} ldř $data:expr) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Ldř($data)) };
    ({} LDŘ $data:expr) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Ldř($data)) };

    ({} ldř) => { compile_error!("missing argument for `ldř` instruction."); };
    ({} LDŘ) => { compile_error!("missing argument for `ldř` instruction."); };

    ({} ldiř $data:expr) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Ldiř($data)) };
    ({} LDIŘ $data:expr) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Ldiř($data)) };

    ({} ldiř) => { compile_error!("missing argument for `ldiř` instruction."); };
    ({} LDIŘ) => { compile_error!("missing argument for `ldiř` instruction."); };

    ({} clß) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Clß) };
    ({} CLß) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Clß) };

    ({} dumpß $data:expr) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Dumpß($data)) };
    ({} DUMPß $data:expr) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Dumpß($data)) };

    ({} dumpß) => { compile_error!("missing argument for `dumpß` instruction."); };
    ({} DUMPß) => { compile_error!("missing argument for `dumpß` instruction."); };

    ({} writeß $data0:expr, $data1:expr) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Writeß($data0, $data1)) };
    ({} WRITEß $data0:expr, $data1:expr) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Writeß($data0, $data1)) };

    ({} writeß) => { compile_error!("missing arguments for `writeß` instruction."); };
    ({} WRITEß) => { compile_error!("missing arguments for `writeß` instruction."); };
    ({} writeß $data:expr) => { compile_error!("missing argument for `writeß` instruction."); };
    ({} WRITEß $data:expr) => { compile_error!("missing argument for `writeß` instruction."); };

    ({} movaß $data:expr) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Movaß($data)) };
    ({} MOVAß $data:expr) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Movaß($data)) };

    ({} movaß) => { compile_error!("missing argument for `movaß` instruction."); };
    ({} MOVAß) => { compile_error!("missing argument for `movaß` instruction."); };

    ({} setß $data0:expr, $data1:expr) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Setß($data0, $data1)) };
    ({} SETß $data0:expr, $data1:expr) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Setß($data0, $data1)) };

    ({} setß) => { compile_error!("missing arguments for `setß` instruction."); };
    ({} SETß) => { compile_error!("missing arguments for `setß` instruction."); };
    ({} setß $data:expr) => { compile_error!("missing argument for `setß` instruction."); };
    ({} SETß $data:expr) => { compile_error!("missing argument for `setß` instruction."); };

    ({} setiß $data0:expr, $data1:expr) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Setiß($data0, $data1)) };
    ({} SETIß $data0:expr, $data1:expr) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Setiß($data0, $data1)) };

    ({} setiß) => { compile_error!("missing arguments for `setiß` instruction."); };
    ({} SETIß) => { compile_error!("missing arguments for `setiß` instruction."); };
    ({} setiß $data:expr) => { compile_error!("missing argument for `setiß` instruction."); };
    ({} SETIß $data:expr) => { compile_error!("missing argument for `setiß` instruction."); };

    ({} ldß $data:expr) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Ldß($data)) };
    ({} LDß $data:expr) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Ldß($data)) };

    ({} ldß) => { compile_error!("missing argument for `ldß` instruction."); };
    ({} LDß) => { compile_error!("missing argument for `ldß` instruction."); };

    ({} pushß) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Pushß) };
    ({} PUSHß) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Pushß) };

    ({} popß) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Popß) };
    ({} POPß) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Popß) };

    ({} lenßa) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Lenßa) };
    ({} LENßA) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Lenßa) };

    ({} ldidp $data:expr) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Ldidp($data)) };
    ({} LDIDP $data:expr) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Ldidp($data)) };

    ({} ldidp) => { compile_error!("missing argument for `ldidp` instruction."); };
    ({} LDIDP) => { compile_error!("missing argument for `ldidp` instruction."); };

    ({} Ωchoiceset $data:expr) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::ΩChoiceSet($data)) };
    ({} ΩCHOICESET $data:expr) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::ΩChoiceSet($data)) };

    ({} Ωchoiceset) => { compile_error!("missing argument for `Ωchoiceset` instruction."); };
    ({} ΩCHOICESET) => { compile_error!("missing argument for `Ωchoiceset` instruction."); };

    ({} Ωchoicegeta) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::ΩChoiceGetA) };
    ({} ΩCHOICEGETA) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::ΩChoiceGetA) };

    ({} Ωgainapolymorphicdesires) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::ΩGainAPolymorphicDesires) };
    ({} ΩGAINAPOLYMORPHICDESIRES) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::ΩGainAPolymorphicDesires) };

    ({} Ωloseapolymorphicdesires) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::ΩLoseAPolymorphicDesires) };
    ({} ΩLOSEAPOLYMORPHICDESIRES) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::ΩLoseAPolymorphicDesires) };

    ({} Ωpushpolymorphicdesires) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::ΩPushPolyMorphicDesires) };
    ({} ΩPUSHPOLYMORPHICDESIRES) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::ΩPushPolyMorphicDesires) };

    ({} Ωtheendisnear) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::ΩTheEndIsNear) };
    ({} ΩTHEENDISNEAR) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::ΩTheEndIsNear) };

    ({} Ωskiptothechase) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::ΩSkipToTheChase) };
    ({} ΩSKIPTOTHECHASE) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::ΩSkipToTheChase) };

    ({} Ωsetsentience $data:expr) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::ΩSetSentience($data)) };
    ({} ΩSETSENTIENCE $data:expr) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::ΩSetSentience($data)) };

    ({} Ωsetsentience) => { compile_error!("missing argument for `Ωsetsentience` instruction."); };
    ({} ΩSETSENTIENCE) => { compile_error!("missing argument for `Ωsetsentience` instruction."); };

    ({} Ωsetpaperclipproduction $data:expr) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::ΩSetPaperclipProduction($data)) };
    ({} ΩSETPAPERCLIPPRODUCTION $data:expr) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::ΩSetPaperclipProduction($data)) };

    ({} Ωsetpaperclipproduction) => { compile_error!("missing argument for `Ωsetpaperclipproduction` instruction."); };
    ({} ΩSETPAPERCLIPPRODUCTION) => { compile_error!("missing argument for `Ωsetpaperclipproduction` instruction."); };

    ({} addbl) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::AddBL) };
    ({} ADDBL) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::AddBL) };

    ({} subbl) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::SubBL) };
    ({} SUBBL) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::SubBL) };

    ({} mulbl) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::MulBL) };
    ({} MULBL) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::MulBL) };

    ({} divbl) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::DivBL) };
    ({} DIVBL) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::DivBL) };

    ({} modbl) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::ModBL) };
    ({} MODBL) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::ModBL) };

    ({} notl) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::NotL) };
    ({} NOTL) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::NotL) };

    ({} andbl) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::AndBL) };
    ({} ANDBL) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::AndBL) };

    ({} orbl) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::OrBL) };
    ({} ORBL) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::OrBL) };

    ({} xorbl) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::XorBL) };
    ({} XORBL) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::XorBL) };

    ({} cmplb) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::CmpLB) };
    ({} CMPLB) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::CmpLB) };

    ({} tgflag) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::TgFlag) };
    ({} TGFLAG) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::TgFlag) };

    ({} clflag) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::ClFlag) };
    ({} CLFLAG) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::ClFlag) };

    ({} addf $data:expr) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::AddF($data)) };
    ({} ADDF $data:expr) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::AddF($data)) };

    ({} addf) => { compile_error!("missing argument for `addf` instruction."); };
    ({} ADDF) => { compile_error!("missing argument for `addf` instruction."); };

    ({} subf $data:expr) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::SubF($data)) };
    ({} SUBF $data:expr) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::SubF($data)) };

    ({} subf) => { compile_error!("missing argument for `subf` instruction."); };
    ({} SUBF) => { compile_error!("missing argument for `subf` instruction."); };

    ({} mulf $data:expr) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::MulF($data)) };
    ({} MULF $data:expr) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::MulF($data)) };

    ({} mulf) => { compile_error!("missing argument for `mulf` instruction."); };
    ({} MULF) => { compile_error!("missing argument for `mulf` instruction."); };

    ({} divf $data:expr) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::DivF($data)) };
    ({} DIVF $data:expr) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::DivF($data)) };

    ({} divf) => { compile_error!("missing argument for `divf` instruction."); };
    ({} DIVF) => { compile_error!("missing argument for `divf` instruction."); };

    ({} modf $data:expr) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::ModF($data)) };
    ({} MODF $data:expr) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::ModF($data)) };

    ({} modf) => { compile_error!("missing argument for `modf` instruction."); };
    ({} MODF) => { compile_error!("missing argument for `modf` instruction."); };

    ({} stackalloc $data:expr) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::StackAlloc($data)) };
    ({} STACKALLOC $data:expr) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::StackAlloc($data)) };

    ({} stackalloc) => { compile_error!("missing argument for `stackalloc` instruction."); };
    ({} STACKALLOC) => { compile_error!("missing argument for `stackalloc` instruction."); };

    ({} stackdealloc $data:expr) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::StackDealloc($data)) };
    ({} STACKDEALLOC $data:expr) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::StackDealloc($data)) };

    ({} stackdealloc) => { compile_error!("missing argument for `stackdealloc` instruction."); };
    ({} STACKDEALLOC) => { compile_error!("missing argument for `stackdealloc` instruction."); };

    ({} push $data:expr) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Push($data)) };
    ({} PUSH $data:expr) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Push($data)) };

    ({} push) => { compile_error!("missing argument for `push` instruction."); };
    ({} PUSH) => { compile_error!("missing argument for `push` instruction."); };

    ({} pushi $data:expr) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Pushi($data)) };
    ({} PUSHI $data:expr) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Pushi($data)) };

    ({} pushi) => { compile_error!("missing argument for `pushi` instruction."); };
    ({} PUSHI) => { compile_error!("missing argument for `pushi` instruction."); };

    ({} pop $data:expr) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Pop($data)) };
    ({} POP $data:expr) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Pop($data)) };

    ({} pop) => { compile_error!("missing argument for `pop` instruction."); };
    ({} POP) => { compile_error!("missing argument for `pop` instruction."); };

    ({} popa) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Popa) };
    ({} POPA) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Popa) };

    ({} pusha) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Pusha) };
    ({} PUSHA) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Pusha) };

    ({} popb) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Popb) };
    ({} POPB) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Popb) };

    ({} pushb) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Pushb) };
    ({} PUSHB) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Pushb) };

    ({} popl) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::PopL) };
    ({} POPL) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::PopL) };

    ({} pushl) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::PushL) };
    ({} PUSHL) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::PushL) };

    ({} popf) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Popf) };
    ({} POPF) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Popf) };

    ({} pushf) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Pushf) };
    ({} PUSHF) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Pushf) };

    ({} popch) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Popch) };
    ({} POPCH) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Popch) };

    ({} pushch) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Pushch) };
    ({} PUSHCH) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Pushch) };

    ({} popnum) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Popnum) };
    ({} POPNUM) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Popnum) };

    ({} pushnum) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Pushnum) };
    ({} PUSHNUM) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Pushnum) };

    ({} popep) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Popep) };
    ({} POPEP) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Popep) };

    ({} zpopep) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Zpopep) };
    ({} ZPOPEP) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Zpopep) };

    ({} ppopep) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Ppopep) };
    ({} PPOPEP) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Ppopep) };

    ({} npopep) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Npopep) };
    ({} NPOPEP) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Npopep) };

    ({} fpopep) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Fpopep) };
    ({} FPOPEP) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Fpopep) };

    ({} dpopep) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Dpopep) };
    ({} DPOPEP) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::Dpopep) };

    ({} getchar) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::GetChar) };
    ({} GETCHAR) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::GetChar) };

    ({} getline) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::GetLine) };
    ({} GETLINE) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::GetLine) };

    ({} writechar) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::WriteChar) };
    ({} WRITECHAR) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::WriteChar) };

    ({} writelineß) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::WriteLineß) };
    ({} WRITELINEß) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::WriteLineß) };

    ({} writeline $data:expr) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::WriteLine($data)) };
    ({} WRITELINE $data:expr) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::WriteLine($data)) };

    ({} writeline) => { compile_error!("missing argument for `writeline` instruction."); };
    ({} WRITELINE) => { compile_error!("missing argument for `writeline` instruction."); };

    ({} toggledebug) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::ToggleDebug) };
    ({} TOGGLEDEBUG) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::ToggleDebug) };

    ({} debugmachinestate) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::DebugMachineState) };
    ({} DEBUGMACHINESTATE) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::DebugMachineState) };

    ({} debugmachinestatecompact) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::DebugMachineStateCompact) };
    ({} DEBUGMACHINESTATECOMPACT) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::DebugMachineStateCompact) };

    ({} debugmemoryregion $data0:expr, $data1:expr) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::DebugMemoryRegion($data0, $data1)) };
    ({} DEBUGMEMORYREGION $data0:expr, $data1:expr) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::DebugMemoryRegion($data0, $data1)) };

    ({} debugmemoryregion) => { compile_error!("missing arguments for `debugmemoryregion` instruction."); };
    ({} DEBUGMEMORYREGION) => { compile_error!("missing arguments for `debugmemoryregion` instruction."); };
    ({} debugmemoryregion $data:expr) => { compile_error!("missing argument for `debugmemoryregion` instruction."); };
    ({} DEBUGMEMORYREGION $data:expr) => { compile_error!("missing argument for `debugmemoryregion` instruction."); };

    ({} debugstackregion $data0:expr, $data1:expr) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::DebugStackRegion($data0, $data1)) };
    ({} DEBUGSTACKREGION $data0:expr, $data1:expr) => { $crate::instruction::DataOrInstruction::Instruction($crate::instruction::Instruction::DebugStackRegion($data0, $data1)) };

    ({} debugstackregion) => { compile_error!("missing arguments for `debugstackregion` instruction."); };
    ({} DEBUGSTACKREGION) => { compile_error!("missing arguments for `debugstackregion` instruction."); };
    ({} debugstackregion $data:expr) => { compile_error!("missing argument for `debugstackregion` instruction."); };
    ({} DEBUGSTACKREGION $data:expr) => { compile_error!("missing argument for `debugstackregion` instruction."); };


    ({} $($trash:tt)*) => { compile_error!(concat!("`", stringify!($($trash)*), "` isn't a valid esoteric assembly instruction")) };

    ($($($n:literal:)? $name:ident $($value:expr),*);* $(;)?) => {{
        $(
            #[cfg(not(any(debug_assertions, not(debug_assertions))))] // never compile
            use $crate::assembly::__instructions::$name;
        )*

        [ $(
                $crate::esoteric_assembly!({} $name $($value),*),
        )* ]
    }};

}
