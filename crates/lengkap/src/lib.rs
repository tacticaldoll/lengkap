//! Lengkap: a sans-I/O all-of evidence completion core.
//!
//! This crate is the curated public entrypoint. It carries no logic and
//! re-exports the complete [`lengkap_contract`] API.
//!
//! ```
//! use lengkap::{
//!     Assembly, Decision, Finding, LocatedFinding, Slot, adjudicate,
//! };
//!
//! let decision = adjudicate(
//!     Assembly::new(2),
//!     [
//!         LocatedFinding::<_, &str>::new(
//!             Slot::new(1),
//!             Finding::Produced("second"),
//!         ),
//!         LocatedFinding::new(
//!             Slot::new(0),
//!             Finding::Produced("first"),
//!         ),
//!     ],
//! )
//! .expect("both slots are valid");
//!
//! assert_eq!(decision, Decision::Ready(vec!["first", "second"]));
//! ```

#![no_std]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub use lengkap_contract::*;
