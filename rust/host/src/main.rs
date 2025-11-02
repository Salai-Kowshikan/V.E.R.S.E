
use methods::{GUEST_CODE_FOR_ZK_PROOF_ELF, GUEST_CODE_FOR_ZK_PROOF_ID};
use risc0_zkvm::{default_prover, ExecutorEnv};
use std::fs;
use std::time::Instant;

fn main() {
    // Get tree path from first CLI arg or default to "tree.json"
    let tree_path = std::env::args().nth(1).unwrap_or_else(|| "tree.json".to_string());
    println!("[host] Using tree JSON path: {}", tree_path);

    let read_start = Instant::now();
    let tree_json = fs::read_to_string(&tree_path).expect("Failed to read tree.json");
    println!(
        "[host] Loaded JSON ({} bytes) in {:.2?}",
        tree_json.len(),
        read_start.elapsed()
    );
    let preview: String = tree_json.chars().take(120).collect();
    println!("[host] JSON preview: {}{}", preview, if tree_json.len() > 120 { "..." } else { "" });

    println!("[host] Building zkVM executor environment...");
    let env = ExecutorEnv::builder()
        .write(&tree_path).unwrap()
        .write(&tree_json).unwrap()
        .build().unwrap();
    println!("[host] Executor environment ready.");

    let prover = default_prover();
    println!("[host] Starting proof generation...");
    let prove_start = Instant::now();
    let prove_info = prover
        .prove(env, GUEST_CODE_FOR_ZK_PROOF_ELF)
        .unwrap();
    println!("[host] Proof generated in {:.2?}", prove_start.elapsed());

    let receipt = prove_info.receipt;
    println!("[host] Decoding journal to predictions...");
    let predictions: Vec<(Vec<f64>, u32)> = receipt.journal.decode().unwrap();
    println!("[host] Decoded {} predictions", predictions.len());

    println!("Sample | PredClass | Prob    | Expected");
    println!("---------------------------------------");
    for (i, (probs, expected)) in predictions.iter().enumerate() {
        let (pred_idx, pred_p) = probs
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(idx, p)| (idx, *p))
            .unwrap_or((usize::MAX, f64::NAN));
        println!("{:<6} | {:<9} | {:<6.3} | {}", i, pred_idx, pred_p, expected);
    }

    println!("[host] Verifying receipt against method ID...");
    let verify_start = Instant::now();
    receipt.verify(GUEST_CODE_FOR_ZK_PROOF_ID).unwrap();
    println!("✅ ZKP verified successfully in {:.2?}", verify_start.elapsed());
}
