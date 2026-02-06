use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use web_blog::engine::build_site;
use web_blog::server::serve;

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
    Serve {
        #[arg(short, long, default_value = ".")]
        input: PathBuf,

        #[arg(short, long, default_value = "public")]
        output: PathBuf,

        #[arg(short, long, default_value_t = 3000)]
        port: u16,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Build { input, output } => {
            build_site(input, output)?;
        }
        Commands::Serve {
            input,
            output,
            port,
        } => {
            serve(input, output, *port).await?;
        }
    }

    Ok(())
}
