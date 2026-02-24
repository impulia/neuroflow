use crate::models::Database;
use anyhow::Result;
use std::fs;
use std::path::PathBuf;

pub struct Storage {
    path: PathBuf,
}

impl Storage {
    pub fn get_base_dir() -> Result<PathBuf> {
        let mut path =
            dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
        path.push(".neflo");
        if !path.exists() {
            fs::create_dir_all(&path)?;
        }
        Ok(path)
    }

    pub fn new() -> Result<Self> {
        let path = Self::get_base_dir()?;
        Ok(Self::from_path(path.join("db.json")))
    }

    pub fn from_path(path: PathBuf) -> Self {
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                let _ = fs::create_dir_all(parent);
            }
        }
        Self { path }
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
        let tmp_path = self.path.with_extension("tmp");
        fs::write(&tmp_path, &data)?;
        fs::rename(&tmp_path, &self.path)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Interval, IntervalType};
    use chrono::Utc;
    use tempfile::tempdir;

    #[test]
    fn test_storage_save_load() -> Result<()> {
        let dir = tempdir()?;
        let db_path = dir.path().join("db.json");
        let storage = Storage::from_path(db_path);

        let mut db = Database::default();
        db.intervals
            .push(Interval::new_at(IntervalType::Focus, Utc::now()));

        storage.save(&db)?;

        let loaded_db = storage.load()?;
        assert_eq!(loaded_db.intervals.len(), 1);
        assert_eq!(loaded_db.intervals[0].kind, IntervalType::Focus);

        Ok(())
    }

    #[test]
    fn test_storage_load_nonexistent() -> Result<()> {
        let dir = tempdir()?;
        let db_path = dir.path().join("nonexistent.json");
        let storage = Storage::from_path(db_path);

        let db = storage.load()?;
        assert!(db.intervals.is_empty());

        Ok(())
    }
}
