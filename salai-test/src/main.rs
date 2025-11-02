use std::fs;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TreeNode {
    id: usize,
    #[serde(default)]
    feature: Option<usize>,
    #[serde(default)]
    threshold: Option<f64>,
    #[serde(default)]
    left: Option<usize>,
    #[serde(default)]
    right: Option<usize>,
    value: Vec<Vec<f64>>,
}

fn main() {
    let path = "tree.json";
    let text = fs::read_to_string(path).unwrap_or_else(|e| {
        eprintln!("Failed to read {}: {}", path, e);
        std::process::exit(1);
    });

    let nodes: Vec<TreeNode> = match serde_json::from_str(&text) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to parse JSON into TreeNode list: {}", e);
            println!("{}", text);
            std::process::exit(1);
        }
    };

    println!("Decision tree loaded: {} nodes", nodes.len());

    let mut id_index: HashMap<usize, usize> = HashMap::with_capacity(nodes.len());
    for (idx, n) in nodes.iter().enumerate() { id_index.insert(n.id, idx); }

    #[derive(Debug, Clone)]
    struct Sample { features: Vec<f64>, expected: usize }
    let test_set = vec![
        Sample { features: vec![5.1, 3.5, 1.4, 0.2], expected: 0 },
        Sample { features: vec![4.9, 3.0, 1.4, 0.2], expected: 0 },
        Sample { features: vec![6.0, 2.2, 4.0, 1.0], expected: 1 },
        Sample { features: vec![5.9, 3.0, 5.1, 1.8], expected: 2 },
        Sample { features: vec![6.5, 3.0, 5.2, 2.0], expected: 2 },
    ];

    fn predict(nodes: &[TreeNode], id_index: &HashMap<usize, usize>, x: &[f64]) -> Vec<f64> {
        let mut current_id: usize = 0;
        loop {
            let idx = *id_index.get(&current_id).expect("Tree refers to unknown node id");
            let node = &nodes[idx];
            if node.feature.is_none() {
                return node.value.get(0).cloned().unwrap_or_default();
            }
            let f = node.feature.unwrap();
            let thr = node.threshold.unwrap_or(0.0);
            let xf = x.get(f).copied().unwrap_or(0.0);
            current_id = if xf <= thr {
                node.left.expect("Missing left child id")
            } else {
                node.right.expect("Missing right child id")
            };
        }
    }

    let mut correct = 0usize;
    for (i, s) in test_set.iter().enumerate() {
        let probs = predict(&nodes, &id_index, &s.features);
        let (pred_class, pred_prob) = probs
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(idx, p)| (idx, *p))
            .unwrap_or((usize::MAX, f64::NAN));
        let ok = pred_class == s.expected;
        if ok { correct += 1; }
        println!(
            "#{} x={:?} => class={} (p={:.3}) | expected={}{}",
            i + 1,
            s.features,
            pred_class,
            pred_prob,
            s.expected,
            if ok { " ✅" } else { " ❌" }
        );
    }
    println!("Accuracy: {}/{} = {:.1}%", correct, test_set.len(), (correct as f64) * 100.0 / (test_set.len() as f64));
}
