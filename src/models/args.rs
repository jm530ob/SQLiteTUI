use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    /// Use this argument to add database files
    #[arg(short, long, num_args = 0.. )]
    pub paths: Vec<String>,
}
