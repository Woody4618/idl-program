use anyhow::{Result, anyhow};
use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::Keypair;
use solana_cli_config::Config;

pub fn get_user_config(rpc_url: Option<&str>) -> Result<(Keypair, RpcClient)> {
    let config_file = solana_cli_config::CONFIG_FILE
        .as_ref()
        .ok_or_else(|| anyhow!("Unable to get config file path"))?;
    let cli_config: Config = Config::load(config_file)?;

    let signer = get_keypair_from_path(&cli_config.keypair_path)?;

    let rpc_client = if let Some(rpc_url) = rpc_url {
        RpcClient::new(rpc_url.to_string())
    } else {
        RpcClient::new(cli_config.json_rpc_url.clone())
    };
    Ok((signer, rpc_client))
}

pub fn get_keypair_from_path(path: &str) -> Result<Keypair> {
    solana_clap_utils::keypair::keypair_from_path(&Default::default(), &path, "keypair", false)
        .map_err(|err| anyhow!("Unable to get signer from path: {}", err))
} 