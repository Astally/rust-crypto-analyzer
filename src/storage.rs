use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;

#[derive(Serialize, Deserialize)]
pub struct FavoritesData {
    pub favorites: Vec<String>,
}

const FAVORITES_FILE: &str = "favorites.json";

pub fn save_favorites(favorites: &HashSet<String>) -> Result<()> {
    let data = FavoritesData {
        favorites: favorites.iter().cloned().collect(),
    };

    let json_string = serde_json::to_string_pretty(&data)?;

    let mut file = File::create(FAVORITES_FILE)?;

    file.write_all(json_string.as_bytes())?;

    Ok(())
}

pub fn load_favorites() {}
