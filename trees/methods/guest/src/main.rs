
// use risc0_zkvm::guest::env;

// // Class mapping: C=0, B=1, A=2, O=3
// type Class = u32;

// /// Simple decision tree prediction
// fn decision_tree_predict(validation_data: &[u32], thresholds: &[u32]) -> Vec<Class> {
//     validation_data
//         .iter()
//         .map(|&x| {
//             if x > thresholds[0] {
//                 2 // Class A
//             } else if x > thresholds[1] {
//                 1 // Class B
//             } else if x > thresholds[2] {
//                 0 // Class C
//             } else {
//                 3 // Class O
//             }
//         })
//         .collect()
// }

// /// Random forest prediction (majority vote)
// fn random_forest_predict(validation_data: &[u32], forest_thresholds: &[Vec<u32>]) -> Vec<Class> {
//     validation_data
//         .iter()
//         .map(|&x| {
//             // For each tree, predict a class
//             let mut votes = vec![0; 4];
//             for thresholds in forest_thresholds.iter() {
//                 let class = if x > thresholds[0] {
//                     2
//                 } else if x > thresholds[1] {
//                     1
//                 } else if x > thresholds[2] {
//                     0
//                 } else {
//                     3
//                 };
//                 votes[class as usize] += 1;
//             }
//             // Return class with max votes
//             votes.iter().enumerate().max_by_key(|&(_, &v)| v).unwrap().0 as u32
//         })
//         .collect()
// }

// /// Gradient boosted tree prediction
// fn gradient_boost_predict(
//     validation_data: &[u32],
//     trees: &[Vec<u32>],
//     tree_weights: &[f32],
// ) -> Vec<Class> {
//     validation_data
//         .iter()
//         .map(|&x| {
//             let mut score = [0f32; 4];
//             for (thresholds, &weight) in trees.iter().zip(tree_weights.iter()) {
//                 let class = if x > thresholds[0] {
//                     2
//                 } else if x > thresholds[1] {
//                     1
//                 } else if x > thresholds[2] {
//                     0
//                 } else {
//                     3
//                 };
//                 score[class as usize] += weight;
//             }
//             // Return class with highest score
//             score.iter().enumerate().max_by(|a, b| a.1.partial_cmp(b.1).unwrap()).unwrap().0 as u32
//         })
//         .collect()
// }
// fn main() {
//     // Guest validation dataset
//     let validation_data: [(u32, u32); 7] = [
//         (34, 1),
//         (45, 1),
//         (33, 1),
//         (12, 0),
//         (23, 1),
//         (70, 2),
//         (120, 3),
//     ];
    
//     // Read tree type from host: 0=decision_tree, 1=random_forest, 2=gradient_boost
//     let tree_type: u32 = env::read();

// // Extract features from validation_data
// let features: Vec<u32> = validation_data.iter().map(|(feature, _label)| *feature).collect();

// // Then call your functions
// let predicted_classes: Vec<u32> = match tree_type {
//     0 => {
//         let thresholds: [u32; 3] = env::read();
//         decision_tree_predict(&features, &thresholds)
//     }
//     1 => {
//         let num_trees: u32 = env::read();
//         let mut forest_thresholds: Vec<Vec<u32>> = Vec::new();
//         for _ in 0..num_trees {
//             let tree: [u32; 3] = env::read();
//             forest_thresholds.push(tree.to_vec());
//         }
//         random_forest_predict(&features, &forest_thresholds)
//     }
//     2 => {
//         let num_trees: u32 = env::read();
//         let mut trees: Vec<Vec<u32>> = Vec::new();
//         let mut weights: Vec<f32> = Vec::new();
//         for _ in 0..num_trees {
//             let tree: [u32; 3] = env::read();
//             trees.push(tree.to_vec());
//         }
//         weights = env::read();
//         gradient_boost_predict(&features, &trees, &weights)
//     }
//     _ => panic!("Unknown tree type"),
// };


//     // Compare predictions with actual labels and print results
//     for (i, &(feature, actual)) in validation_data.iter().enumerate() {
//         let predicted = predicted_classes[i];
//         if predicted == actual {
//             println!("Data {} (feature={}): Prediction correct ✅", i+1, feature);
//         } else {
//             println!(
//                 "Data {} (feature={}): Prediction wrong ❌, predicted={}, actual={}",
//                 i+1, feature, predicted, actual
//             );
//         }
//     }

//     // Commit predicted classes
//     env::commit(&predicted_classes);
// }

#![no_std]
#![no_main]

extern crate alloc;
use alloc::vec::Vec;
use risc0_zkvm::guest::env;
use risc0_zkvm::serde::{Serialize, Deserialize};

#[repr(C)]
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Node {
    pub feature_index: u32,
    pub threshold: f32,
    pub left: i32,
    pub right: i32,
    pub class_label: i32,
}

const VALIDATION_DATA: [[f32; 4]; 5] = [
    [5.1, 3.5, 1.4, 0.2],
    [7.0, 3.2, 4.7, 1.4],
    [6.3, 3.3, 6.0, 2.5],
    [5.8, 2.7, 5.1, 1.9],
    [5.0, 3.4, 1.5, 0.2],
];

const VALIDATION_LABELS: [i32; 5] = [0, 1, 2, 2, 0];

fn eval_tree(sample: &[f32; 4], nodes: &Vec<Node>) -> i32 {
    let mut idx = 0;
    loop {
        let node = &nodes[idx];
        if node.class_label != -1 {
            return node.class_label;
        }
        if sample[node.feature_index as usize] <= node.threshold {
            idx = node.left as usize;
        } else {
            idx = node.right as usize;
        }
    }
}

risc0_zkvm::guest::entry!(main);

fn main() {
    // Deserialize model nodes from host
    let nodes: Vec<Node> = env::read::<Vec<Node>>();

    for (i, sample) in VALIDATION_DATA.iter().enumerate() {
        let predicted = eval_tree(sample, &nodes);
        let actual = VALIDATION_LABELS[i];
        env::commit_slice(&[predicted as u32]);
        env::commit_slice(&[actual as u32]);
    }
}
