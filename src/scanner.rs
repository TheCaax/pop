use crate::db::{Database, FileRecord};
use std::path::Path;
use std::time::UNIX_EPOCH;
use walkdir::WalkDir;

use std::sync::mpsc;

pub fn scan_directory(root: &Path, db: &mut Database) -> anyhow::Result<()> {
    let (tx, rx) = mpsc::channel();
    let root_path = root.to_path_buf();

    // Use a thread to walk the directory
    std::thread::spawn(move || {
        for entry in WalkDir::new(root_path)
            .into_iter()
            .filter_map(|e| e.ok()) {
                let path = entry.path().to_path_buf();
                let metadata = match entry.metadata() {
                    Ok(m) => m,
                    Err(_) => continue,
                };

                let name = path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("")
                    .to_string();
                
                let extension = path.extension()
                    .and_then(|e| e.to_str())
                    .map(|s| s.to_lowercase());

                let size = metadata.len();
                let last_modified = metadata.modified()
                    .unwrap_or(UNIX_EPOCH)
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs() as i64;

                let is_dir = metadata.is_dir();

                let record = FileRecord {
                    path: path.to_string_lossy().to_string(),
                    name,
                    extension,
                    size,
                    last_modified,
                    is_dir,
                };

                if tx.send(record).is_err() {
                    break;
                }
            }
    });

    let mut batch = Vec::with_capacity(10000);
    for record in rx {
        batch.push(record);
        if batch.len() >= 10000 {
            db.insert_batch(&batch)?;
            batch.clear();
        }
    }
    
    if !batch.is_empty() {
        db.insert_batch(&batch)?;
    }

    Ok(())
}
