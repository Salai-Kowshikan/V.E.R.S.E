// use risc0_zkvm::guest::env;

// fn main() {
//     // Linear regression model: y = x*a + b
    
//     // Read the coefficients from host
//     let a: f32 = env::read();  // slope
//     let b: f32 = env::read();  // intercept
    
//     // Define x value inside the guest (this remains private)
//     let x: f32 = 5.0;
    
//     // Compute linear regression: y = x*a + b
//     let y: f32 = x * a + b;
    
//     // Commit the result to the journal (this becomes public)
//     env::commit(&y);
// }


#![no_main]
#![no_main]
use risc0_zkvm::guest::env;

fn linear_regression(x: f32, a: f32, b: f32) -> f32 {
    x * a + b
}

fn multiple_regression(xs: &[f32], weights: &[f32], b: f32) -> f32 {
    xs.iter().zip(weights.iter()).map(|(x, w)| x * w).sum::<f32>() + b
}

fn polynomial_regression(x: f32, coeffs: &[f32]) -> f32 {
    coeffs
        .iter()
        .enumerate()
        .map(|(i, a)| a * x.powi(i as i32))
        .sum()
}

fn logistic_regression(xs: &[f32], weights: &[f32], b: f32) -> f32 {
    let z: f32 = xs.iter().zip(weights.iter()).map(|(x, w)| x * w).sum::<f32>() + b;
    1.0 / (1.0 + (-z).exp())
}

risc0_zkvm::guest::entry!(main);
fn main() {
    let model_type: u32 = env::read();
    let weights: Vec<f32> = env::read();
    let b: f32 = env::read();

   
    const DATASET: [([f32; 3], f32); 4] = [
        ([1.0, 2.0, 3.0], 14.0),
        ([2.0, 3.0, 4.0], 20.0),
        ([3.0, 4.0, 5.0], 26.0),
        ([4.0, 5.0, 6.0], 32.0),
    ];

    let results: Vec<(f32, f32)> = match model_type {
        1 => {
            assert!(weights.len() == 1, "Linear regression needs 1 weight (a)");
            DATASET
                .iter()
                .map(|(features, y_true)| {
                    let y_pred = linear_regression(features[0], weights[0], b);
                    (y_pred, *y_true)
                })
                .collect()
        }
        2 => {
            let feat = DATASET[0].0.len();
            assert!(weights.len() == feat, "Multiple regression needs {} weights", feat);
            DATASET
                .iter()
                .map(|(features, y_true)| {
                    let y_pred = multiple_regression(features, &weights, b);
                    (y_pred, *y_true)
                })
                .collect()
        }
        3 => {
            assert!(!weights.is_empty(), "Polynomial needs >= 1 coefficient");
            DATASET
                .iter()
                .map(|(features, y_true)| {
                    let y_pred = polynomial_regression(features[0], &weights);
                    (y_pred, *y_true)
                })
                .collect()
        }
        4 => {
            let feat = DATASET[0].0.len();
            assert!(weights.len() == feat, "Logistic regression needs {} weights", feat);
            DATASET
                .iter()
                .map(|(features, y_true)| {
                    let y_pred = logistic_regression(features, &weights, b);
                    (y_pred, *y_true)
                })
                .collect()
        }
        _ => panic!("Unknown model type {}", model_type),
    };

    env::commit(&results);
}
