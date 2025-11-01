use anyhow::Result;
use log::debug;

mod utility;
mod config;
mod api;

#[tokio::main]
async fn main() -> Result<()> {
    utility::init_logging();

    let args = utility::args::parse_args()?;

    let config = config::load_config(&args.config_path)?;

    debug!("Loaded configuration: \n{:#?}", config);

    let app_state = api::create_app_state_from_config(config);


    api::start_service(app_state).await?;

   Ok(())
}