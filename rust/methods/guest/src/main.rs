// use risc0_zkvm::guest::env;

// fn main() {
//     // TODO: Implement your guest code here

//     // read the input
//     let input: u32 = env::read();

//     // TODO: do something with the input

//     // write public output to the journal
//     env::commit(&input);
// }





//hardcoded tree




// use risc0_zkvm::guest::env;

// // Class names array (indexed)
// const CLASS_NAMES: [&str; 3] = ["setosa", "versicolor", "virginica"];

// // Prediction function using dynamic thresholds
// fn predict_tree(features: &[f32; 4], thresholds: &[f32; 3]) -> u32 {
//     // thresholds[0] = petal_length split for setosa
//     // thresholds[1] = petal_width split for versicolor vs virginica
//     // thresholds[2] = petal_length split for versicolor vs virginica

//     if features[2] <= thresholds[0] {
//         0 // setosa
//     } else if features[3] <= thresholds[1] {
//         if features[2] <= thresholds[2] {
//             1 // versicolor
//         } else {
//             2 // virginica
//         }
//     } else {
//         2 // virginica
//     }
// }

// fn main() {
//     // 1️⃣ Read thresholds from host
//     let thresholds: [f32; 3] = env::read();

//     // 2️⃣ Hardcoded validation dataset (5 samples)
//     let validation_data: [[f32; 4]; 5] = [
//         [5.1, 3.5, 1.4, 0.2], // setosa
//         [4.9, 3.0, 1.4, 0.2], // setosa
//         [5.8, 2.7, 4.1, 1.0], // versicolor
//         [6.0, 2.7, 5.1, 1.6], // virginica
//         [6.3, 3.3, 6.0, 2.5], // virginica
//     ];

//     // 3️⃣ Arrays to store predictions
//     let mut numeric_classes: [u32; 5] = [0; 5];
//     let mut class_name_indices: [u32; 5] = [0; 5];

//     // 4️⃣ Predict for each sample using dynamic thresholds
//     for (i, sample) in validation_data.iter().enumerate() {
//         let class_num = predict_tree(sample, &thresholds);
//         numeric_classes[i] = class_num;
//         class_name_indices[i] = class_num; // index into CLASS_NAMES
//     }

//     // 5️⃣ Commit predictions to journal
//     env::commit(&(numeric_classes, class_name_indices));
// }



// #![no_std]
// #![no_main]

// extern crate alloc;

// use alloc::vec;
// use alloc::vec::Vec;
// use risc0_zkvm::guest::env;
// use serde::{Deserialize, Serialize};

// #[derive(Debug, Serialize, Deserialize)]
// pub struct TreeNode {
//     pub id: usize,
//     pub feature: Option<usize>,
//     pub threshold: Option<f32>,
//     pub left: Option<usize>,
//     pub right: Option<usize>,
//     pub value: Option<Vec<Vec<f32>>>,
// }

// #[derive(Debug)]
// pub struct Sample {
//     pub features: Vec<f32>,
//     pub expected: u32,
// }

// // Hardcoded dataset in guest
// fn get_dataset() -> Vec<Sample> {
//     vec![
//         Sample { features: vec![5.1, 3.5, 1.4, 0.2], expected: 0 },
//         Sample { features: vec![4.9, 3.0, 1.4, 0.2], expected: 0 },
//         Sample { features: vec![6.0, 2.2, 4.0, 1.0], expected: 1 },
//         Sample { features: vec![5.9, 3.0, 5.1, 1.8], expected: 2 },
//         Sample { features: vec![6.5, 3.0, 5.2, 2.0], expected: 2 },
//     ]
// }

// fn traverse_tree(tree: &[TreeNode], sample: &[f32]) -> u32 {
//     let mut node_index = 0;
//     loop {
//         let node = &tree[node_index];

//         // If this node is a leaf, return the predicted class
//         if let Some(value) = &node.value {
//             let max_index = value[0]
//                 .iter()
//                 .enumerate()
//                 .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
//                 .unwrap()
//                 .0;
//             return max_index as u32;
//         }

