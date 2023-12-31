use argh::FromArgs;

#[derive(FromArgs)]
#[argh(subcommand)]
/// command interface
pub enum CliCommand {
    Get(GetOptions),
    Set(SetOptions),
    SunInfo(SunInfoOptions),
}

#[derive(FromArgs)]
#[argh(subcommand, name = "get")]
/// Get current brightness
pub struct GetOptions {}

#[derive(FromArgs)]
#[argh(subcommand, name = "set")]
/// Set new brightness
pub struct SetOptions {
    /// enable smooth transition
    #[argh(switch, short = 's')]
    pub smooth: bool,

    #[argh(positional)]
    /// new value
    pub value: u8,
}

#[derive(FromArgs)]
#[argh(subcommand, name = "suninfo")]
/// Get todays sunrise and sunset info
pub struct SunInfoOptions {}

/// Gixie Clock Brightness Control
#[derive(FromArgs)]
pub struct Arguments {
    /// configuration file
    #[argh(option, short = 'c', default = r#"String::from("config.yaml")"#)]
    pub config: String,

    /// cli
    #[argh(subcommand)]
    pub cli: Option<CliCommand>,

    /// enable full log
    #[argh(switch, short = 'v')]
    pub verbose: bool,
}
