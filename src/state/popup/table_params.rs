use crate::state::popup::token::Token;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TableParams {
    pub uuid: String,
    pub headers: Vec<String>,
    pub rows: Vec<TableRow>,
}

impl Default for TableParams {
    fn default() -> Self {
        Self::new()
    }
}

impl TableParams {
    pub fn new() -> Self {
        Self {
            uuid: Uuid::new_v4().to_string(),
            headers: vec![],
            rows: vec![],
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TableRow {
    pub cells: Vec<TableCell>,
}

impl Default for TableRow {
    fn default() -> Self {
        Self::new()
    }
}

impl TableRow {
    pub fn new() -> Self {
        Self { cells: vec![] }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TableCell {
    pub tokens: Vec<Token>,
}

impl Default for TableCell {
    fn default() -> Self {
        Self::new()
    }
}

impl TableCell {
    pub fn new() -> Self {
        Self { tokens: vec![] }
    }
}
