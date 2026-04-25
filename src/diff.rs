use crate::types::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct DiffReport {
    pub added_binaries: Vec<String>,
    pub removed_binaries: Vec<String>,
    pub changed_binaries: Vec<(String, String, String)>,
    pub added_lockfiles: Vec<String>,
    pub removed_lockfiles: Vec<String>,
    pub changed_lockfiles: Vec<(String, String, String)>,
}

pub fn diff_snapshots(old: &Snapshot, new: &Snapshot) -> DiffReport {
    let old_binaries: HashMap<String, String> = old
        .binaries
        .iter()
        .map(|e| (e.path.clone(), e.hash.clone()))
        .collect();
    
    let new_binaries: HashMap<String, String> = new
        .binaries
        .iter()
        .map(|e| (e.path.clone(), e.hash.clone()))
        .collect();

    let old_lockfiles: HashMap<String, String> = old
        .lockfiles
        .iter()
        .map(|e| (e.path.clone(), e.hash.clone()))
        .collect();
    
    let new_lockfiles: HashMap<String, String> = new
        .lockfiles
        .iter()
        .map(|e| (e.path.clone(), e.hash.clone()))
        .collect();

    let mut added_binaries = Vec::new();
    let mut removed_binaries = Vec::new();
    let mut changed_binaries = Vec::new();

    for (path, new_hash) in &new_binaries {
        match old_binaries.get(path) {
            None => added_binaries.push(path.clone()),
            Some(old_hash) => {
                if old_hash != new_hash {
                    changed_binaries.push((path.clone(), old_hash.clone(), new_hash.clone()));
                }
            }
        }
    }

    for path in old_binaries.keys() {
        if !new_binaries.contains_key(path) {
            removed_binaries.push(path.clone());
        }
    }

    let mut added_lockfiles = Vec::new();
    let mut removed_lockfiles = Vec::new();
    let mut changed_lockfiles = Vec::new();

    for (path, new_hash) in &new_lockfiles {
        match old_lockfiles.get(path) {
            None => added_lockfiles.push(path.clone()),
            Some(old_hash) => {
                if old_hash != new_hash {
                    changed_lockfiles.push((path.clone(), old_hash.clone(), new_hash.clone()));
                }
            }
        }
    }

    for path in old_lockfiles.keys() {
        if !new_lockfiles.contains_key(path) {
            removed_lockfiles.push(path.clone());
        }
    }

    DiffReport {
        added_binaries,
        removed_binaries,
        changed_binaries,
        added_lockfiles,
        removed_lockfiles,
        changed_lockfiles,
    }
}

impl DiffReport {
    pub fn to_strings(&self) -> Vec<String> {
        let mut lines = Vec::new();

        if !self.added_binaries.is_empty() {
            lines.push("Added binaries:".to_string());
            for path in &self.added_binaries {
                lines.push(format!("  + {}", path));
            }
        }

        if !self.removed_binaries.is_empty() {
            lines.push("Removed binaries:".to_string());
            for path in &self.removed_binaries {
                lines.push(format!("  - {}", path));
            }
        }

        if !self.changed_binaries.is_empty() {
            lines.push("Changed binaries:".to_string());
            for (path, old_hash, new_hash) in &self.changed_binaries {
                lines.push(format!("  ~ {} ({}...{} )", path, &old_hash[..8], &new_hash[..8]));
            }
        }

        if !self.added_lockfiles.is_empty() {
            lines.push("Added lockfiles:".to_string());
            for path in &self.added_lockfiles {
                lines.push(format!("  + {}", path));
            }
        }

        if !self.removed_lockfiles.is_empty() {
            lines.push("Removed lockfiles:".to_string());
            for path in &self.removed_lockfiles {
                lines.push(format!("  - {}", path));
            }
        }

        if !self.changed_lockfiles.is_empty() {
            lines.push("Changed lockfiles:".to_string());
            for (path, old_hash, new_hash) in &self.changed_lockfiles {
                lines.push(format!("  ~ {} ({}...{} )", path, &old_hash[..8], &new_hash[..8]));
            }
        }

        if lines.is_empty() {
            lines.push("No differences detected".to_string());
        }

        lines
    }

    pub fn has_diffs(&self) -> bool {
        !self.added_binaries.is_empty()
            || !self.removed_binaries.is_empty()
            || !self.changed_binaries.is_empty()
            || !self.added_lockfiles.is_empty()
            || !self.removed_lockfiles.is_empty()
            || !self.changed_lockfiles.is_empty()
    }
}
