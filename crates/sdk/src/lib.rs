//! terp-rs: Prost/Tonic type definitions for the terp-core Cosmos SDK chain.
//!
//! Generated modules live in `src/gen/` and are re-exported in a nested module
//! tree that matches the proto package hierarchy (required for `super::` refs).

#![allow(
    clippy::all,
    rustdoc::broken_intra_doc_links,
    non_camel_case_types,
    unused_imports
)]

pub use prost::{Message, Name};
// ---------------------------------------------------------------------------
// PyO3 Python bindings (enabled with `--features python`)
// ---------------------------------------------------------------------------

#[cfg(feature = "python")]
mod py;

// ---------------------------------------------------------------------------
// Generated proto modules — nested to satisfy prost's `super::` cross-refs
// ---------------------------------------------------------------------------

pub mod cosmos_proto {
    include!("gen/cosmos_proto.rs");
}

pub mod cosmos {
    pub mod auth {
        pub mod v1beta1 {
            include!("gen/cosmos.auth.v1beta1.rs");
        }
    }
    pub mod bank {
        pub mod v1beta1 {
            include!("gen/cosmos.bank.v1beta1.rs");
        }
    }
    pub mod base {
        pub mod query {
            pub mod v1beta1 {
                include!("gen/cosmos.base.query.v1beta1.rs");
            }
        }
        pub mod v1beta1 {
            include!("gen/cosmos.base.v1beta1.rs");
        }
    }
    pub mod ics23 {
        pub mod v1 {
            include!("gen/cosmos.ics23.v1.rs");
        }
    }
    pub mod upgrade {
        pub mod v1beta1 {
            include!("gen/cosmos.upgrade.v1beta1.rs");
        }
    }
}

pub mod gaia {
    pub mod globalfee {
        pub mod v1beta1 {
            include!("gen/gaia.globalfee.v1beta1.rs");
        }
    }
}

pub mod google {
    pub mod api {
        include!("gen/google.api.rs");
    }
}

pub mod ibc {
    pub use ibc_proto::google::protobuf::Any;
    pub use ibc_proto::ics23::{self};
    #[cfg(feature = "ibc")]
    pub mod crates {}

    // pub use ibc_app_nft_transfer_types::{self};
    // pub use ibc_app_transfer_types::{self};
    // // pub use ibc_client_wasm_types::{self};

    // pub use ibc_types::{
    //     self,
    //     core::{self as ibc_core},
    //     lightclients::{self as lightclient},
    //     timestamp::{self},
    //     transfer::acknowledgement::{self},
    // };

    pub mod applications {
        pub mod gmp {
            pub mod v1 {
                include!("gen/ibc.applications.gmp.v1.rs");
            }
        }
        pub mod interchain_accounts {
            pub mod controller {
                pub mod v1 {
                    include!("gen/ibc.applications.interchain_accounts.controller.v1.rs");
                }
            }
            pub mod genesis {
                pub mod v1 {
                    include!("gen/ibc.applications.interchain_accounts.genesis.v1.rs");
                }
            }
            pub mod host {
                pub mod v1 {
                    include!("gen/ibc.applications.interchain_accounts.host.v1.rs");
                }
            }
            pub mod v1 {
                include!("gen/ibc.applications.interchain_accounts.v1.rs");
            }
        }
        pub mod packet_forward_middleware {
            pub mod v1 {
                include!("gen/ibc.applications.packet_forward_middleware.v1.rs");
            }
        }
        pub mod rate_limiting {
            pub mod v1 {
                include!("gen/ibc.applications.rate_limiting.v1.rs");
            }
        }
        pub mod transfer {
            pub mod v1 {
                include!("gen/ibc.applications.transfer.v1.rs");
            }
        }
    }
    pub mod v2 {}
    pub mod core {
        pub mod channel {
            pub mod v1 {
                include!("gen/ibc.core.channel.v1.rs");
            }
            pub mod v2 {
                include!("gen/ibc.core.channel.v2.rs");
            }
        }
        pub mod client {
            pub mod v1 {
                include!("gen/ibc.core.client.v1.rs");
            }
            pub mod v2 {
                include!("gen/ibc.core.client.v2.rs");
            }
        }
        pub mod commitment {
            pub mod v1 {
                include!("gen/ibc.core.commitment.v1.rs");
            }
            pub mod v2 {
                include!("gen/ibc.core.commitment.v2.rs");
            }
        }
        pub mod connection {
            pub mod v1 {
                include!("gen/ibc.core.connection.v1.rs");
            }
        }
        pub mod types {
            pub mod v1 {
                include!("gen/ibc.core.types.v1.rs");
            }
        }
    }
    pub mod lightclients {
        pub mod attestations {
            pub mod v1 {
                include!("gen/ibc.lightclients.attestations.v1.rs");
            }
        }
        pub mod solomachine {
            pub mod v2 {
                include!("gen/ibc.lightclients.solomachine.v2.rs");
            }
            pub mod v3 {
                include!("gen/ibc.lightclients.solomachine.v3.rs");
            }
        }
        pub mod tendermint {
            pub mod v1 {
                include!("gen/ibc.lightclients.tendermint.v1.rs");
            }
        }
        pub mod wasm {
            pub mod v1 {
                include!("gen/ibc.lightclients.wasm.v1.rs");
            }
        }
    }
}

pub mod osmosis {
    pub mod tokenfactory {
        pub mod v1beta1 {
            include!("gen/osmosis.tokenfactory.v1beta1.rs");
        }
    }
}

pub mod tendermint {
    pub mod crypto {
        include!("gen/tendermint.crypto.rs");
    }
    pub mod types {
        include!("gen/tendermint.types.rs");
    }
    pub mod version {
        include!("gen/tendermint.version.rs");
    }
}

pub mod terp {
    pub mod clock {
        pub mod v1 {
            include!("gen/terp.clock.v1.rs");
        }
    }
    pub mod drip {
        pub mod v1 {
            include!("gen/terp.drip.v1.rs");
        }
    }
    pub mod feeshare {
        pub mod v1 {
            include!("gen/terp.feeshare.v1.rs");
        }
    }
    pub mod smartaccount {
        pub mod v1beta1 {
            include!("gen/terp.smartaccount.v1beta1.rs");
        }
    }
    pub mod hashmerchant {
        pub mod v1beta1 {
            include!("gen/terp.hashmerchant.v1.rs");
        }
    }
}
