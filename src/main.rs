use crate::args::SetOptions;

pub mod args;
pub mod brightness;
pub mod command;
pub mod config;
pub mod connection;
pub mod error;
pub mod suninfo;

fn main() -> Result<(), String> {
    let arg: args::Arguments = argh::from_env();

    let level = if arg.verbose { log::Level::Debug } else { log::Level::Info };
    simple_logger::init_with_level(level).map_err(|_| "Cannot init logger")?;

    log::debug!("Load config from `{}`", arg.config);
    let config = config::Config::load(&arg.config)?;
    log::debug!("{config:?}");

    match arg.cli {
        Some(args::CliCommand::Get(_)) => command::get(&config),
        Some(args::CliCommand::Set(SetOptions { smooth, value })) => command::set(&config, smooth, value),
        Some(args::CliCommand::SunInfo(_)) => command::suninfo(&config),
        None => command::default(&config),
    }
}
