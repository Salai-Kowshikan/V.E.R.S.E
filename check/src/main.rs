// use std::fs;
// use std::mem;
// use std::convert::TryInto;

// #[repr(C)]
// #[derive(Debug, Clone, Copy)]
// struct Node {
//     feature_index: u32,
//     threshold: f32,
//     left: i32,
//     right: i32,
//     class_label: i32,
// }

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let file_bytes = fs::read("iris_tree_nodes.bin")?;
//     println!("File size: {} bytes", file_bytes.len());

//     let node_size = mem::size_of::<Node>(); // 20 bytes
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

//     // Print nodes
//     for (i, node) in nodes.iter().enumerate() {
//         println!(
//             "Node {}: feature_index={}, threshold={}, left={}, right={}, class_label={}",
//             i, node.feature_index, node.threshold, node.left, node.right, node.class_label
//         );
//     }

//     Ok(())
// }


// #![forbid(unsafe_code)]

// use onnxruntime::{
//     environment::Environment,
//     ndarray::array,
//     tensor::OrtOwnedTensor,
//     GraphOptimizationLevel, LoggingLevel,
// };
// use std::sync::Arc;
// use tracing::Level;
// use tracing_subscriber::FmtSubscriber;

// type Error = Box<dyn std::error::Error>;

// fn main() {
//     if let Err(e) = run() {
//         eprintln!("Error: {}", e);
//         std::process::exit(1);
//     }
// }

// fn run() -> Result<(), Error> {
//     use std::path::Path;

//     // Setup logging
//     let subscriber = FmtSubscriber::builder()
//         .with_max_level(Level::TRACE)
//         .finish();
//     tracing::subscriber::set_global_default(subscriber)
//         .expect("setting default subscriber failed");

//     let model_path = "iris_tree_copy11.onnx";

//     if !Path::new(model_path).exists() {
//         return Err(format!("ONNX model file not found: {}", model_path).into());
//     }
//     println!("Using model file: {}", model_path);

//     // Create shared environment
//     let environment = Arc::new(
//         Environment::builder()
//             .with_name("iris-env")
//             .with_log_level(LoggingLevel::Warning)
//             .build()?,
//     );

//     // Create session
//     let mut session = environment
//         .as_ref()
//         .new_session_builder()?
//         .with_optimization_level(GraphOptimizationLevel::Basic)?
//         .with_number_threads(1)?
//         .with_model_from_file(model_path)?;

//     // Input: 1 sample, 4 features
//     let input = array![[5.1_f32, 3.5, 1.4, 0.2]];

//     // Run inference (Vec input)
//     let outputs: Vec<OrtOwnedTensor<f32, _>> = session.run(vec![input])?;
//     let predictions = &outputs[0];

//     println!("Raw output: {:?}", predictions.as_slice().unwrap());

//     // Map to class names
//     let classes = ["setosa", "versicolor", "virginica"];
//     let predicted_index = predictions.as_slice().unwrap()[0] as usize;
//     println!("Predicted class: {}", classes[predicted_index]);

//     Ok(())
// }


#![forbid(unsafe_code)]

use ort::{environment::Environment, session::SessionBuilder, tensor::OrtOwnedTensor};
use ndarray::array;
use std::sync::Arc;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use std::path::Path;

type Error = Box<dyn std::error::Error>;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Error> {
    // Setup logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    let model_path = "iris_tree_copy.onnx";
    if !Path::new(model_path).exists() {
        return Err(format!("ONNX model file not found: {}", model_path).into());
    }
    println!("Using model file: {}", model_path);

    // Create shared environment
    let environment = Arc::new(Environment::builder()
        .with_name("iris-env")
        .with_log_level(ort::LoggingLevel::Warning)
        .build()?);

    // Build session
    let session = SessionBuilder::new(&environment)?
        .with_optimization_level(ort::GraphOptimizationLevel::Basic)?
        .with_number_threads(1)?
        .with_model_from_file(model_path)?;

    // Input: 1 sample, 4 features
    let input = array![[5.1_f32, 3.5, 1.4, 0.2]];

    // Run inference
    let outputs: Vec<OrtOwnedTensor<f32, _>> = session.run(vec![input.into()])?;
    let predictions = &outputs[0];

    println!("Raw output: {:?}", predictions.as_slice()?);

    // Map to class names
    let classes = ["setosa", "versicolor", "virginica"];
    let predicted_index = predictions.as_slice()?[0] as usize;
    println!("Predicted class: {}", classes[predicted_index]);

    Ok(())
}
