pub mod environments;
pub mod suite;
pub use environments::{nostr, quickspawn};

pub mod prelude {
    pub use crate::nostr::{NostrClient, NostrEvent, NostrRelayerManager, NostrTestEnv};
    pub use crate::suite::{
        deploy_data::preflight_check, DeploySuite, DockerSidecar, TerpNetworkDeployData,
    };
    // pub use crate::quickspawn::{QuickSpawnEnv, QuickSpawnMultiEnv};
}
