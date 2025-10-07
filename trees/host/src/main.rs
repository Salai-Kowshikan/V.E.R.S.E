// use methods::{GUEST_CODE_FOR_ZK_PROOF_ELF, GUEST_CODE_FOR_ZK_PROOF_ID};
// use risc0_zkvm::{default_prover, ExecutorEnv};

// fn main() {
//     let tree_type = 1; // 0=decision_tree, 1=random_forest, 2=gradient_boost

//     let mut env_builder = ExecutorEnv::builder();

//     env_builder.write(&tree_type).unwrap();

//     match tree_type {
//         0 => {
//             let thresholds = [100, 45, 20];
//             env_builder.write(&thresholds).unwrap();
//         }
//         1 => {
//             let num_trees: u32 = 2;
//             env_builder.write(&num_trees).unwrap();
//             let trees = [
//                 [100, 45, 20],
//                 [90, 40, 15],
//                 [95, 42, 18],
//             ];
//             for tree in trees.iter() {
//                 env_builder.write(tree).unwrap();
//             }
//         }
//         2 => {
//             let num_trees: u32 = 2;
//             env_builder.write(&num_trees).unwrap();
//             let trees = [
//                 [100, 45, 20],
//                 [90, 40, 15],
//             ];
//             let weights = [0.6f32, 0.4f32];
//             for tree in trees.iter() {
//                 env_builder.write(tree).unwrap();
//             }
//             env_builder.write(&weights).unwrap();
//         }
//         _ => panic!("Unknown tree type"),
//     }

//     let env = env_builder.build().unwrap();
//     let prover = default_prover();
//     let prove_info = prover.prove(env, GUEST_CODE_FOR_ZK_PROOF_ELF).unwrap();

//     let predicted_classes: Vec<u32> = prove_info.receipt.journal.decode().unwrap();
//     println!("Predicted classes: {:?}", predicted_classes);

//     prove_info.receipt.verify(GUEST_CODE_FOR_ZK_PROOF_ID).unwrap();
//     println!("Proof verified successfully!");

// }

// use methods::{GUEST_CODE_FOR_ZK_PROOF_ELF, GUEST_CODE_FOR_ZK_PROOF_ID};
// use risc0_zkvm::{default_prover, ExecutorEnv};
// use std::fs;
// use serde::{Serialize, Deserialize};

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let model_path = "trees/host/src/iris_tree_nodes.bin";
//     let model_bytes = fs::read(model_path)?;

//     // Build environment with input
//     let env = ExecutorEnv::builder()
//         .write(&model_bytes)?
//         .build()?;

//     // Get default prover
//     let prover = default_prover();

//     // Run proof
//     let prove_info = prover.prove(env, GUEST_CODE_FOR_ZK_PROOF_ELF)?;

//     // Decode journal as Vec<u32>
//     let predicted_classes: Vec<u32> = prove_info.receipt.journal.decode()?;
//     println!("Predicted classes: {:?}", predicted_classes);

//     // Verify proof
//     prove_info.receipt.verify(GUEST_CODE_FOR_ZK_PROOF_ID)?;
//     println!("Proof verified successfully!");

//     Ok(())
// }


use std::env;
use std::path::Path;
use std::fs;
use serde::{Serialize, Deserialize};
use bincode;
use risc0_zkvm::{default_prover, ExecutorEnv};
use risc0_zkvm::serde::to_vec;
use methods::{GUEST_CODE_FOR_ZK_PROOF_ELF, GUEST_CODE_FOR_ZK_PROOF_ID};

// Host-side Node struct (must match guest)
#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Node {
    pub feature_index: u32,
    pub threshold: f32,
    pub left: i32,
    pub right: i32,
    pub class_label: i32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1️⃣ Load serialized model nodes
    let model_path = "iris_tree_nodes.bin";
    let file_bytes = fs::read(model_path)?;

    // 🔍 Debug prints before reading
    println!("🔹 Current working directory: {:?}", env::current_dir()?);
    println!("🔹 Trying to read file from path: {}", model_path);

    // Check if file exists
    if !Path::new(model_path).exists() {
        println!("⚠️ File not found at: {}", model_path);
        println!("💡 Try placing it in the same directory as your executable or use an absolute path.");
        return Ok(()); 
    }

    println!("File size: {} bytes", file_bytes.len());
    println!("First 32 bytes: {:?}", &file_bytes[..32.min(file_bytes.len())]);

    let nodes: Vec<Node> = match bincode::deserialize(&file_bytes) {
        Ok(nodes) => nodes,
        Err(e) => {
            eprintln!("Failed to deserialize nodes: {:?}", e);
            return Err(Box::new(e));
        }
    };
    println!("Deserialized {} nodes", nodes.len());

    // 2️⃣ Serialize nodes for guest
    let serialized_nodes = to_vec(&nodes)?;

    // 3️⃣ Build guest execution environment
    let env = ExecutorEnv::builder()
        .write(&serialized_nodes)?
        .build()?;

    // 4️⃣ Run prover
    let prover = default_prover();
    let prove_info = prover.prove(env, GUEST_CODE_FOR_ZK_PROOF_ELF)?;

    // 5️⃣ Decode predictions from guest
    let predicted_classes: Vec<u32> = prove_info.receipt.journal.decode()?;
    println!("Predicted classes from guest: {:?}", predicted_classes);

    // 6️⃣ Compare with expected values
    let expected_classes = [0, 1, 2, 2, 0]; // Change according to your validation set
    for (i, &pred) in predicted_classes.iter().enumerate() {
        println!(
            "Sample {}: Predicted = {}, Expected = {} => {}",
            i + 1,
            pred,
            expected_classes[i],
            if pred == expected_classes[i] { "✅" } else { "❌" }
        );
    }

    // 7️⃣ Verify ZK proof
    prove_info.receipt.verify(GUEST_CODE_FOR_ZK_PROOF_ID)?;
    println!("Proof verified successfully!");

    Ok(())
}
    