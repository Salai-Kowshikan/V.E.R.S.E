
use risc0_zkvm::{default_prover, ExecutorEnv};
use serde_json;
use std::io;

use std::fs;

fn main() {
    println!("Enter path to guest ELF file:");

    let mut path = String::new();
    std::io::stdin().read_line(&mut path).expect("Failed to read input");
    let path = path.trim();
   
    let elf_bytes = fs::read(path).expect("Failed to read ELF file");
    let guest_elf: &[u8] = &elf_bytes;
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    println!("Enter a number a:");

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read a");
    let a: f32 = buffer.trim().parse().expect("Please enter a valid number");
    println!("You entered: {}", a);
    println!("Enter a number b:");

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read a");

   let b: f32 = buffer.trim().parse().expect("Please enter a valid number");

    println!("You entered: {}", b);

  
    
    let env = ExecutorEnv::builder()
        .write(&a)
        .unwrap()
        .write(&b)
        .unwrap()
        .build()
        .unwrap();

    // Create prover and generate proof
    let prover = default_prover();
    let prove_info = prover.prove(env, guest_elf).unwrap();
    let receipt = prove_info.receipt;

    let output: f32 = receipt.journal.decode().unwrap();
    println!("{}", output);

    let proof_json =
        serde_json::to_string(&receipt).expect("Failed to serialize receipt");

    
    std::fs::write("proof.json", &proof_json).expect("Failed to write proof file");
    println!("Proof saved to proof.json");

    // let proof_bytes = bincode::serialize(&receipt).expect("Failed to serialize receipt to binary");
    // fs::write("proof.bin", &proof_bytes).expect("Failed to write proof.bin");
    // println!("proof.bin saved");

    // let loaded_receipt: risc0_zkvm::Receipt =
    //     serde_json::from_str(&proof_json).expect("Failed to deserialize proof");

    // // Verify
    // loaded_receipt.verify(GUEST_ID).unwrap();
    // println!("Guest id : {:?}", GUEST_ID);
    // println!("✅ Proof verified successfully!");
}


