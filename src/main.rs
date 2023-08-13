use crate::args::SetOptions;

pub mod args;
pub mod brightness;
pub mod command;
pub mod config;
pub mod error;
pub mod suninfo;

fn main() -> Result<(), String> {
    let arg: args::Arguments = argh::from_env();

    let level = if arg.verbose { log::Level::Debug } else { log::Level::Info };
    simple_logger::init_with_level(level).map_err(|_| "Cannot init logger")?;

    log::debug!("Load config from `{}`", arg.config);
    let config = config::Config::load(&arg.config)?;
    log::debug!("{config:?}");

    let mut connection = brightness::Connection::connect(&config)?;
    log::debug!("Establish websocket connection");

    match arg.cli {
        Some(args::CliCommand::Get(_)) => command::get(&config, &mut connection),
        Some(args::CliCommand::Set(SetOptions { value })) => command::set(&config, &mut connection, value),
        None => command::default(&config, &mut connection),
    }
}
