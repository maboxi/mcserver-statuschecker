use anyhow::Result;
use log::debug;

mod utility;
mod config;
mod api;
mod updater;

fn main() -> Result<()> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;

    runtime.block_on(tokio_main())
}

async fn tokio_main() -> Result<()> {
    utility::init_logging();

    let args = utility::args::parse_args()?;
    let config = config::load_config(&args.config_path)?;

    debug!("Loaded configuration: \n{:#?}", config);

    let app_state = api::create_app_state_from_config(config);

    debug!("Starting updater task...");
    updater::run_as_task(&app_state);

    debug!("Starting API service...");
    api::start_service(app_state).await?;

    Ok(())
}