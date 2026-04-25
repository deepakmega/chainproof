use crate::types::*;
use sha2::{Sha256, Digest};
use walkdir::WalkDir;
use hex;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

/// Create a snapshot of lockfiles and CLI binaries from the filesystem
pub fn create_snapshot(dir: &Path) -> anyhow::Result<Snapshot> {
    let mut lockfiles = Vec::new();
    let mut binaries = Vec::new();

    // Lockfile patterns to search for
    let lockfile_patterns = [
        "lock.json",
        "lock.yaml",
        "lock.toml",
        "Cargo.lock",
        "go.sum",
        "requirements.txt",
        "yarn.lock",
        "package-lock.json",
        "pnpm-lock.yaml",
    ];

    // Walk the target directory tree for lockfiles
    for entry in WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let file_name = entry.file_name().to_string_lossy();
        
        // Check if filename matches any lockfile pattern
        let is_lockfile = lockfile_patterns.iter().any(|pattern| {
            file_name.ends_with(pattern) || file_name == *pattern
        });

        if is_lockfile {
            if let Ok(hash) = compute_file_hash(entry.path()) {
                lockfiles.push(LockfileEntry {
                    path: entry.path().to_string_lossy().to_string(),
                    hash,
                });
            }
        }
    }

    // Search for CLI binaries in standard locations
    let bin_paths = [
        "/usr/bin",
        "/usr/local/bin",
        "/opt/homebrew/bin",
    ];

    for bin_path in &bin_paths {
        let path = Path::new(bin_path);
        if path.exists() && path.is_dir() {
            for entry in WalkDir::new(path)
                .max_depth(1)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().is_file())
            {
                if let Ok(hash) = compute_file_hash(entry.path()) {
                    binaries.push(BinaryEntry {
                        path: entry.path().to_string_lossy().to_string(),
                        hash,
                    });
                }
            }
        }
    }

    // Get current timestamp in RFC3339 format
    let created_at = format_rfc3339_now();

    Ok(Snapshot {
        binaries,
        lockfiles,
        meta: SnapshotMeta {
            created_at,
            signature: String::new(),
        },
    })
}

/// Compute SHA-256 hash of a file and return as hex string
fn compute_file_hash(path: &Path) -> anyhow::Result<String> {
    let file_data = fs::read(path)?;
    let mut hasher = Sha256::new();
    hasher.update(&file_data);
    let hash_bytes = hasher.finalize();
    Ok(hex::encode(hash_bytes))
}

/// Format current time as RFC3339 string
fn format_rfc3339_now() -> String {
    let now = SystemTime::now();
    let duration = now
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default();
    
    let secs = duration.as_secs();
    let nanos = duration.subsec_nanos();
    
    // Convert Unix timestamp to approximate RFC3339 format
    // For simplicity, we use a basic approach - a production system would use chrono or time crate
    let days_since_epoch = secs / 86400;
    let year = 1970 + days_since_epoch / 365;
    let day_of_year = days_since_epoch % 365;
    
    let month = (day_of_year / 30).min(11) + 1;
    let day = (day_of_year % 30) + 1;
    
    let seconds_today = secs % 86400;
    let hours = seconds_today / 3600;
    let minutes = (seconds_today % 3600) / 60;
    let seconds = seconds_today % 60;
    
    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:09}Z",
        year, month, day, hours, minutes, seconds, nanos
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_create_snapshot_with_lockfiles() -> anyhow::Result<()> {
        let temp_dir = TempDir::new()?;
        let temp_path = temp_dir.path();

        // Create a test lockfile
        let lock_file = temp_path.join("package-lock.json");
        let mut file = fs::File::create(&lock_file)?;
        file.write_all(b"test content")?;
        drop(file);

        let snapshot = create_snapshot(temp_path)?;
        
        assert_eq!(snapshot.lockfiles.len(), 1);
        assert!(snapshot.lockfiles[0].path.ends_with("package-lock.json"));
        assert!(!snapshot.lockfiles[0].hash.is_empty());

        Ok(())
    }

    #[test]
    fn test_compute_file_hash() -> anyhow::Result<()> {
        let temp_dir = TempDir::new()?;
        let temp_path = temp_dir.path();

        let test_file = temp_path.join("test.txt");
        let mut file = fs::File::create(&test_file)?;
        file.write_all(b"test")?;
        drop(file);

        let hash = compute_file_hash(&test_file)?;
        
        // Verify it's a valid hex string and has expected length (64 chars for SHA-256)
        assert_eq!(hash.len(), 64);
        assert!(hash.chars().all(|c| c.is_ascii_hexdigit()));

        Ok(())
    }

    #[test]
    fn test_format_rfc3339_now() {
        let timestamp = format_rfc3339_now();
        
        // Basic sanity checks for RFC3339 format
        assert!(timestamp.contains('T'));
        assert!(timestamp.contains('Z'));
        assert!(timestamp.len() > 20);
    }
}
