//! Gateway layer — public surface for the egress message-broker transport crate.
pub(crate) mod egress;
pub(crate) mod ingress;

pub use egress::*;
