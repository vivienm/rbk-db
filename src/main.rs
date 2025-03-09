mod commands;
mod database;
mod rebrikable;
mod types;

use self::commands::Command;

#[derive(Debug, clap::Parser)]
#[clap(about)]
struct Args {
    /// Set the verbosity level for log messages.
    #[arg(global = true, long, default_value = "info", env = "RBK_DB_LOG_LEVEL")]
    log_level: tracing::level_filters::LevelFilter,
    /// The command to execute.
    #[command(subcommand)]
    command: Command,
}

fn setup_logging(log_level: tracing::level_filters::LevelFilter) -> anyhow::Result<()> {
    let subscriber = tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_max_level(log_level)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;
    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let args = <Args as clap::Parser>::parse();
    setup_logging(args.log_level)?;
    commands::run(args.command).await?;
    Ok(())
}
