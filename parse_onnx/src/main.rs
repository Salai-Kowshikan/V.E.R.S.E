use std::fs;
use std::collections::HashMap;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Node {
    id: usize,
    feature: Option<usize>,
    threshold: Option<f32>,
    left: Option<usize>,
    right: Option<usize>,
    value: Option<Vec<Vec<f32>>>,
}

fn predict(tree: &HashMap<usize, Node>, features: &[f32], node_id: usize) -> usize {
    let node = &tree[&node_id];
    if node.feature.is_none() {
        let row = &node.value.as_ref().unwrap()[0];
        let (idx, _) = row.iter().enumerate().max_by(|a, b| a.1.partial_cmp(b.1).unwrap()).unwrap();
        return idx;
    }

    let feature_idx = node.feature.unwrap();
    let threshold = node.threshold.unwrap();
    if features[feature_idx] <= threshold {
        predict(tree, features, node.left.unwrap())
    } else {
        predict(tree, features, node.right.unwrap())
    }
}

fn main() -> std::io::Result<()> {
    // Load JSON tree
    let data = fs::read_to_string("tree.json")?;
    let nodes: Vec<Node> = serde_json::from_str(&data).unwrap();

    // Convert to map for fast lookup
    let mut tree = HashMap::new();
    for node in nodes {
        tree.insert(node.id, node);
    }

    // Verification samples
    let samples = [
        ([5.1, 3.5, 1.4, 0.2], 0),
        ([4.9, 3.0, 1.4, 0.2], 0),
        ([6.0, 2.2, 4.0, 1.0], 1),
        ([5.9, 3.0, 5.1, 1.8], 2),
        ([6.5, 3.0, 5.2, 2.0], 2),
    ];

    for (features, expected) in samples {
        let pred = predict(&tree, &features, 0);
        println!("Features: {:?}, Predicted: {}, Expected: {}", features, pred, expected);
    }

    Ok(())
}
