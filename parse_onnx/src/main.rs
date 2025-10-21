use tract_onnx::prelude::*;
use serde_json::json;
use std::fs;

fn main() -> TractResult<()> {
    // Load ONNX model
    let model = tract_onnx::onnx()
        .model_for_path("iris_tree_model.onnx")?
        .into_optimized()?
        .into_runnable()?;

    // Inspect and extract layers
    let layers: Vec<_> = model.model.nodes().iter().map(|n| {
        json!({
            "name": n.name,
            "op": format!("{:?}", n.op),
            "inputs": n.inputs.iter().map(|i| format!("{:?}", i)).collect::<Vec<_>>(),
        })
    }).collect();

    // Write to JSON file
    fs::write("model_structure.json", serde_json::to_string_pretty(&layers)?)?;
    println!("✅ Model structure saved to model_structure.json");
    Ok(())
}
