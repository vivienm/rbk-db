use std::path::Path;

use rusqlite::{params, CachedStatement, Connection, Transaction};
use url::Url;

use crate::{
    rebrikable::record,
    types::{PartMaterial, PartRelationType},
};

mod embedded {
    refinery::embed_migrations!("./src/database/migrations");
}

#[derive(Debug)]
pub struct Database {
    conn: Connection,
}

impl Database {
    fn new(conn: Connection) -> Self {
        Self { conn }
    }

    pub fn open<P>(path: P) -> anyhow::Result<Self>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        let mut conn = Connection::open(path)?;
        embedded::migrations::runner().run(&mut conn)?;
        Ok(Self::new(conn))
    }

    pub fn insert_many<'a, R, I>(&mut self, rows: I) -> anyhow::Result<()>
    where
        R: Insertable + 'a + ?Sized,
        I: IntoIterator<Item = &'a R>,
    {
        R::insert_many(&mut self.conn, rows)
    }
}

#[allow(private_bounds)]
pub trait Insertable: InsertableSealed {}

trait InsertableSealed {
    const INSERT_STMT: &str;

    fn insert_row(stmt: &mut CachedStatement, row: &Self) -> anyhow::Result<()>;

    fn pre_hook(_tx: &Transaction) -> anyhow::Result<()> {
        Ok(())
    }

    fn post_hook(_tx: &Transaction) -> anyhow::Result<()> {
        Ok(())
    }

    fn insert_many<'a, I>(conn: &mut Connection, rows: I) -> anyhow::Result<()>
    where
        Self: 'a,
        I: IntoIterator<Item = &'a Self>,
    {
        let tx = conn.transaction()?;
        Self::pre_hook(&tx)?;
        let mut stmt = tx.prepare_cached(Self::INSERT_STMT)?;
        for row in rows {
            Self::insert_row(&mut stmt, row)?;
        }
        drop(stmt);
        Self::post_hook(&tx)?;
        tx.commit()?;
        Ok(())
    }
}

impl Insertable for record::Inventory {}

impl InsertableSealed for record::Inventory {
    const INSERT_STMT: &str = r#"
        INSERT INTO inventories (
            id,
            version,
            set_num
        )
        VALUES (?, ?, ?)
    "#;

    fn insert_row(stmt: &mut CachedStatement, row: &record::Inventory) -> anyhow::Result<()> {
        stmt.execute(params![&row.id, &row.version, &row.set_num])?;
        Ok(())
    }
}

impl Insertable for record::InventoryPart {}

impl InsertableSealed for record::InventoryPart {
    const INSERT_STMT: &str = r#"
        INSERT INTO inventory_parts (
            inventory_id,
            part_num,
            color_id,
            quantity,
            is_spare,
            img_url
        )
        VALUES (?, ?, ?, ?, ?, ?)
    "#;

    fn insert_row(stmt: &mut CachedStatement, row: &record::InventoryPart) -> anyhow::Result<()> {
        stmt.execute(params![
            &row.inventory_id,
            &row.part_num,
            &row.color_id,
            &row.quantity,
            &row.is_spare,
            &row.img_url.as_ref().map(Url::as_str)
        ])?;
        Ok(())
    }
}

impl Insertable for record::InventoryMinifig {}

impl InsertableSealed for record::InventoryMinifig {
    const INSERT_STMT: &str = r#"
        INSERT INTO inventory_minifigs (
            inventory_id,
            fig_num,
            quantity
        )
        VALUES (?, ?, ?)
    "#;

    fn insert_row(
        stmt: &mut CachedStatement,
        row: &record::InventoryMinifig,
    ) -> anyhow::Result<()> {
        stmt.execute(params![&row.inventory_id, &row.fig_num, &row.quantity])?;
        Ok(())
    }
}

impl Insertable for record::InventorySet {}

impl InsertableSealed for record::InventorySet {
    const INSERT_STMT: &str = r#"
        INSERT INTO inventory_sets (
            inventory_id,
            set_num,
            quantity
        )
        VALUES (?, ?, ?)
    "#;

    fn insert_row(stmt: &mut CachedStatement, row: &record::InventorySet) -> anyhow::Result<()> {
        stmt.execute(params![&row.inventory_id, &row.set_num, &row.quantity])?;
        Ok(())
    }
}

impl Insertable for record::Part {}

impl InsertableSealed for record::Part {
    const INSERT_STMT: &str = r#"
        INSERT INTO parts (
            part_num,
            name,
            part_cat_id,
            part_material
        )
        VALUES (?, ?, ?, ?)
    "#;

    fn insert_row(stmt: &mut CachedStatement, row: &record::Part) -> anyhow::Result<()> {
        stmt.execute(params![
            &row.part_num,
            &row.name,
            &row.part_cat_id,
            encode_part_material(row.part_material)
        ])?;
        Ok(())
    }
}

impl Insertable for record::PartCategory {}

impl InsertableSealed for record::PartCategory {
    const INSERT_STMT: &str = r#"
        INSERT INTO part_categories (
            id,
            name
        )
        VALUES (?, ?)
    "#;

    fn insert_row(stmt: &mut CachedStatement, row: &record::PartCategory) -> anyhow::Result<()> {
        stmt.execute(params![&row.id, &row.name])?;
        Ok(())
    }
}