//         // If this is an internal node
//         if let Some(feature) = node.feature {
//             let next_index = if sample[feature] <= node.threshold.unwrap_or(0.0) {
//                 node.left
//             } else {
//                 node.right
//             };

//             match next_index {
//                 Some(idx) => node_index = idx,
//                 None => panic!("Invalid tree: node with no child and no value"),
//             }
//         } else {
//             panic!("Invalid tree: node with no feature and no value");
//         }
//     }
// }


// risc0_zkvm::guest::entry!(main);

// fn main() {
//     // Read tree from host
//     let tree: Vec<TreeNode> = env::read();

//     let dataset = get_dataset();
//     let mut predictions = Vec::new();

//     for sample in dataset.iter() {
//         let pred = traverse_tree(&tree, &sample.features);
//         predictions.push((pred, sample.expected));
//     }

//     env::commit(&predictions);
// }




#![no_std]
#![no_main]

extern crate alloc;

use alloc::{vec, vec::Vec, string::String};
use alloc::format;
use alloc::string::ToString;

use risc0_zkvm::guest::env;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeNode {
    pub id: usize,
    pub feature: Option<usize>,
    pub threshold: Option<f32>,
    pub left: Option<usize>,
    pub right: Option<usize>,
    pub value: Option<Vec<Vec<f32>>>,
}

#[derive(Debug)]
pub struct Sample {
    pub features: Vec<f32>,
    pub expected: u32,
}

// Hardcoded dataset
fn get_dataset() -> Vec<Sample> {
    vec![
        Sample { features: vec![5.1, 3.5, 1.4, 0.2], expected: 0 },
        Sample { features: vec![4.9, 3.0, 1.4, 0.2], expected: 0 },
        Sample { features: vec![6.0, 2.2, 4.0, 1.0], expected: 1 },
        Sample { features: vec![5.9, 3.0, 5.1, 1.8], expected: 2 },
        Sample { features: vec![6.5, 3.0, 5.2, 2.0], expected: 2 },
    ]
}

// Simple helper to log messages
fn log_msg(msg: &str) {
    env::log(msg);
}

// Traverse the decision tree for a sample
fn traverse_tree(tree: &[TreeNode], sample: &[f32]) -> u32 {
    let mut node_index = 0;

    loop {
        let node = &tree[node_index];

        // Log visiting node
        let mut log_line = String::from("Visiting node ");
        log_line.push_str(&node.id.to_string());
        if let Some(f) = node.feature {
            log_line.push_str(", feature=");
            log_line.push_str(&f.to_string());
        }
        log_msg(&log_line);

        // Leaf node
        if let Some(value) = &node.value {
            // Find index of max value
            let max_index = value[0]
                .iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                .unwrap()
                .0;

            let mut leaf_msg = String::from("Leaf reached, returning class ");
            leaf_msg.push_str(&max_index.to_string());
            log_msg(&leaf_msg);

            return max_index as u32;
        }

        // Internal node
        if let Some(feature) = node.feature {
            let sample_val = sample[feature];
            let threshold = node.threshold.unwrap_or(0.0);
            let next_index = if sample_val <= threshold { node.left } else { node.right };

            log_msg("Internal node decision made");

            match next_index {
                Some(idx) => node_index = idx,
                None => panic!("Invalid tree: node {} has no child and no value", node.id),
            }
        } else {
            panic!("Invalid tree: node {} has no feature and no value", node.id);
        }
    }
}

risc0_zkvm::guest::entry!(main);

fn main() {
    // Read tree from host
    let tree: Vec<TreeNode> = env::read();

    // Get dataset
    let dataset = get_dataset();
    let mut predictions = Vec::new();

    for (i, sample) in dataset.iter().enumerate() {
        log_msg(&format!("Processing sample {}", i));
        let pred = traverse_tree(&tree, &sample.features);
        predictions.push((pred, sample.expected));
    }

    // Commit predictions to host
    env::commit(&predictions);
}
