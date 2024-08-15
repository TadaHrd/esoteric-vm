//! An esoteric virtual machine.
//!
//! Create a new machine with [`Machine::default`] and load
//! machine code and data to it with [`Machine::load`].
//!
//! # Examples
//!
//! ```rust
//! use esoteric_vm::{esoteric_assembly, Machine};
//!
//! # fn main() -> Machine {
//! // initialize a new machine
//! let mut machine = Machine::default();
//!
//! // assembly code for the machine
//! let asm = esoteric_assembly! {
//!     // initialize dot pointer so that IO operations work
//!
//!     // push a dot character to stack
//!     0: pushi b'.';
//!     // pop to address 28657
//!     2: pop 28657;
//!
//!     // set dot pointer to 28657 (has to be a prime or semiprime, which is also a fibonacci number)
//!     5: ldidp 28657;
//!
//!     // -----------------
//!
//!     // print hello world
//!     8: writeline 13;
//!
//!     // halt machine
//!     11: Ωtheendisnear;
//!     12: Ωskiptothechase;
//!
//!     // hello world text
//!     13: data b"Hello, world!\n\0";
//! };
//!
//! // load machine code
//! machine.load(&asm, 0);
//!
//! // run machine until it halts
//! machine.run();
//!
//! // return the machine's register A (unused)
//! machine
//! # }
//! ```

#![warn(
    clippy::pedantic,
    clippy::complexity,
    clippy::cognitive_complexity,
    clippy::correctness,
    clippy::nursery,
    clippy::perf,
    clippy::style,
    clippy::suspicious,
    clippy::arithmetic_side_effects,
    clippy::little_endian_bytes,
    clippy::dbg_macro,
    clippy::decimal_literal_representation,
    clippy::exit,
    clippy::expect_used,
    clippy::unwrap_used,
    clippy::host_endian_bytes,
    clippy::if_then_some_else_none,
    clippy::indexing_slicing,
    missing_docs,
    clippy::missing_docs_in_private_items,
    clippy::mixed_read_write_in_expression,
    clippy::multiple_unsafe_ops_per_block,
    clippy::panic,
    clippy::partial_pub_fields,
    clippy::pub_without_shorthand,
    clippy::self_named_module_files,
    clippy::semicolon_inside_block,
    clippy::todo,
    clippy::undocumented_unsafe_blocks,
    clippy::wildcard_enum_match_arm
)]
#![deny(clippy::must_use_candidate, unsafe_op_in_unsafe_fn)]

pub mod instruction;
pub mod machine;
/// Utilities used throughout the crate.
pub(crate) mod utils {
    pub mod array_debug;
    pub mod constant_size_string;
    pub mod multi_index;
    pub mod non_invalidatable;
    pub mod primes;
}
pub mod assembly;

pub use machine::Machine;
