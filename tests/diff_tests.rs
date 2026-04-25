use chainproof::types::*;
use chainproof::diff;

#[test]
fn test_identical_snapshots_no_diff() {
    let snap1 = Snapshot {
        binaries: vec![BinaryEntry {
            path: "/usr/bin/git".to_string(),
            hash: "hash1".to_string(),
        }],
        lockfiles: vec![],
        meta: SnapshotMeta {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            signature: "sig1".to_string(),
        },
    };

    let snap2 = Snapshot {
        binaries: vec![BinaryEntry {
            path: "/usr/bin/git".to_string(),
            hash: "hash1".to_string(),
        }],
        lockfiles: vec![],
        meta: SnapshotMeta {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            signature: "sig1".to_string(),
        },
    };

    let diff = diff::diff_snapshots(&snap1, &snap2);
    assert!(diff.added_binaries.is_empty());
    assert!(diff.removed_binaries.is_empty());
    assert!(diff.changed_binaries.is_empty());
}

#[test]
fn test_added_lockfile_detected() {
    let snap1 = Snapshot {
        binaries: vec![],
        lockfiles: vec![],
        meta: SnapshotMeta {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            signature: "sig1".to_string(),
        },
    };

    let snap2 = Snapshot {
        binaries: vec![],
        lockfiles: vec![LockfileEntry {
            path: "Cargo.lock".to_string(),
            hash: "hash1".to_string(),
        }],
        meta: SnapshotMeta {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            signature: "sig2".to_string(),
        },
    };

    let diff = diff::diff_snapshots(&snap1, &snap2);
    assert!(diff.added_lockfiles.contains(&"Cargo.lock".to_string()));
}

#[test]
fn test_removed_lockfile_detected() {
    let snap1 = Snapshot {
        binaries: vec![],
        lockfiles: vec![LockfileEntry {
            path: "Cargo.lock".to_string(),
            hash: "hash1".to_string(),
        }],
        meta: SnapshotMeta {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            signature: "sig1".to_string(),
        },
    };

    let snap2 = Snapshot {
        binaries: vec![],
        lockfiles: vec![],
        meta: SnapshotMeta {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            signature: "sig2".to_string(),
        },
    };

    let diff = diff::diff_snapshots(&snap1, &snap2);
    assert!(diff.removed_lockfiles.contains(&"Cargo.lock".to_string()));
}

#[test]
fn test_changed_lockfile_detected() {
    let snap1 = Snapshot {
        binaries: vec![],
        lockfiles: vec![LockfileEntry {
            path: "Cargo.lock".to_string(),
            hash: "hash1".to_string(),
        }],
        meta: SnapshotMeta {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            signature: "sig1".to_string(),
        },
    };

    let snap2 = Snapshot {
        binaries: vec![],
        lockfiles: vec![LockfileEntry {
            path: "Cargo.lock".to_string(),
            hash: "hash2".to_string(),
        }],
        meta: SnapshotMeta {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            signature: "sig2".to_string(),
        },
    };

    let diff = diff::diff_snapshots(&snap1, &snap2);
    assert!(diff.changed_lockfiles.contains(&("Cargo.lock".to_string(), "hash1".to_string(), "hash2".to_string())));
}
