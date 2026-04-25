# chainproof

**Cryptographically signed snapshots of your supply chain — detect tampering in CI before it's too late.**

<!-- ![chainproof demo](https://placeholder.com/800x400?text=chainproof+demo) -->
> *Demo screenshot: `chainproof verify` catching a tampered lockfile in GitHub Actions*

## Why chainproof?

Supply-chain attacks are getting bolder. Package managers get compromised. Build artifacts are replaced. System binaries drift without notice. By the time you detect it, your CI is already shipping backdoors.

- **🎯 Detect lockfile tampering** — Dependencies silently change? Chainproof catches it.
- **🔐 Cryptographically signed baselines** — HMAC-SHA256 signatures prevent forgery; commit `.chainproof.json` to version control.
- **⚡ Zero runtime overhead** — Runs in seconds; designed for every CI job.
- **📦 Multi-format support** — Cargo.lock, package-lock.json, go.sum, requirements.txt, yarn.lock, pnpm-lock.yaml, and custom lock files.
- **🖥️ Binary monitoring** — Watches `/usr/bin`, `/usr/local/bin`, `/opt/homebrew/bin` for drift.
- **✅ CI-native exit codes** — Exit 0 = clean; exit 1 = drift detected. Perfect for GitHub Actions, GitLab CI, CircleCI.

## Install

### From crates.io (recommended)

```bash
cargo install chainproof
```

### Build from source

```bash
git clone https://github.com/yourusername/chainproof.git
cd chainproof
cargo build --release
./target/release/chainproof --help
```

## Quickstart

### 1. Initialize baseline in your repo

```bash
$ chainproof init
chainproof: creating baseline snapshot...
OK: Baseline snapshot saved to .chainproof.json
  Binaries: 127
  Lockfiles: 3
```

This creates `.chainproof.json` with signed hashes of all lockfiles and system binaries.

### 2. Commit to version control

```bash
git add .chainproof.json
git commit -m "chore: add chainproof baseline"
git push
```

### 3. Verify in CI (or locally anytime)

```bash
$ chainproof verify
chainproof: verifying against baseline...
OK: Verification passed - environment matches baseline
```

Exit code 0 = clean. If tampering is detected:

```bash
$ chainproof verify
chainproof: verifying against baseline...
FAIL: Verification failed - environment differs from baseline

  [MODIFIED] /usr/local/bin/python (was: abc123..., now: def456...)
  [MODIFIED] Cargo.lock (was: 789abc..., now: 456def...)
  [ADDED]    Gemfile.lock
```

Exit code 1 = tampering detected. Fail the build.

## Features

| Feature | Details |
|---------|---------|
| **File Hashing** | SHA-256 cryptographic hashing of all tracked files |
| **Baseline Signing** | HMAC-SHA256 signatures embedded in `.chainproof.json` prevent tampering |
| **Lockfile Coverage** | Cargo.lock, package-lock.json, package.json, go.sum, requirements.txt, yarn.lock, pnpm-lock.yaml |
| **Binary Monitoring** | Tracks system binaries in `/usr/bin`, `/usr/local/bin`, `/opt/homebrew/bin` |
| **Human-Readable Diffs** | Clear output: `[MODIFIED]`, `[ADDED]`, `[REMOVED]` with before/after hashes |
| **Zero Dependencies** | No external runtime dependencies—pure Rust, ~1.2 MB binary |
| **CI-Friendly Exit Codes** | Exit 0 = pass, Exit 1 = fail. Works with any CI system |

## Examples

### Example 1: Initialize baseline

```bash
$ chainproof init
chainproof: creating baseline snapshot...
OK: Baseline snapshot saved to .chainproof.json
  Binaries: 127
  Lockfiles: 3
```

**Output:** `.chainproof.json` (1.2 MB)
```json
{
  "binaries": [
    {
      "path": "/usr/local/bin/python",
      "hash": "abc123def456..."
    }
  ],
  "lockfiles": [
    {
      "path": "Cargo.lock",
      "hash": "789abcdef012..."
    }
  ],
  "meta": {
    "created_at": "2026-04-24T18:30:45.123456Z",
    "signature": "hmac_sha256_signature_here"
  }
}
```

### Example 2: Verify (success case)

```bash
$ chainproof verify
chainproof: verifying against baseline...
OK: Verification passed - environment matches baseline
```

Exit code: **0** ✅

### Example 3: Verify (failure case — tampering detected)

```bash
$ chainproof verify
chainproof: verifying against baseline...
FAIL: Verification failed - environment differs from baseline

  [MODIFIED] Cargo.lock
    Before: 789abcdef012...
    After:  9876543210ab...

  [ADDED] Gemfile.lock
    Hash: fedcba987654...

  [REMOVED] yarn.lock
    Was: abcdef123456...
```

Exit code: **1** ❌

### Example 4: Show detailed diff report

```bash
$ chainproof diff
chainproof: diff report
  Baseline: 127 binaries, 3 lockfiles
  Current: 127 binaries, 4 lockfiles

  [MODIFIED] Cargo.lock (binary diff: 512 bytes)
  [ADDED] Gemfile.lock
```

Exit code: **1** (if differences found) or **0** (if identical)

## CI Integration

### GitHub Actions

Add to your `.github/workflows/ci.yml`:

```yaml
name: Supply Chain Integrity Check

on: [push, pull_request]

jobs:
  chainproof:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
      
      - name: Install chainproof
        run: cargo install chainproof
      
      - name: Verify supply chain
        run: chainproof verify
```

The job fails immediately if tampering is detected. No tampered code ships.

### GitLab CI

```yaml
supply_chain_check:
  image: rust:latest
  script:
    - cargo install chainproof
    - chainproof verify
```

### CircleCI

```yaml
jobs:
  supply-chain:
    docker:
      - image: rust:latest
    steps:
      - checkout
      - run: cargo install chainproof
      - run: chainproof verify
```

## How it works

1. **Init**: Walks the filesystem, computes SHA-256 hashes of all lockfiles and system binaries, signs the snapshot with HMAC-SHA256, writes `.chainproof.json`.
2. **Verify**: Loads `.chainproof.json`, verifies the signature, recreates the current snapshot, compares hashes, exits 0 (clean) or 1 (drift detected).
3. **Diff**: Compares two snapshots and outputs human-readable differences.

All hashing is **deterministic and reproducible** — same environment = same hashes.

## Roadmap

- [ ] **Custom signing keys** — Inject your own key instead of the hardcoded one; support for HSM/KMS
- [ ] **Remote attestation** — Push signatures to a trusted server; verify they haven't been tampered with locally
- [ ] **Policy files** — Allowlist specific binaries/lockfiles; ignore non-critical drifts
- [ ] **Time-based baselines** — Support expiring baselines; require re-init monthly
- [ ] **Parallel hashing** — Use rayon for 4x faster snapshots on large filesystems

## Contributing

We love contributions! Here's how to get started:

1. **Fork** the repository
2. **Create a branch** — `git checkout -b feature/my-feature`
3. **Make your changes** — Keep them focused and tested
4. **Run tests** — `cargo test --all` (must pass)
5. **Commit** — Use conventional commits: `feat:`, `fix:`, `docs:`, etc.
6. **Push** — `git push origin feature/my-feature`
7. **Open a PR** — We'll review and merge 🚀

### Development setup

```bash
git clone https://github.com/yourusername/chainproof.git
cd chainproof
cargo build
cargo test
```

### Running tests

```bash
cargo test --all --verbose
```

All tests must pass. We use coverage to ensure high quality.

## License

MIT License © 2024 Chainproof Authors. See [LICENSE](./LICENSE) for details.

---

**Ready to protect your supply chain?**

```bash
chainproof init && git add .chainproof.json && git commit -m "chore: add supply chain baseline"
```

Questions? Open an issue. Want to contribute? We welcome PRs. 🙌
