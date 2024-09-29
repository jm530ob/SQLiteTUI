use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    /// Use this argument to add database files to your buffer
    #[arg(short, long)]
    pub paths: Vec<String>,
}
