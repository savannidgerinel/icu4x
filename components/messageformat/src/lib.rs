
// Parse a message into a data structure
//
// Data structure can provide reflection
//
// Data structure can be rendered into a final form
//
// Be explicit about whether we're handling MessageFormat 1 or MessageFormat 2

#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![cfg_attr(
    not(test),
    deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::exhaustive_structs,
        clippy::exhaustive_enums,
        missing_debug_implementations,
    )
)]
#![warn(missing_docs)]

extern crate alloc;

pub mod v1;
