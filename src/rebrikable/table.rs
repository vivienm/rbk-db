use std::io::Read;

use serde::de::DeserializeOwned;

use super::record;

pub trait Table {
    const NAME: &str;
    const FILENAME: &str;

    type Record: DeserializeOwned;

    fn read_records_gz<R>(rdr: R) -> impl Iterator<Item = csv::Result<Self::Record>>
    where
        R: Read,
    {
        let rdr = flate2::read::GzDecoder::new(rdr);
        Self::read_records(rdr)
    }

    fn read_records<R>(rdr: R) -> impl Iterator<Item = csv::Result<Self::Record>>
    where
        R: Read,
    {
        let rdr = csv::Reader::from_reader(rdr);
        rdr.into_deserialize()
    }
}

macro_rules! decl_table {
    ($ident:ident, $name:literal, $record:ty) => {
        pub enum $ident {}

        impl Table for $ident {
            const NAME: &str = $name;
            const FILENAME: &str = concat!($name, ".csv.gz");

            type Record = $record;
        }
    };
}

decl_table!(Inventories, "inventories", record::Inventory);
decl_table!(InventoryParts, "inventory_parts", record::InventoryPart);
decl_table!(
    InventoryMinifigs,
    "inventory_minifigs",
    record::InventoryMinifig
);
decl_table!(InventorySets, "inventory_sets", record::InventorySet);
decl_table!(Parts, "parts", record::Part);
decl_table!(PartCategories, "part_categories", record::PartCategory);
decl_table!(
    PartRelationships,
    "part_relationships",
    record::PartRelationship
);
decl_table!(Elements, "elements", record::Element);
decl_table!(Colors, "colors", record::Color);
decl_table!(Minifigs, "minifigs", record::Minifig);
decl_table!(Sets, "sets", record::Set);
decl_table!(Themes, "themes", record::Theme);
