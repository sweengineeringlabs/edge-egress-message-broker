//! Gateway layer — public surface for the egress message-broker transport crate.
pub(crate) mod input;
pub(crate) mod output;

pub use output::*;
