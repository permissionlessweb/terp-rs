//! cw-orch interface for the Crosslink IBC light client.
//!
//! Feature-gated behind `interface`. Provides `CrosslinkLightClient<Chain>`
//! for use in cw-orchestrator test suites and deployment scripts.

use cosmwasm_std::Empty;
use cw_orch::{interface, prelude::*};

use crate::contract::{CONTRACT_NAME, execute, instantiate, migrate, query, sudo};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

/// cw-orch interface for the crosslink light client contract.
#[interface(InstantiateMsg, ExecuteMsg, QueryMsg, Empty, id = CONTRACT_NAME )]
pub struct CrosslinkLightClient;

impl<Chain: TxHandler> Uploadable for CrosslinkLightClient<Chain> {
    /// Return the path to the compiled WASM artifact.
    fn wasm(_chain: &ChainInfoOwned) -> WasmPath {
        let debug_path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../target/wasm32-unknown-unknown/debug/cw_ics08_wasm_crosslink.wasm"
        );
        let release_path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../target/wasm32-unknown-unknown/release/cw_ics08_wasm_crosslink.wasm"
        );

        if std::path::Path::new(release_path).exists() {
            WasmPath::new(release_path).expect("release WASM path")
        } else {
            WasmPath::new(debug_path).expect("debug WASM path")
        }
    }

    /// Return a mock contract wrapper for unit testing.
    fn wrapper() -> Box<dyn MockContract<Empty, Empty>> {
        Box::new(
            ContractWrapper::new_with_empty(execute, instantiate, query)
                .with_sudo(sudo)
                .with_migrate(migrate),
        )
    }
}
