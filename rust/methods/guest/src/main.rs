
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

fn build_id_index(nodes: &Vec<TreeNode>) -> Vec<usize> {
    // Build a dense id->index map to handle arbitrary node ids
    let mut max_id = 0usize;
    for n in nodes.iter() { if n.id > max_id { max_id = n.id; } }
    let mut map = vec![usize::MAX; max_id + 1];
    for (idx, n) in nodes.iter().enumerate() { map[n.id] = idx; }
    map
}

fn traverse_tree(nodes: &Vec<TreeNode>, id_index: &Vec<usize>, x: &[f64]) -> Vec<f64> {
    let mut current_id: usize = 0;
    loop {
        // Resolve node by id using the index map
        if current_id >= id_index.len() { panic!("Unknown node id"); }
        let idx = id_index[current_id];
        if idx == usize::MAX { panic!("Unmapped node id"); }
        let node = &nodes[idx];

        if node.feature.is_none() {
            return node.value[0].clone();
        }

        let feat = node.feature.unwrap();
        let thr = node.threshold.unwrap();
        let xf = x[feat];

        if xf <= thr {
            current_id = node.left.expect("Missing left child");
        } else {
            current_id = node.right.expect("Missing right child");
        }
    }
}

risc0_zkvm::guest::entry!(main);

fn main() {
    let _tree_path: alloc::string::String = env::read();
    let tree_json: alloc::string::String = env::read();
    let tree: Vec<TreeNode> = match serde_json::from_str(&tree_json) {
        Ok(t) => t,
        Err(_) => panic!("Failed to parse tree JSON in guest"),
    };
    let id_index = build_id_index(&tree);
    let dataset = get_dataset();
    let mut predictions = Vec::new();

    for sample in dataset.iter() {
        let pred = traverse_tree(&tree, &id_index, &sample.features);
        predictions.push((pred, sample.expected));
    }

    env::commit(&predictions);
}
