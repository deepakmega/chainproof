use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BinaryEntry {
    pub path: String,
    pub hash: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LockfileEntry {
    pub path: String,
    pub hash: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SnapshotMeta {
    pub created_at: String,
    pub signature: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Snapshot {
    pub binaries: Vec<BinaryEntry>,
    pub lockfiles: Vec<LockfileEntry>,
    pub meta: SnapshotMeta,
}
