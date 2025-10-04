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



use methods::{GUEST_CODE_FOR_ZK_PROOF_ELF, GUEST_CODE_FOR_ZK_PROOF_ID};
use risc0_zkvm::{default_prover, ExecutorEnv};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let model_path = "iris_tree_nodes.bin";
    let model_bytes = fs::read(model_path)?;

    // Example ground truth labels (replace with your dataset labels)
    let actual_classes: Vec<u32> = vec![0, 1, 2, 1, 0]; 

    // Build environment and send model bytes
    let env = ExecutorEnv::builder()
        .write(&model_bytes)?
        .build()?;

    // Run prover
    let prover = default_prover();
    let prove_info = prover.prove(env, GUEST_CODE_FOR_ZK_PROOF_ELF)?;

    // Decode predicted results from the journal
    let predicted_classes: Vec<u32> = prove_info.receipt.journal.decode()?;
    println!("Predicted classes: {:?}", predicted_classes);

    // Verify proof
    prove_info.receipt.verify(GUEST_CODE_FOR_ZK_PROOF_ID)?;
    println!("Proof verified successfully!");

    // --- Compare actual vs predicted ---
    println!("\n--- Results ---");
    for (i, (actual, predicted)) in actual_classes.iter().zip(predicted_classes.iter()).enumerate() {
        println!("Sample {} => Actual: {}, Predicted: {}", i, actual, predicted);
    }

    Ok(())
}
