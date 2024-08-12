//! An esoteric type.
//!
//! For more information, read the docs for [`Ω`].

use std::io::{self, Write};

/// An esoteric type
#[derive(Debug, Clone)]
pub struct Ω {
    /// The illusion of choice.
    ///
    /// Highlights how ZSTs tend to be useless.
    pub illusion_of_choice: Option<Option<Option<Option<()>>>>,

    /// Polymorphic desires.
    pub polymorphic_desires: u64,

    /// Feeling of impending doom.
    pub feeling_of_impending_doom: bool,

    /// Is sentient.
    ///
    /// Highlights the theory that AI will become sentient.
    pub is_sentient: bool,

    /// Whether infinite paperclips should be produced.
    ///
    /// This is a reference to the game [Universal paperclips](https://www.decisionproblem.com/paperclips/index2.html)
    pub should_make_infinite_paperclips: bool,
}

impl Ω {
    /// This is a zeroed instance of [`Ω`], which replaces a `new()` function.
    pub const ZEROED: Self = Self {
        illusion_of_choice: None,
        polymorphic_desires: 0,
        feeling_of_impending_doom: false,
        is_sentient: false,
        should_make_infinite_paperclips: false,
    };
}

impl Ω {
    /// Write the illusion of choice to the specified buffer.
    ///
    /// # Errors
    ///
    /// Errors if writing to the buffer failed
    pub fn display_illusion_of_choice<W: Write>(&self, f: &mut W) -> io::Result<()> {
        f.write_all(match self.illusion_of_choice {
            Some(Some(Some(Some(())))) => b"Some Something with Some valueless Something",
            Some(Some(Some(None))) => b"Some Something with Some Nothing",
            Some(Some(None)) => b"Some Something with Nothing",
            Some(None) => b"Some Nothing",
            None => b"Nothing",
        })
    }
}
