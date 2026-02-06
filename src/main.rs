use clap::{Parser, Subcommand};
use web_blog::engine::build_site;
use anyhow::Result;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Build {
        #[arg(short, long, default_value = ".")]
        input: PathBuf,

        #[arg(short, long, default_value = "public")]
        output: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Build { input, output } => {
            build_site(input, output)?;
        }
    }

    Ok(())
}