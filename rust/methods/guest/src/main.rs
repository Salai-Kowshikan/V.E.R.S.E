
#![no_std]
#![no_main]

extern crate alloc;

use alloc::{vec, vec::Vec};
use risc0_zkvm::guest::env;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TreeNode {
    pub id: usize,
    pub feature: Option<usize>,   // None if leaf
    pub threshold: Option<f64>,   // None if leaf
    pub left: Option<usize>,      // None if leaf
    pub right: Option<usize>,     // None if leaf
    pub value: Vec<Vec<f64>>,     // non-empty if leaf
}

#[derive(Debug)]
pub struct Sample {
    pub features: Vec<f64>,
    pub expected: u32,
}

fn get_dataset() -> Vec<Sample> {
    vec![
        Sample { features: vec![5.1, 3.5, 1.4, 0.2], expected: 0 },
        Sample { features: vec![4.9, 3.0, 1.4, 0.2], expected: 0 },
        Sample { features: vec![6.0, 2.2, 4.0, 1.0], expected: 1 },
        Sample { features: vec![5.9, 3.0, 5.1, 1.8], expected: 2 },
        Sample { features: vec![6.5, 3.0, 5.2, 2.0], expected: 2 },
    ]
}

fn traverse_tree(tree: &[TreeNode], sample: &[f64]) -> u32 {
    let mut node_index = 0;

    loop {
        let node = &tree[node_index];

        // Leaf node detection
        if !node.value.is_empty() {
            return node.value[0]
                .iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                .unwrap()
                .0 as u32;
        }

        // Internal node: must have all fields
        let feature = node.feature.expect("Internal node must have feature");
        let threshold = node.threshold.expect("Internal node must have threshold");
        let left = node.left.expect("Internal node must have left child");
        let right = node.right.expect("Internal node must have right child");

        node_index = if sample[feature] <= threshold { left } else { right };
    }
}

risc0_zkvm::guest::entry!(main);

fn main() {
    // Read tree from host
    let tree: Vec<TreeNode> = env::read();
    let dataset = get_dataset();
    let mut predictions = Vec::new();

    for sample in dataset.iter() {
        let pred = traverse_tree(&tree, &sample.features);
        predictions.push((pred, sample.expected));
    }

    // Commit predictions to host
    env::commit(&predictions);
}
