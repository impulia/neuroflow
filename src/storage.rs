use crate::models::Database;
use anyhow::Result;
use std::fs;
use std::path::PathBuf;

pub struct Storage {
    path: PathBuf,
}

impl Storage {
    pub fn new() -> Result<Self> {
        let mut path =
            dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
        path.push(".neflo");
        if !path.exists() {
            fs::create_dir_all(&path)?;
        }
        path.push("db.json");
        Ok(Self { path })
    }

    pub fn load(&self) -> Result<Database> {
        if !self.path.exists() {
            return Ok(Database::default());
        }
        let data = fs::read_to_string(&self.path)?;
        let db = serde_json::from_str(&data)?;
        Ok(db)
    }

    pub fn save(&self, db: &Database) -> Result<()> {
        let data = serde_json::to_string_pretty(db)?;
        fs::write(&self.path, data)?;
        Ok(())
    }
}
