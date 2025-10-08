use risc0_zkvm::{default_prover, ExecutorEnv};
use risc0_zkvm::serde::from_slice;
use serde_json;
use std::fs;
use std::time::Instant;
use methods::{LINEARREGRESSION_ELF, LINEARREGRESSION_ID};

fn main() {
    let guest_elf: &[u8] = LINEARREGRESSION_ELF;
    println!("Using embedded guest ELF (LinearRegression)");
    let timer_start = Instant::now();

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();
    let use_opt_flag: u32 = std::env::var("VERSE_OPTIMIZED")
        .ok()
        .map(|v| {
            match v.to_ascii_lowercase().as_str() {
                "1" | "true" | "yes" | "on" => 1,
                _ => 0,
            }
        })
        .unwrap_or(0);
    println!(
        "Inference mode: {}",
        if use_opt_flag == 1 { "optimized (fixed-point)" } else { "unoptimized (float)" }
    );

    let model_type: u32 = 2;
    let weights: Vec<f32> = vec![1.0, -2.0, 0.5];
    let b: f32 = 0.3;
    println!(
        "Benchmark config -> model_type: {}, weights: {:?}, bias: {}",
        model_type, weights, b
    );

    let env = ExecutorEnv::builder()
        // Pass optimization toggle first (guest expects this order)
        .write(&use_opt_flag)
        .unwrap()
        .write(&model_type)
        .unwrap()
        .write(&weights)
        .unwrap()
        .write(&b)
        .unwrap()
        .build()
        .unwrap();

    let prover = default_prover();
    let prove_info = prover.prove(env, guest_elf).unwrap();
    let receipt = prove_info.receipt;

    let elapsed = timer_start.elapsed();
    println!(
        "\n⏱️ Time to generate proof (from ELF load): {:.3} seconds\n",
        elapsed.as_secs_f64()
    );

    let output: Vec<(f32, f32)> = from_slice(receipt.journal.bytes.as_slice()).unwrap();

    println!("\nOutput size: {}", output.len());

    let proof_json = serde_json::to_string_pretty(&receipt).expect("Failed to serialize receipt");
    fs::write("proof.json", &proof_json).expect("Failed to write proof file");
    println!("✅ Proof saved to proof.json");
}
