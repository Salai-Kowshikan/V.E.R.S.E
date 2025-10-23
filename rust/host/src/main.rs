
use methods::{GUEST_CODE_FOR_ZK_PROOF_ELF, GUEST_CODE_FOR_ZK_PROOF_ID};
use risc0_zkvm::{default_prover, ExecutorEnv};
use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeNode {
    pub id: usize,
    pub feature: Option<usize>,   // None if leaf
    pub threshold: Option<f64>,   // None if leaf
    pub left: Option<usize>,      // None if leaf
    pub right: Option<usize>,     // None if leaf
    pub value: Vec<Vec<f64>>,     // non-empty if leaf
}

fn main() {
    // Load JSON tree
    let tree_json = fs::read_to_string("tree.json").expect("Failed to read tree.json");
    let tree: Vec<TreeNode> = serde_json::from_str(&tree_json).expect("Failed to parse JSON");

    // Build environment
    let env = ExecutorEnv::builder()
        .write(&tree).unwrap()
        .build().unwrap();

    let prover = default_prover();
    let prove_info = prover
        .prove(env, GUEST_CODE_FOR_ZK_PROOF_ELF)
        .unwrap();

    let receipt = prove_info.receipt;

    // Decode predictions
    let predictions: Vec<(u32, u32)> = receipt.journal.decode().unwrap();

    println!("Sample | Predicted | Expected");
    println!("------------------------------");
    for (i, (pred, expected)) in predictions.iter().enumerate() {
        println!("{:<6} | {:<9} | {}", i, pred, expected);
    }

    receipt.verify(GUEST_CODE_FOR_ZK_PROOF_ID).unwrap();
    println!("✅ ZKP verified successfully");
}
