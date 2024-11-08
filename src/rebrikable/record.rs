use serde::Deserialize;
use url::Url;

use crate::types::{PartMaterial, PartRelationType, Rgb};

#[derive(Clone, Eq, PartialEq, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Inventory {
    pub id: i32,
    pub version: i32,
    pub set_num: String,
}

#[derive(Clone, Eq, PartialEq, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InventoryPart {
    pub inventory_id: i32,
    pub part_num: String,
    pub color_id: i32,
    pub quantity: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub is_spare: bool,
    #[serde(deserialize_with = "deserialize_option_url")]
    pub img_url: Option<Url>,
}

#[derive(Clone, Eq, PartialEq, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InventoryMinifig {
    pub inventory_id: i32,
    pub fig_num: String,
    pub quantity: i32,
}

#[derive(Clone, Eq, PartialEq, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InventorySet {
    pub inventory_id: i32,
    pub set_num: String,
    pub quantity: i32,
}

#[derive(Clone, Eq, PartialEq, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Part {
    pub part_num: String,
    pub name: String,
    pub part_cat_id: i32,
    #[serde(deserialize_with = "deserialize_part_material")]
    pub part_material: PartMaterial,
}

#[derive(Clone, Eq, PartialEq, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PartCategory {
    pub id: i32,
    pub name: String,
}

#[derive(Clone, Eq, PartialEq, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PartRelationship {
    #[serde(deserialize_with = "deserialize_part_relation")]
    pub rel_type: PartRelationType,
    pub child_part_num: String,
    pub parent_part_num: String,
}

#[derive(Clone, Eq, PartialEq, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Element {
    pub element_id: String,
    pub part_num: String,
    pub color_id: i32,
    pub design_id: Option<i32>,
}

#[derive(Clone, Eq, PartialEq, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Color {
    pub id: i32,
    pub name: String,
    pub rgb: Rgb,
    #[serde(deserialize_with = "deserialize_bool")]
    pub is_trans: bool,
}

#[derive(Clone, Eq, PartialEq, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Minifig {
    pub fig_num: String,
    pub name: String,
    pub num_parts: i32,
    #[serde(deserialize_with = "deserialize_option_url")]
    pub img_url: Option<Url>,
}

#[derive(Clone, Eq, PartialEq, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Set {
    pub set_num: String,
    pub name: String,
    pub year: i32,
    pub theme_id: i32,
    pub num_parts: i32,
    #[serde(deserialize_with = "deserialize_option_url")]
    pub img_url: Option<Url>,
}

#[derive(Clone, Eq, PartialEq, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Theme {
    pub id: i32,
    pub name: String,
    pub parent_id: Option<i32>,
}

fn deserialize_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = serde::Deserialize::deserialize(deserializer)?;
    match s.as_str() {
        "t" => Ok(true),
        "f" => Ok(false),
        _ => Err(serde::de::Error::custom("expected 't' or 'f'")),
    }
}

fn deserialize_option_url<'de, D>(deserializer: D) -> Result<Option<Url>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = serde::Deserialize::deserialize(deserializer)?;
    match s.as_str() {
        "" => Ok(None),
        s => Url::parse(s).map_err(serde::de::Error::custom).map(Some),
    }
}

fn deserialize_part_relation<'de, D>(deserializer: D) -> Result<PartRelationType, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = serde::Deserialize::deserialize(deserializer)?;
    match s.as_str() {
        "P" => Ok(PartRelationType::Print),
        "R" => Ok(PartRelationType::Pair),
        "B" => Ok(PartRelationType::SubPart),
        "M" => Ok(PartRelationType::Mold),
        "T" => Ok(PartRelationType::Pattern),
        "A" => Ok(PartRelationType::Alternate),
        _ => Err(serde::de::Error::custom(format!(
            "unknown part relation type: {:?}",
            s
        ))),
    }
}

fn deserialize_part_material<'de, D>(deserializer: D) -> Result<PartMaterial, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = serde::Deserialize::deserialize(deserializer)?;
    match s.as_str() {
        "Cardboard/Paper" => Ok(PartMaterial::CardboardPaper),
        "Cloth" => Ok(PartMaterial::Cloth),
        "Flexible Plastic" => Ok(PartMaterial::FlexiblePlastic),
        "Foam" => Ok(PartMaterial::Foam),
        "Metal" => Ok(PartMaterial::Metal),
        "Plastic" => Ok(PartMaterial::Plastic),
        "Rubber" => Ok(PartMaterial::Rubber),
        _ => Err(serde::de::Error::custom(format!(
            "unknown part material: {:?}",
            s
        ))),
    }
}
