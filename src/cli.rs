use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// My Rust version of RSocrata written as a cli app
pub struct Args {
    /// URL
    #[arg(value_name = "URL")]
    pub dataset_url: String, // work on vectors later

    /// Api Key
    #[arg(short = 'k', long = "key", default_value = "-", value_name = "API_KEY")]
    pub api_key: String,

    /// Output File
    #[arg(value_name = "OUT_FILE")]
    pub out_file: Option<String>,

    /// Username
    #[arg(
        short = 'n',
        long = "username",
        default_value = "-",
        value_name = "USERNAME"
    )]
    pub username: String,

    /// Password
    #[arg(
        short = 'p',
        long = "password",
        default_value = "-",
        value_name = "PASSWORD"
    )]
    pub password: String,

    /// Offset
    #[arg(short = 'o', long = "offset", value_name = "OFFSET")]
    pub offset: Option<u32>,

    /// Limit
    #[arg(short = 'l', long = "limit", value_name = "LIMIT")]
    pub limit: Option<u32>,
}
