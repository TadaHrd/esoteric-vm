//! TODO: docs

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
