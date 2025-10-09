
// use risc0_zkvm::{default_prover, ExecutorEnv};
// use serde_json;
// use std::io;

// use std::fs;

// fn main() {
//     println!("Enter path to guest ELF file:");

//     let mut path = String::new();
//     std::io::stdin().read_line(&mut path).expect("Failed to read input");
//     let path = path.trim();
   
//     let elf_bytes = fs::read(path).expect("shobhaaaaaa");
//     let guest_elf: &[u8] = &elf_bytes;
//     tracing_subscriber::fmt()
//         .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
//         .init();

//     println!("Enter a number a:");

//     let mut buffer = String::new();
//     io::stdin().read_line(&mut buffer).expect("Failed to read a");
//     let a: f32 = buffer.trim().parse().expect("Please enter a valid number");
//     println!("You entered: {}", a);
//     println!("Enter a number b:");

//     let mut buffer = String::new();
//     io::stdin().read_line(&mut buffer).expect("Failed to read a");

//    let b: f32 = buffer.trim().parse().expect("Please enter a valid number");

//     println!("You entered: {}", b);

  
    
//     let env = ExecutorEnv::builder()
//         .write(&a)
//         .unwrap()
//         .write(&b)
//         .unwrap()
//         .build()
//         .unwrap();

//     // Create prover and generate proof
//     let prover = default_prover();
//     let prove_info = prover.prove(env, guest_elf).unwrap();
//     let receipt = prove_info.receipt;

//     let output: f32 = receipt.journal.decode().unwrap();
//     println!("{}", output);

//     let proof_json =
//         serde_json::to_string(&receipt).expect("Failed to serialize receipt");

    
//     std::fs::write("proof.json", &proof_json).expect("Failed to write proof file");
//     println!("Proof saved to proof.json");

//     // let proof_bytes = bincode::serialize(&receipt).expect("Failed to serialize receipt to binary");
//     // fs::write("proof.bin", &proof_bytes).expect("Failed to write proof.bin");
//     // println!("proof.bin saved");

//     // let loaded_receipt: risc0_zkvm::Receipt =
//     //     serde_json::from_str(&proof_json).expect("Failed to deserialize proof");

//     // // Verify
//     // loaded_receipt.verify(GUEST_ID).unwrap();
//     // println!("Guest id : {:?}", GUEST_ID);
//     // println!("✅ Proof verified successfully!");
// }



use risc0_zkvm::{default_prover, ExecutorEnv};
use risc0_zkvm::serde::from_slice;
use serde_json;
use std::fs;
use std::io;

fn main() {
    println!("Enter path to guest ELF file:");
    let mut path = String::new();
    io::stdin().read_line(&mut path).expect("Failed to read input");
    let path = path.trim();
    let elf_bytes = fs::read(path).expect("Failed to read ELF file");
    let guest_elf: &[u8] = &elf_bytes;

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    println!("Select model type (1=linear, 2=multiple, 3=polynomial, 4=logistic):");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read model type");
    let model_type: u32 = buffer.trim().parse().expect("Please enter a valid number");

    println!("Enter weights (comma-separated, e.g., 1.0,2.0,3.0):");
    buffer.clear();
    io::stdin().read_line(&mut buffer).expect("Failed to read weights");
    let weights: Vec<f32> = buffer
        .trim()
        .split(',')
        .map(|s| s.parse::<f32>().expect("Invalid weight"))
        .collect();

    println!("Enter bias (b):");
    buffer.clear();
    io::stdin().read_line(&mut buffer).expect("Failed to read bias");
    let b: f32 = buffer.trim().parse().expect("Please enter a valid number");

    let env = ExecutorEnv::builder()
        .write(&model_type)
        .unwrap()
        .write(&weights)
        .unwrap()
        .write(&b)
        .unwrap()
        .build()
        .unwrap();

    let prover = default_prover();
    let prove_info = prover.prove(env, &guest_elf).unwrap();
    let receipt = prove_info.receipt;

    let output: Vec<(f32, f32)> = from_slice(receipt.journal.bytes.as_slice()).unwrap();

    println!("\n======= Inference Results =======");
    for (i, (y_pred, y_true)) in output.iter().enumerate() {
        println!("Sample {} => Predicted: {}, True: {}", i + 1, y_pred, y_true);
    }
    println!("================================\n");

    let proof_json = serde_json::to_string_pretty(&receipt).expect("Failed to serialize receipt");
    fs::write("proof.json", &proof_json).expect("Failed to write proof file");
    println!("✅ Proof saved to proof.json");
}
