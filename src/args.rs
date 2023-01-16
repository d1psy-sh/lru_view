// use clap to parse some args if i need them
use clap::Parser;

#[derive(Debug, Parser)]
#[clap(
    author = "nkirschall",
    version,
    about = "A App to blasingly fast open files"
)]
pub struct Args {
    /// the file you want to open
    pub file: Option<String>,
    #[arg(
        short,
        long,
        default_value_t = false,
        help = "Whether to delet the hole list"
    )]
    pub clear: bool,
    #[arg(
        short,
        long,
        default_value_t = false,
        help = "Whether the app runs in test mode"
    )]
    pub test: bool,
}

impl Default for Args {
    fn default() -> Self {
        Self::new()
    }
}

impl Args {
   pub fn new() -> Self {
        Args::parse()
    }
}
