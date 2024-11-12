use std::{
    fs::{self, File},
    path::{Path, PathBuf},
    time::SystemTime,
};

use crate::{
    database::{Database, Insertable},
    rebrikable::{table, Client, Table},
};

#[derive(Debug, clap::Parser)]
pub struct Args {
    /// If the database file already exists, overwrite it.
    #[clap(short, long)]
    force: bool,
    /// The database file to create.
    #[clap(default_value = "rebrickable.db", env = "RBK_DB_DATABASE")]
    database: PathBuf,
}

pub async fn run(args: Args) -> anyhow::Result<()> {
    let db_path = args.database.as_path();
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

    client
        .download_tables(temp_dir, current_timestamp()?)
        .execute()
        .await?;

    let mut db = Database::open(db_path)?;
    copy_tables(temp_dir, &mut db)?;
    Ok(())
}

fn current_timestamp() -> anyhow::Result<u64> {
    Ok(SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs())
}

fn copy_tables<P>(dump_dir: P, db: &mut Database) -> anyhow::Result<()>
where
    P: AsRef<Path>,
{
    let dump_dir = dump_dir.as_ref();
    copy_table::<table::Colors, _>(dump_dir, db)?;
    copy_table::<table::PartCategories, _>(dump_dir, db)?;
    copy_table::<table::Parts, _>(dump_dir, db)?;
    copy_table::<table::PartRelationships, _>(dump_dir, db)?;
    copy_table::<table::Elements, _>(dump_dir, db)?;
    copy_table::<table::Minifigs, _>(dump_dir, db)?;
    copy_table::<table::Themes, _>(dump_dir, db)?;
    copy_table::<table::Sets, _>(dump_dir, db)?;
    copy_table::<table::Inventories, _>(dump_dir, db)?;
    copy_table::<table::InventoryParts, _>(dump_dir, db)?;
    copy_table::<table::InventoryMinifigs, _>(dump_dir, db)?;
    copy_table::<table::InventorySets, _>(dump_dir, db)?;

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
