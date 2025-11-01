use ort::{session::{Session, builder::GraphOptimizationLevel}, value::{Tensor, DynValue, MapValueType}};
use std::collections::HashMap;
fn main() -> anyhow::Result<()> {
    println!("Hello, world!");

    let mut model = Session::builder()?
    .with_optimization_level(GraphOptimizationLevel::Level3)?
    .with_intra_threads(4)?
    .commit_from_file("iris_tree_model.onnx")?;

    let input_data: Vec<f32> = vec![5.1, 3.5, 1.4, 0.2];
    let shape = [1usize, 4usize];

    let input_tensor = Tensor::from_array((shape, input_data.into_boxed_slice()))?;
    let mut outputs = model.run(ort::inputs!["float_input" => input_tensor])?;
    let label_value: DynValue = outputs
        .remove("output_label")
        .expect("missing output_label");
    let prob_value: DynValue = outputs
        .remove("output_probability")
        .expect("missing output_probability");

    drop(outputs);

    println!("Predicted label: {:?}", label_value.try_extract_array::<i64>()?);

    let allocator = model.allocator();
    let prob_sequence = prob_value.try_extract_sequence::<MapValueType<i64, f32>>(allocator)?;
    println!("Predicted probabilities: {:?}", prob_sequence);

    for (i, map_val) in prob_sequence.iter().enumerate() {
    let prob_map: HashMap<i64, f32> = map_val.try_extract_map::<i64, f32>()?;
    println!("--- Probability map {} ---", i + 1);

    for (class, prob) in &prob_map {
        println!("Class {} → Probability {:.3}", class, prob);
    }

    if let Some((best_class, best_prob)) = prob_map.iter().max_by(|a, b| a.1.partial_cmp(b.1).unwrap()) {
        println!("Most likely class: {}, Probability: {:.3}", best_class, best_prob);
    }
}

    // for (i, map_val) in prob_sequence.iter().enumerate() {
    //     let prob_map = map_val.try_extract_map::<i64, f32>(allocator)?;
    //     println!("--- Probability map {} ---", i + 1);

    //     for (class, prob) in prob_map.iter() {
    //         println!("Class {} → Probability {:.3}", class, prob);
    //     }

    //     if let Some((best_class, best_prob)) =
    //         prob_map.iter().max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
    //     {
    //         println!(
    //             "Most likely class: {}, Probability: {:.3}",
    //             best_class, best_prob
    //         );
    //     }
    // }

    Ok(())
}