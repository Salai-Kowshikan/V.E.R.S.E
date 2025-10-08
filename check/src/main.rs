use std::fs;
use std::mem;
use std::convert::TryInto;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct Node {
    feature_index: u32,
    threshold: f32,
    left: i32,
    right: i32,
    class_label: i32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_bytes = fs::read("iris_tree_nodes.bin")?;
    println!("File size: {} bytes", file_bytes.len());

    let node_size = mem::size_of::<Node>(); // 20 bytes
    let mut nodes = Vec::new();

    for chunk in file_bytes.chunks_exact(node_size) {
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

    // Print nodes
    for (i, node) in nodes.iter().enumerate() {
        println!(
            "Node {}: feature_index={}, threshold={}, left={}, right={}, class_label={}",
            i, node.feature_index, node.threshold, node.left, node.right, node.class_label
        );
    }

    Ok(())
}
