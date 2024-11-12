mod completion;
mod dump;

#[derive(Debug, clap::Parser)]
pub enum Command {
    /// Generate completion scripts.
    Completion(completion::Args),
    /// Dump the Rebrickable API tables to an SQLite database.
    Dump(dump::Args),
}

pub async fn run(command: Command) -> anyhow::Result<()> {
    match command {
        Command::Completion(args) => completion::run(args).await,
        Command::Dump(args) => dump::run(args).await,
    }
}
