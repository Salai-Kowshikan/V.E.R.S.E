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

use risc0_zkvm::host::Prover;
use std::fs;
use risc0_zkvm::serde::{Serialize, Deserialize}; // Updated imports

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let model_path = "trees/host/src/iris_tree_nodes.bin";
    let model_bytes = fs::read(model_path)?;
    let mut prover = Prover::new(GUEST_CODE_FOR_ZK_PROOF_ELF, &model_bytes)?;
    let receipt = prover.run()?;
    let journal = receipt.get_journal_vec();
    println!("Journal committed: {:?}", journal);
    Ok(())
}