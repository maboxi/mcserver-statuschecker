use std::path::PathBuf;
use anyhow::Result;
use log::{trace, warn};

pub struct Args {
    pub config_path: PathBuf,
}

pub fn parse_args() -> Result<Args> {
    const USAGE: &str = "Usage: my_app <config_path>";
    let mut args = std::env::args().collect::<Vec<String>>();

    trace!("Program arguments: \n{:#?}", args);

    if args.len() < 2 {
        warn!("{}", USAGE);
        anyhow::bail!("Not enough arguments provided");
    } else if args.len() > 2 {
        warn!("{}", USAGE);
        anyhow::bail!("Too many arguments provided");
    }

    Ok(Args {
        config_path: args.pop().unwrap().into(),
    })
}