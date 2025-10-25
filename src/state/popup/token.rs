use super::dimensions::Dimensions;
use super::{style::Style, table_params::TableParams, tag_params::TagParams};
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Token {
    Text(String, Style),
    Tag(TagParams),
    Spacing,
    Table(TableParams),
    ListElement,
    Indent(i32),
    Image(String, Option<Dimensions>),
}
