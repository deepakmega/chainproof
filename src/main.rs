mod types;
mod signing;
mod snapshot;
mod diff;
mod verify;

use std::env;
use std::fs;
use std::path::Path;
use anyhow::Result;

const BASELINE_PATH: &str = ".chainproof.json";
const SIGNING_KEY: &[u8] = b"chainproof-secret";

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        return Ok(());
    }

    match args[1].as_str() {
        "init" => cmd_init()?,
        "verify" => cmd_verify()?,
        "diff" => cmd_diff(&args[2..])?,
        "--help" | "-h" | "help" => print_usage(),
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            print_usage();
        }
    }

    Ok(())
}

fn cmd_init() -> Result<()> {
    println!("chainproof: creating baseline snapshot...");
    
    let snapshot = snapshot::create_snapshot(Path::new("."))?;
    
    let json_str = serde_json::to_string_pretty(&snapshot)?;
    let signature = signing::sign(&json_str, SIGNING_KEY);
    
    let mut signed_snapshot = snapshot;
    signed_snapshot.meta.signature = signature;
    
    let json = serde_json::to_string_pretty(&signed_snapshot)?;
    fs::write(BASELINE_PATH, &json)?;
    
    println!("OK: Baseline snapshot saved to {}", BASELINE_PATH);
    println!("  Binaries: {}", signed_snapshot.binaries.len());
    println!("  Lockfiles: {}", signed_snapshot.lockfiles.len());
    
    Ok(())
}

fn cmd_verify() -> Result<()> {
    if !Path::new(BASELINE_PATH).exists() {
        eprintln!("Error: {} not found. Run 'chainproof init' first.", BASELINE_PATH);
        std::process::exit(1);
    }

    println!("chainproof: verifying against baseline...");
    
    let result = verify::verify_against_baseline(Path::new(BASELINE_PATH), Path::new("."))?;
    
    if result.passed {
        println!("OK: Verification passed - environment matches baseline");
        std::process::exit(0);
    } else {
        println!("FAIL: Verification failed - environment differs from baseline");
        println!();
        for line in &result.diffs {
            println!("{}", line);
        }
        std::process::exit(1);
    }
}

fn cmd_diff(args: &[String]) -> Result<()> {
    let (baseline_path, current_path) = if args.len() >= 2 && args[0] == "--baseline" {
        (args[1].clone(), BASELINE_PATH.to_string())
    } else if args.is_empty() {
        // When no args, compare baseline to a fresh live snapshot
        (BASELINE_PATH.to_string(), "".to_string())
    } else {
        eprintln!("Usage: chainproof diff [--baseline PATH]");
        std::process::exit(1);
    };

    let baseline_json = fs::read_to_string(&baseline_path)?;
    let baseline: types::Snapshot = serde_json::from_str(&baseline_json)?;

    let current = if current_path.is_empty() {
        // Create fresh snapshot from current directory
        snapshot::create_snapshot(Path::new("."))?
    } else if current_path == BASELINE_PATH && Path::new(BASELINE_PATH).exists() {
        let current_json = fs::read_to_string(BASELINE_PATH)?;
        serde_json::from_str::<types::Snapshot>(&current_json)?
    } else {
        let current_json = fs::read_to_string(&current_path)?;
        serde_json::from_str::<types::Snapshot>(&current_json)?
    };

    let diff_report = diff::diff_snapshots(&baseline, &current);
    
    println!("chainproof: diff report");
    println!("  Baseline: {} binaries, {} lockfiles", baseline.binaries.len(), baseline.lockfiles.len());
    println!("  Current: {} binaries, {} lockfiles", current.binaries.len(), current.lockfiles.len());
    println!();
    
    for line in diff_report.to_strings() {
        println!("{}", line);
    }

    if !diff_report.has_diffs() {
        std::process::exit(0);
    } else {
        std::process::exit(1);
    }
}

fn print_usage() {
    println!("chainproof v0.1.0 - supply-chain integrity monitor");
    println!();
    println!("USAGE:");
    println!("  chainproof init                    Create baseline snapshot of environment");
    println!("  chainproof verify                  Verify environment against baseline");
    println!("  chainproof diff [--baseline PATH]  Show differences between snapshots");
    println!("  chainproof help                    Show this help message");
}
