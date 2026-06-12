//! TerpNetworkSuite — contract deployment suite for Terp Network.
//!
//! Composes cw-orch sub-suites: DAO, cw-infuser/SVG, headstash/zk,
//! billboards/terp-accounts, and shitstraps — deployed conditionally
//! based on supplied deploy data.
//!
//! Built for integration tests to spin up the full on-chain contract stack
//! on any chain implementing `ZkCwEnv`.

use cw_infuser_scripts::suite::CwSvgSuite;
use cw_orch::prelude::*;
use shit_scripts::CwShitstrapSuite;
use std::collections::HashMap;
use terp_account_scripts::TerpAccountSuite;

// use super::deploy_data::TerpNetworkDeployData;

// /// Unified deployment suite composing all website contract suites.
// pub struct TerpNetworkSuite<Chain: CwEnv>
 
// {
//     pub chain: Chain,
//     pub infuser: Option<CwSvgSuite<Chain>>,
//     pub billboards: Option<TerpAccountSuite<Chain>>,
//     // pub dao: Option<DaoDaoSuite<Chain>>,
//     #[cfg(feature = "zk")]
//     pub zk: Option<zk_test_press::TestPressSuite<Chain>>,
// }

// impl<Chain: ZkCwEnv> TerpNetworkSuite<Chain>
// where
//     CwOrchError: From<<Chain as TxHandler>::Error>,
// {
//     /// Deploy suites conditionally based on which deploy data fields are `Some`.
//     pub fn deploy_on(chain: Chain, data: TerpNetworkDeployData) -> Result<Self, CwOrchError> {
//         // 1. CwSvgSuite (cw-infuser + SVG collections + shitstraps)
//         let infuser = if let Some(svg_data) = data.cw_infuser {
//             let mut suite = CwSvgSuite::deploy_on(chain.clone(), Some(svg_data))?;
//             if let Some(shit_data) = data.shitstraps {
//                 suite.shit = CwShitstrapSuite::deploy_on(chain.clone(), Some(shit_data))?;
//             }
//             Some(suite)
//         } else {
//             None
//         };

//         // 2. TerpAccountSuite (billboards)
//         let billboards = if let Some(admin) = data.terp_billboards {
//             Some(TerpAccountSuite::deploy_on(chain.clone(), admin)?)
//         } else {
//             None
//         };

//         // 4. ZkHeadstashSuite (cw-headstash + cw-headstash-manifold + circuits)
//         let zk = if let Some(zk_data) = data.zk {
//             match TestPressSuite::deploy_on(chain.clone(), zk_data) {
//                 Ok(suite) => Some(suite),
//                 Err(e) => {
//                     tracing::warn!("ZK headstash deploy failed (non-fatal): {}", e);
//                     None
//                 }
//             }
//         } else {
//             None
//         };

//         Ok(Self {
//             chain,
//             infuser,
//             billboards,
//         })
//     }

//     /// Collect deployed contract addresses as a map (config key -> address).
//     pub fn collect_addresses(&self) -> HashMap<String, String> {
//         let mut addrs = HashMap::new();

//         if let Some(ref suite) = self.dao {
//             if let Ok(addr) = suite.dao_core.addr_str() {
//                 addrs.insert("daoCore".into(), addr);
//             }
//             if let Ok(addr) = suite.proposal.calendar.addr_str() {
//                 addrs.insert("daoCalendar".into(), addr);
//             }
//         }

//         // Future: ZK, infuser, billboard addresses
//         // if let Some(ref suite) = self.zk { ... }

//         addrs
//     }

//     /// Print deployed contract addresses in `CONTRACT_ADDR:name=addr` format.
//     pub fn print_addresses(&self) {
//         for (key, addr) in self.collect_addresses() {
//             println!("CONTRACT_ADDR:{}={}", key, addr);
//         }
//     }
// }
