use clap::Parser;

#[derive(Clone, Parser)]
pub struct Cli {
    #[command(flatten)]
    pub fuel_core_config: fuel_core_bin::cli::run::Command,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    Ok(())
}
