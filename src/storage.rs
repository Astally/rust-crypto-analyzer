use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;
use std::path::Path;

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

pub fn load_favorites() -> Result<HashSet<String>> {
    if !Path::new(FAVORITES_FILE).exists() {
        return Ok(HashSet::new());
    }

    let json = std::fs::read_to_string(FAVORITES_FILE)?;

    let data: FavoritesData = serde_json::from_str(&json)?;

    Ok(data.favorites.into_iter().collect())
}
