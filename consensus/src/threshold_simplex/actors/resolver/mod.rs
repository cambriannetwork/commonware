mod actor;
mod ingress;

use crate::Supervisor;
pub use actor::Actor;
use commonware_cryptography::PublicKey;
use commonware_p2p::Blocker;
use governor::Quota;
pub use ingress::Mailbox;
#[cfg(test)]
pub use ingress::Message;
use std::time::Duration;

pub struct Config<C: PublicKey, B: Blocker, S: Supervisor> {
    pub crypto: C,
    pub blocker: B,
    pub supervisor: S,

    pub namespace: Vec<u8>,
    pub mailbox_size: usize,
    pub activity_timeout: u64,
    pub fetch_timeout: Duration,
    pub max_fetch_count: usize,
    pub fetch_rate_per_peer: Quota,
    pub fetch_concurrent: usize,
}
