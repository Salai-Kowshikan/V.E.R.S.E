// #![no_std]
// #![no_main]

// extern crate alloc;
// use alloc::{vec::Vec, format};
// use risc0_zkvm::guest::env;
// use core::convert::TryInto;

// #[derive(Debug, Clone, Copy)]
// pub struct Node {
//     pub feature_index: u32,
//     pub threshold: f32,
//     pub left: i32,
//     pub right: i32,
//     pub class_label: i32,
// }

// // Validation samples with actual labels
// const VALIDATION_DATA: [([f32; 4], i32); 5] = [
//     ([5.1, 3.5, 1.4, 0.2], 0),
//     ([7.0, 3.2, 4.7, 1.4], 1),
//     ([6.3, 3.3, 6.0, 2.5], 2),
//     ([5.8, 2.7, 5.1, 1.9], 2),
//     ([5.0, 3.4, 1.5, 0.2], 0),
// ];

// fn eval_tree(sample: &[f32; 4], nodes: &Vec<Node>) -> i32 {
//     let mut idx: usize = 0;
//     loop {
//         let node = &nodes[idx];
//         if node.class_label != -1 {
//             return node.class_label;
//         }
//         let feat_idx = node.feature_index as usize;
//         if sample[feat_idx] <= node.threshold {
//             idx = node.left as usize;
//         } else {
//             idx = node.right as usize;
//         }
//     }
// }

// risc0_zkvm::guest::entry!(main);

// fn main() {
//     // Read raw bytes from host
//     let raw_bytes: Vec<u8> = env::read::<Vec<u8>>();

//     let node_size = 20;
//     let mut nodes: Vec<Node> = Vec::new();

//     for chunk in raw_bytes.chunks_exact(node_size) {
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

//     let mut predictions: Vec<i32> = Vec::new();
//     for (sample, actual_label) in VALIDATION_DATA.iter() {
//         let pred = eval_tree(sample, &nodes);
//         predictions.push(pred);

//         // Logging using alloc::format
//         if pred == *actual_label {
//             env::log(&format!("✅ Sample predicted correctly: {}", pred));
//         } else {
//             env::log(&format!("❌ Sample predicted incorrectly: {} (expected {})", pred, actual_label));
//         }
//     }

//     env::commit_slice(&predictions);
// }

#![no_std]
#![no_main]

extern crate alloc;
use alloc::{vec::Vec};
use risc0_zkvm::guest::env;
use core::convert::TryInto;

#[derive(Debug, Clone, Copy)]
pub struct Node {
    pub feature_index: u32,
    pub threshold: f32,
    pub left: i32,
    pub right: i32,
    pub class_label: i32,
}

// Validation samples (features only; labels are on host)
const VALIDATION_DATA: [[f32; 4]; 5] = [
    [5.1, 3.5, 1.4, 0.2],
    [7.0, 3.2, 4.7, 1.4],
    [6.3, 3.3, 6.0, 2.5],
    [5.8, 2.7, 5.1, 1.9],
    [5.0, 3.4, 1.5, 0.2],
];

// Evaluate one sample through the tree
fn eval_tree(sample: &[f32; 4], nodes: &Vec<Node>) -> i32 {
    let mut idx: i32 = 0; // i32 because -1 can indicate leaf
    loop {
        let node = &nodes[idx as usize];
        if node.class_label != -1 {
            return node.class_label;
        }
        let feat_idx = node.feature_index as usize;
        idx = if sample[feat_idx] <= node.threshold {
            node.left
        } else {
            node.right
        };
    }
}

risc0_zkvm::guest::entry!(main);
fn main() {
    // Read raw node bytes from host
    let raw_bytes: Vec<u8> = env::read::<Vec<u8>>();

    let node_size = 20;
    let mut nodes: Vec<Node> = Vec::new();

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

    // Evaluate all samples
    let mut predictions: Vec<i32> = Vec::new();
    for sample in VALIDATION_DATA.iter() {
        let pred = eval_tree(sample, &nodes);
        predictions.push(pred);
    }

    // Commit predictions to host
    env::commit_slice(&predictions);
}
