// use std::env;
// use std::fs;
// use std::path::Path;
// use std::mem;

// use risc0_zkvm::{default_prover, ExecutorEnv};
// use risc0_zkvm::serde::to_vec;
// use methods::{GUEST_CODE_FOR_ZK_PROOF_ELF, GUEST_CODE_FOR_ZK_PROOF_ID};

// #[repr(C)]
// #[derive(Debug, Clone, Copy)]
// pub struct Node {
//     pub feature_index: u32,
//     pub threshold: f32,
//     pub left: i32,
//     pub right: i32,
//     pub class_label: i32,
// }

// // Parse raw Python-generated nodes
// fn parse_nodes_from_raw(file_bytes: &[u8]) -> Vec<Node> {
//     let node_size = mem::size_of::<Node>();
//     let mut nodes = Vec::new();

//     for chunk in file_bytes.chunks_exact(node_size) {
//         let feature_index = u32::from_le_bytes(chunk[0..4].try_into().unwrap());
//         let threshold = f32::from_le_bytes(chunk[4..8].try_into().unwrap());
//         let left = i32::from_le_bytes(chunk[8..12].try_into().unwrap());
//         let right = i32::from_le_bytes(chunk[12..16].try_into().unwrap());
//         let class_label = i32::from_le_bytes(chunk[16..20].try_into().unwrap());

//         nodes.push(Node {
//             feature_index,
//             threshold,
//             left,
//             right,
//             class_label,
//         });
//     }

//     nodes
// }

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let model_path = "iris_tree_nodes.bin";

//     println!("Current directory: {:?}", env::current_dir()?);

//     if !Path::new(model_path).exists() {
//         println!("⚠️ File not found: {}", model_path);
//         return Ok(());
//     }

//     // Read raw bytes
//     let raw_bytes = fs::read(model_path)?;
//     println!("File size: {} bytes", raw_bytes.len());

//     let nodes = parse_nodes_from_raw(&raw_bytes);
//     println!("Parsed {} nodes locally:", nodes.len());
//     for (i, node) in nodes.iter().enumerate() {
//         println!(
//             "Node {}: feature_index={}, threshold={}, left={}, right={}, class_label={}",
//             i, node.feature_index, node.threshold, node.left, node.right, node.class_label
//         );
//     }

//     // Send raw bytes to guest
//     let env = ExecutorEnv::builder()
//         .write(&raw_bytes)?  // guest reads raw bytes
//         .build()?;

//     // Run prover
//     let prover = default_prover();
//     let prove_info = prover.prove(env, GUEST_CODE_FOR_ZK_PROOF_ELF)?;

//     // Decode predictions from guest
//     let predicted_classes: Vec<i32> = prove_info.receipt.journal.decode()?;
//     println!("Predicted classes from guest: {:?}", predicted_classes);

//     // Expected validation labels
//     let expected_classes = [0, 1, 2, 2, 0];
//     for (i, &pred) in predicted_classes.iter().enumerate() {
//         let expected = expected_classes[i];
//         println!(
//             "Sample {}: Predicted = {}, Expected = {} => {}",
//             i + 1,
//             pred,
//             expected,
//             if pred == expected { "✅" } else { "❌" }
//         );
//     }

//     // Verify proof
//     prove_info.receipt.verify(GUEST_CODE_FOR_ZK_PROOF_ID)?;
//     println!("Proof verified successfully!");

//     Ok(())
// }






//beta






use std::{fs, path::Path};
use risc0_zkvm::{default_prover, ExecutorEnv};
use methods::{GUEST_CODE_FOR_ZK_PROOF_ELF, GUEST_CODE_FOR_ZK_PROOF_ID};

// Node struct for host parsing
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Node {
    pub feature_index: u32,
    pub threshold: f32,
    pub left: i32,
    pub right: i32,
    pub class_label: i32,
}

// Parse raw node bytes
fn parse_nodes_from_raw(raw_bytes: &[u8]) -> Vec<Node> {
    let node_size = 20; // must match guest
    let mut nodes = Vec::new();

    for chunk in raw_bytes.chunks_exact(node_size) {
        let feature_index = u32::from_le_bytes(chunk[0..4].try_into().unwrap());
        let threshold = f32::from_le_bytes(chunk[4..8].try_into().unwrap());
        let left = i32::from_le_bytes(chunk[8..12].try_into().unwrap());
        let right = i32::from_le_bytes(chunk[12..16].try_into().unwrap());
        let class_label = i32::from_le_bytes(chunk[16..20].try_into().unwrap());

        nodes.push(Node {
            feature_index,
            threshold,
            left,
            right,
            class_label,
        });
    }
    nodes
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let model_path = "iris_tree_nodes.bin";

    if !Path::new(model_path).exists() {
        panic!("Model file not found: {}", model_path);
    }

    // Read bytes from Python-generated file
    let raw_bytes = fs::read(model_path)?;
    println!("Read {} bytes from file", raw_bytes.len());

    let nodes = parse_nodes_from_raw(&raw_bytes);
    println!("Parsed {} nodes locally", nodes.len());

    for (i, node) in nodes.iter().enumerate() {
        println!(
            "Node {}: feature_index={}, threshold={}, left={}, right={}, class_label={}",
            i, node.feature_index, node.threshold, node.left, node.right, node.class_label
        );
    }

    // Setup guest env
    let env = ExecutorEnv::builder()
        .write(&raw_bytes)? // pass exactly raw bytes
        .build()?;

    // Run prover
    let prover = default_prover();
    let prove_info = prover.prove(env, GUEST_CODE_FOR_ZK_PROOF_ELF)?;

    // Decode predictions from guest
    let predicted_classes: Vec<i32> = prove_info.receipt.journal.decode()?;
    println!("Predicted classes from guest: {:?}", predicted_classes);

    // Compare with expected labels
    let expected_classes = [0, 1, 2, 2, 0];
    for (i, &pred) in predicted_classes.iter().enumerate() {
        let expected = expected_classes[i];
        println!(
            "Sample {}: Predicted = {}, Expected = {} => {}",
            i + 1,
            pred,
            expected,
            if pred == expected { "✅" } else { "❌" }
        );
    }

    // Verify proof
    prove_info.receipt.verify(GUEST_CODE_FOR_ZK_PROOF_ID)?;
    println!("Proof verified successfully!");

    Ok(())
}
