#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(
    rustdoc::bare_urls,
    rustdoc::broken_intra_doc_links,
    clippy::derive_partial_eq_without_eq
)]
#![forbid(unsafe_code)]
#![warn(trivial_casts, trivial_numeric_casts, unused_import_braces)]

pub mod traits;
mod type_urls;

pub use prost;
pub use prost_types::Any;
/// The version (commit hash) of the Cosmos SDK used when generating this library.
pub const COSMOS_SDK_VERSION: &str = include_str!("prost/cosmos-sdk/COSMOS_SDK_COMMIT");

pub mod cosmos {
    /// Balances.
    pub mod bank {
        pub mod v1beta1 {
            include!("prost/cosmos-sdk/cosmos.bank.v1beta1.rs");
        }
    }

    pub mod base {
        pub mod v1beta1 {
            include!("prost/cosmos-sdk/cosmos.base.v1beta1.rs");
        }

        /// Query support.
        pub mod query {
            pub mod v1beta1 {
                include!("prost/cosmos-sdk/cosmos.base.query.v1beta1.rs");
            }
        }
    }

    pub mod distribution {
        pub mod v1beta1 {
            include!("prost/cosmos-sdk/cosmos.distribution.v1beta1.rs");
        }
    }

    pub mod staking {
        pub mod v1beta1 {
            include!("prost/cosmos-sdk/cosmos.staking.v1beta1.rs");
        }
    }
}

pub mod cosmwasm {
    /// Messages and services handling CosmWasm.
    pub mod wasm {
        pub mod v1 {
            include!("prost/wasmd/cosmwasm.wasm.v1.rs");
        }
    }
}

pub mod ibc {
    pub mod applications {
        pub mod interchain_accounts {
            pub mod v1 {
                include!("prost/ibc-go/ibc.applications.interchain_accounts.v1.rs");
            }
        }

        pub mod transfer {
            pub mod v1 {
                include!("prost/ibc-go/ibc.applications.transfer.v1.rs");
            }
        }
    }

    /// IBC core.
    pub mod core {
        // /// IBC channels.
        // pub mod channel {
        //     pub mod v1 {
        //         include!("prost/ibc-go/ibc.core.channel.v1.rs");
        //     }
        // }

        /// IBC client.
        pub mod client {
            pub mod v1 {
                include!("prost/ibc-go/ibc.core.client.v1.rs");
            }
        }

        // /// IBC commitments.
        // pub mod commitment {
        //     pub mod v1 {
        //         include!("prost/ibc-go/ibc.core.commitment.v1.rs");
        //     }
        // }

        // /// IBC connections.
        // pub mod connection {
        //     pub mod v1 {
        //         include!("prost/ibc-go/ibc.core.connection.v1.rs");
        //     }
        // }

        // /// IBC types.
        // pub mod types {
        //     pub mod v1 {
        //         include!("prost/ibc-go/ibc.core.types.v1.rs");
        //     }
        // }
    }
}
