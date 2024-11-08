use std::{
    fs::{self, File},
    path::{Path, PathBuf},
    time::SystemTime,
};

use database::{Database, Insertable};
use rebrikable::{table, Client, Table};

mod database;
mod rebrikable;
mod types;

#[derive(Debug, clap::Parser)]
#[clap(about)]
struct Args {
    /// Set the verbosity level for log messages.
    #[clap(global = true, long, default_value = "info", env = "RBK_DB_LOG_LEVEL")]
    log_level: tracing::level_filters::LevelFilter,
    /// Generate the completion script for the specified shell.
    #[clap(long, exclusive = true, name = "SHELL")]
    completion: Option<clap_complete::Shell>,
    /// Overwrite the output file if it already exists.
    #[clap(short, long)]
    force: bool,
    /// The output file.
    #[clap(required = true)]
    database: Option<PathBuf>,
}

fn generate_completions(shell: clap_complete::Shell) -> ! {
    clap_complete::generate(
        shell,
        &mut <Args as clap::CommandFactory>::command(),
        clap::crate_name!(),
        &mut std::io::stdout(),
    );
    std::process::exit(0);
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
    if let Some(shell) = args.completion {
        generate_completions(shell);
    }
    setup_logging(args.log_level)?;
    run(&args).await?;
    Ok(())
}

async fn run(args: &Args) -> anyhow::Result<()> {
    let db_path = args.database.as_deref().unwrap();
    if db_path.exists() {
        if args.force {
            tracing::warn!("overwriting existing database at {}", db_path.display());
            fs::remove_file(db_path)?;
        } else {
            anyhow::bail!("database already exists at {}", db_path.display());
        }
    }

    let temp_dir = tempfile::tempdir()?;
    let temp_dir = temp_dir.path();

    let client = Client::new();
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs();
    client
        .download_tables(temp_dir, timestamp)
        .execute()
        .await?;

    let mut db = Database::open(db_path)?;
    copy_table::<table::Colors, _>(temp_dir, &mut db)?;
    copy_table::<table::PartCategories, _>(temp_dir, &mut db)?;
    copy_table::<table::Parts, _>(temp_dir, &mut db)?;
    copy_table::<table::PartRelationships, _>(temp_dir, &mut db)?;
    copy_table::<table::Elements, _>(temp_dir, &mut db)?;
    copy_table::<table::Minifigs, _>(temp_dir, &mut db)?;
    copy_table::<table::Themes, _>(temp_dir, &mut db)?;
    copy_table::<table::Sets, _>(temp_dir, &mut db)?;
    copy_table::<table::Inventories, _>(temp_dir, &mut db)?;
    copy_table::<table::InventoryParts, _>(temp_dir, &mut db)?;
    copy_table::<table::InventoryMinifigs, _>(temp_dir, &mut db)?;
    copy_table::<table::InventorySets, _>(temp_dir, &mut db)?;
    Ok(())
}

fn copy_table<T, P>(dump_dir: P, db: &mut Database) -> anyhow::Result<()>
where
    P: AsRef<Path>,
    T: Table,
    <T as Table>::Record: Insertable,
{
    let table_path = dump_dir.as_ref().join(T::FILENAME);
    let table_file = File::open(table_path)?;
    let records = T::read_records_gz(table_file);
    let records = records.collect::<Result<Vec<_>, _>>()?;
    tracing::info!("copying {} records to table {}", records.len(), T::NAME);
    db.insert_many(&records)?;
    Ok(())
}
