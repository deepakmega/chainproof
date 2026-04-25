use chainproof::types::*;

#[test]
fn test_snapshot_data_structures() {
    let snapshot = Snapshot {
        binaries: vec![BinaryEntry {
            path: "/usr/bin/gcc".to_string(),
            hash: "abc123".to_string(),
        }],
        lockfiles: vec![LockfileEntry {
            path: "Cargo.lock".to_string(),
            hash: "def456".to_string(),
        }],
        meta: SnapshotMeta {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            signature: "ghi789".to_string(),
        },
    };

    assert_eq!(snapshot.binaries.len(), 1);
    assert_eq!(snapshot.lockfiles.len(), 1);
    assert_eq!(snapshot.meta.created_at, "2024-01-01T00:00:00Z");
}

#[test]
fn test_empty_snapshot_serialization() {
    let snapshot = Snapshot {
        binaries: vec![],
        lockfiles: vec![],
        meta: SnapshotMeta {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            signature: "sig".to_string(),
        },
    };

    let json = serde_json::to_string(&snapshot).expect("Should serialize");
    assert!(json.contains("binaries"));
    assert!(json.contains("lockfiles"));
    assert!(json.contains("meta"));
}

#[test]
fn test_lockfile_entry_creation() {
    let entry = LockfileEntry {
        path: "Cargo.lock".to_string(),
        hash: "abc123def456".to_string(),
    };

    assert_eq!(entry.path, "Cargo.lock");
    assert_eq!(entry.hash.len(), 12);
}

#[test]
fn test_binary_entry_creation() {
    let entry = BinaryEntry {
        path: "/usr/bin/cargo".to_string(),
        hash: "0123456789abcdef".to_string(),
    };

    assert_eq!(entry.path, "/usr/bin/cargo");
    assert!(entry.hash.starts_with("0123"));
}
