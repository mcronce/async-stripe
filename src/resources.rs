//! resources module
//!
//! This module exposes various elements of the
//! stripe api depending on the features exposed.
//!
//! Some of these modules are hand-written, and
//! some are generated.

#[allow(clippy::module_inception)]
#[allow(clippy::new_without_default)]
pub mod generated;
mod types;

#[path = "resources"]
mod core {
    pub mod payment_source;
}

#[rustfmt::skip]
pub use {
	types::*,
    self::core::{
        payment_source::*,
    },
    generated::core::{
		address::*,
        customer::*,
    },
};
