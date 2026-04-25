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
    
    // Extract signature from JSON without re-parsing to preserve field order
    let baseline_obj: serde_json::Value = serde_json::from_str(&baseline_json)?;
    let signature = baseline_obj["meta"]["signature"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("Missing signature in baseline"))?
        .to_string();
    
    // Remove signature by string replacement to preserve JSON structure and field order
    // Replace "signature": "XXX" with "signature": ""
    let unsigned_json = baseline_json.replace(
        &format!("\"signature\": \"{}\"", signature),
        "\"signature\": \"\""
    );
    
    // Verify the baseline signature
    if !crate::signing::verify(&unsigned_json, &signature, SIGNING_KEY) {
        return Ok(VerifyResult {
            passed: false,
            diffs: vec!["ERROR: Baseline signature verification failed - baseline may be tampered".to_string()],
        });
    }
    
    // Parse baseline snapshot
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

const SIGNING_KEY: &[u8] = b"chainproof-secret";

