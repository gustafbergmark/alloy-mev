use alloy::signers::Signer;
use alloy::transports::BoxTransport;
use url::Url;

use super::bundle_signer::BundleSigner;

/// An Alloy `Transport` that add a signature in the headers for `mev_*` and
/// `eth_*` requests and delegates all others to the inner [`Transport`].
#[derive(Debug, Clone)]
pub struct MevHttp {
    /// The endpoint to send requests.
    pub endpoint: Url,
    /// The inner transport to send non-MEV requests.
    pub transport: BoxTransport,
    /// The signer used to build bundles signatures.
    pub bundle_signer: BundleSigner,
}

impl MevHttp {
    /// Creates a new [`MevHttp`] transport.
    pub const fn new(endpoint: Url, transport: BoxTransport, bundle_signer: BundleSigner) -> Self {
        Self {
            endpoint,
            transport,
            bundle_signer,
        }
    }

    /// Creates a transport to send requests to flashbots.
    pub fn flashbots<S>(transport: BoxTransport, signer: S) -> Self
    where
        S: Signer + Send + Sync + 'static,
    {
        Self {
            endpoint: "https://relay.flashbots.net".parse().unwrap(),
            transport,
            bundle_signer: BundleSigner::flashbots(Box::new(signer)),
        }
    }
}
