// Copyright 2023, Offchain Labs, Inc.
// For licensing, see https://github.com/OffchainLabs/stylus-sdk-rs/blob/stylus/licenses/COPYRIGHT.md

//! VM affordances for inspecting the current call.
//!
//! See also [`block`](crate::block), [`contract`](crate::contract), [`evm`](crate::evm),
//! [`msg`](crate::msg), and [`tx`](crate::tx).
//!
//! ```no_run
//! use stylus_sdk::msg;
//!
//! let call_value = msg::value();
//! ```

use crate::hostio::{self, wrap_hostio};
use alloy_primitives::{Address, B256, U256};

wrap_hostio!(
    /// Whether the current call is reentrant.
    /// CHRIS: Could perhaps be named is_reentrant, is also fine as is. Only confusion could be thinking that
    /// reentrant refers to the party that is re-entering, or the function that is being reentered from?
    /// But I think those are unlikely/not sensible confusions.
    reentrant REENTRANT msg_reentrant bool
);

wrap_hostio!(
    /// Gets the address of the account that called the program. For normal L2-to-L2 transactions
    /// the semantics are equivalent to that of the EVM's [`CALLER`] opcode, including in cases
    /// arising from [`DELEGATE_CALL`].
    ///
    /// For L1-to-L2 retryable ticket transactions, the top-level sender's address will be aliased.
    /// See [`Retryable Ticket Address Aliasing`] for more information on how this works.
    ///
    /// [`CALLER`]: https://www.evm.codes/#33
    /// [`DELEGATE_CALL`]: https://www.evm.codes/#f4
    /// [`Retryable Ticket Address Aliasing`]: https://developer.arbitrum.io/arbos/l1-to-l2-messaging#address-aliasing
    sender SENDER msg_sender Address
);

wrap_hostio!(
    /// Get the ETH value in wei sent to the program.
    value VALUE msg_value U256
);

// CHRIS: is there a missing data() func for accessing all the call data? This is quite commonly used