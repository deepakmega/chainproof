use crate::types::*;
use crate::diff;
use std::path::Path;
use std::fs;
use anyhow::Result;

#[derive(Debug)]
pub struct VerifyResult {
    pub passed: bool,
    pub diffs: Vec<String>,
}

/// Verify a current directory against a baseline snapshot
pub fn verify_against_baseline(baseline_path: &Path, current_dir: &Path) -> Result<VerifyResult> {
    // Load baseline snapshot from baseline_path (JSON)
    let baseline_json = fs::read_to_string(baseline_path)?;
    let baseline: Snapshot = serde_json::from_str(&baseline_json)?;

    // Create fresh snapshot from current_dir using crate::snapshot::create_snapshot
    let current = crate::snapshot::create_snapshot(current_dir)?;

    // Call crate::diff::diff_snapshots to compare
    let report = diff::diff_snapshots(&baseline, &current);

    // Check if there are any differences
    let has_differences = !report.added_binaries.is_empty()
        || !report.removed_binaries.is_empty()
        || !report.changed_binaries.is_empty()
        || !report.added_lockfiles.is_empty()
        || !report.removed_lockfiles.is_empty()
        || !report.changed_lockfiles.is_empty();

    let mut diffs = Vec::new();

    // Build human-readable diff messages
    for binary in &report.added_binaries {
        diffs.push(format!("Added binary: {}", binary));
    }

    for binary in &report.removed_binaries {
        diffs.push(format!("Removed binary: {}", binary));
    }

    for (path, old_hash, new_hash) in &report.changed_binaries {
        diffs.push(format!(
            "Changed binary: {} (old: {}, new: {})",
            path, old_hash, new_hash
        ));
    }

    for lockfile in &report.added_lockfiles {
        diffs.push(format!("Added lockfile: {}", lockfile));
    }

    for lockfile in &report.removed_lockfiles {
        diffs.push(format!("Removed lockfile: {}", lockfile));
    }

    for (path, old_hash, new_hash) in &report.changed_lockfiles {
        diffs.push(format!(
            "Changed lockfile: {} (old: {}, new: {})",
            path, old_hash, new_hash
        ));
    }

    Ok(VerifyResult {
        passed: !has_differences,
        diffs,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_verify_identical_snapshots() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let baseline_file = temp_dir.path().join("baseline.json");

        // Create a baseline snapshot
        let snapshot = Snapshot {
            binaries: vec![],
            lockfiles: vec![],
            meta: SnapshotMeta {
                created_at: "2024-01-01T00:00:00Z".to_string(),
                signature: "sig".to_string(),
            },
        };

        let json = serde_json::to_string(&snapshot)?;
        let mut file = fs::File::create(&baseline_file)?;
        file.write_all(json.as_bytes())?;
        drop(file);

        // Verify against an empty directory (which will create an empty snapshot)
        let empty_dir = TempDir::new()?;
        let result = verify_against_baseline(&baseline_file, empty_dir.path())?;

        assert!(result.passed);
        assert!(result.diffs.is_empty());

        Ok(())
    }

    #[test]
    fn test_verify_with_differences() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let baseline_file = temp_dir.path().join("baseline.json");

        // Create a baseline with one lockfile
        let baseline = Snapshot {
            binaries: vec![],
            lockfiles: vec![
                LockfileEntry {
                    path: "old.lock".to_string(),
                    hash: "oldhhash".to_string(),
                },
            ],
            meta: SnapshotMeta {
                created_at: "2024-01-01T00:00:00Z".to_string(),
                signature: "sig".to_string(),
            },
        };

        let json = serde_json::to_string(&baseline)?;
        let mut file = fs::File::create(&baseline_file)?;
        file.write_all(json.as_bytes())?;
        drop(file);

        // Create a current directory without the lockfile
        let current_dir = TempDir::new()?;

        let result = verify_against_baseline(&baseline_file, current_dir.path())?;

        assert!(!result.passed);
        assert!(!result.diffs.is_empty());

        Ok(())
    }
}