impl Insertable for record::PartRelationship {}

impl InsertableSealed for record::PartRelationship {
    const INSERT_STMT: &str = r#"
        INSERT INTO part_relationships (
            rel_type,
            child_part_num,
            parent_part_num
        )
        VALUES (?, ?, ?)
    "#;

    fn insert_row(
        stmt: &mut CachedStatement,
        row: &record::PartRelationship,
    ) -> anyhow::Result<()> {
        stmt.execute(params![
            encode_relation_type(row.rel_type),
            &row.child_part_num,
            &row.parent_part_num
        ])?;
        Ok(())
    }
}

impl Insertable for record::Element {}

impl InsertableSealed for record::Element {
    const INSERT_STMT: &str = r#"
        INSERT INTO elements (
            element_id,
            part_num,
            color_id
        )
        VALUES (?, ?, ?)
    "#;

    fn insert_row(stmt: &mut CachedStatement, row: &record::Element) -> anyhow::Result<()> {
        stmt.execute(params![&row.element_id, &row.part_num, &row.color_id])?;
        Ok(())
    }
}

impl Insertable for record::Color {}

impl InsertableSealed for record::Color {
    const INSERT_STMT: &str = r#"
        INSERT INTO colors (
            id,
            name,
            rgb,
            is_trans
        )
        VALUES (?, ?, ?, ?)
    "#;

    fn insert_row(stmt: &mut CachedStatement, row: &record::Color) -> anyhow::Result<()> {
        stmt.execute(params![
            &row.id,
            &row.name,
            &row.rgb.to_string(),
            row.is_trans
        ])?;
        Ok(())
    }
}

impl Insertable for record::Minifig {}

impl InsertableSealed for record::Minifig {
    const INSERT_STMT: &str = r#"
        INSERT INTO minifigs (
            fig_num,
            name,
            num_parts,
            img_url
        )
        VALUES (?, ?, ?, ?)
    "#;

    fn insert_row(stmt: &mut CachedStatement, row: &record::Minifig) -> anyhow::Result<()> {
        stmt.execute(params![
            &row.fig_num,
            &row.name,
            &row.num_parts,
            &row.img_url.as_ref().map(Url::as_str)
        ])?;
        Ok(())
    }
}

impl Insertable for record::Set {}

impl InsertableSealed for record::Set {
    const INSERT_STMT: &str = r#"
        INSERT INTO sets (
            set_num,
            name,
            year,
            theme_id,
            num_parts,
            img_url
        )
        VALUES (?, ?, ?, ?, ?, ?)
    "#;

    fn insert_row(stmt: &mut CachedStatement, row: &record::Set) -> anyhow::Result<()> {
        stmt.execute(params![
            &row.set_num,
            &row.name,
            &row.year,
            &row.theme_id,
            &row.num_parts,
            &row.img_url.as_ref().map(Url::as_str)
        ])?;
        Ok(())
    }
}

impl Insertable for record::Theme {}

impl InsertableSealed for record::Theme {
    const INSERT_STMT: &str = r#"
        INSERT INTO themes (
            id,
            name,
            tmp_parent_id
        )
        VALUES (?, ?, ?)
    "#;

    // Rows are not inserted in the correct (topological) order, so some records
    // have a `parent_id` that does not exist yet. We add a temporary column to
    // store the `parent_id` and update it after all rows have been inserted.
    fn pre_hook(tx: &Transaction) -> anyhow::Result<()> {
        tx.execute("ALTER TABLE themes ADD COLUMN tmp_parent_id INTEGER", [])?;
        Ok(())
    }

    fn post_hook(tx: &Transaction) -> anyhow::Result<()> {
        tx.execute("UPDATE themes SET parent_id = tmp_parent_id", [])?;
        tx.execute("ALTER TABLE themes DROP COLUMN tmp_parent_id", [])?;
        Ok(())
    }

    fn insert_row(stmt: &mut CachedStatement, row: &record::Theme) -> anyhow::Result<()> {
        stmt.execute(params![&row.id, &row.name, &row.parent_id])?;
        Ok(())
    }
}

fn encode_relation_type(rel_type: PartRelationType) -> &'static str {
    match rel_type {
        PartRelationType::Print => "print",
        PartRelationType::Pair => "pair",
        PartRelationType::SubPart => "subpart",
        PartRelationType::Mold => "mold",
        PartRelationType::Pattern => "pattern",
        PartRelationType::Alternate => "alternate",
    }
}

fn encode_part_material(part_material: PartMaterial) -> &'static str {
    match part_material {
        PartMaterial::CardboardPaper => "cardboard/paper",
        PartMaterial::Cloth => "cloth",
        PartMaterial::FlexiblePlastic => "flexible plastic",
        PartMaterial::Foam => "foam",
        PartMaterial::Metal => "metal",
        PartMaterial::Plastic => "plastic",
        PartMaterial::Rubber => "rubber",
    }
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::Database;

    #[test]
    fn init() -> anyhow::Result<()> {
        let dir = tempdir()?;
        let path = dir.path().join("test.db");
        Database::open(path)?;
        Ok(())
    }
}
