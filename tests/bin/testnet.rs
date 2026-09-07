use cw_orch::{
    daemon::networks::{ATOMEONE_TESTNET, TERP_TESTNET},
    prelude::*,
};
use cw_orch_interchain::prelude::*;

fn main() -> anyhow::Result<()> {
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .unwrap();
    // dotenv::dotenv().ok();
    env_logger::init();

    derive_full_ibc_state()
}

fn derive_full_ibc_state() -> anyhow::Result<()> {
    let interchain = DaemonInterchain::new(
        vec![TERP_TESTNET.clone(), ATOMEONE_TESTNET.clone()],
        &ChannelCreationValidator,
    )?;

    Ok(())
}
