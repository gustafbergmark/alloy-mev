#![doc = include_str!("../README.md")]
#![warn(
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    unreachable_pub,
    clippy::missing_const_for_fn,
    rustdoc::all
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]

mod eth;
pub use eth::{BroadcastableCall, Endpoints, EndpointsBuilder, EthMevProviderExt, EthBundle, EthereumReqwestEthBundle};

mod mev_share;
pub use mev_share::{MevShareProviderExt, EthereumReqwestMevShareBundle, MevShareBundle};

mod transport;
pub use transport::{BundleSigner, MevHttp};
