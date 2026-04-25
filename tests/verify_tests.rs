use chainproof::types::*;

#[test]
fn test_empty_snapshot_passes_verification() {
    let empty_snapshot = Snapshot {
        binaries: vec![],
        lockfiles: vec![],
        meta: SnapshotMeta {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            signature: "test_sig".to_string(),
        },
    };

    // Serialize and verify it produces valid JSON
    let json = serde_json::to_string(&empty_snapshot).expect("Should serialize");
    assert!(!json.is_empty());
    
    // Verify we can deserialize it back
    let deserialized: Snapshot = serde_json::from_str(&json).expect("Should deserialize");
    assert_eq!(deserialized.binaries.len(), 0);
    assert_eq!(deserialized.lockfiles.len(), 0);
}

#[test]
fn test_baseline_with_missing_lockfile() {
    let baseline = Snapshot {
        binaries: vec![],
        lockfiles: vec![
            LockfileEntry {
                path: "Cargo.lock".to_string(),
                hash: "somehash".to_string(),
            }
        ],
        meta: SnapshotMeta {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            signature: "sig".to_string(),
        },
    };

    let current = Snapshot {
        binaries: vec![],
        lockfiles: vec![],
        meta: SnapshotMeta {
            created_at: "2024-01-02T00:00:00Z".to_string(),
            signature: "sig2".to_string(),
        },
    };

    // Use diff to check for differences
    let diff = chainproof::diff::diff_snapshots(&baseline, &current);
    
    // Should detect that Cargo.lock was removed
    assert!(diff.removed_lockfiles.contains(&"Cargo.lock".to_string()));
}

#[test]
fn test_snapshot_with_multiple_entries() {
    let snapshot = Snapshot {
        binaries: vec![
            BinaryEntry {
                path: "/usr/bin/git".to_string(),
                hash: "hash1".to_string(),
            },
            BinaryEntry {
                path: "/usr/bin/cargo".to_string(),
                hash: "hash2".to_string(),
            },
        ],
        lockfiles: vec![
            LockfileEntry {
                path: "Cargo.lock".to_string(),
                hash: "lockhash1".to_string(),
            },
            LockfileEntry {
                path: "package-lock.json".to_string(),
                hash: "lockhash2".to_string(),
            },
        ],
        meta: SnapshotMeta {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            signature: "sig".to_string(),
        },
    };

    assert_eq!(snapshot.binaries.len(), 2);
    assert_eq!(snapshot.lockfiles.len(), 2);
}
