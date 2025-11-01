use anyhow::Result;
use log::debug;

mod utility;
use crate::utility::args::parse_args;
use crate::utility::init_logging;

mod config;
use crate::config::load_config;

mod api;

#[tokio::main]
async fn main() -> Result<()> {
    init_logging();

    let args = parse_args()?;

    let config = load_config(&args.config_path)?;

    debug!("Loaded configuration: \n{:#?}", config);

    api::start_service(config).await?;

   Ok(())
}