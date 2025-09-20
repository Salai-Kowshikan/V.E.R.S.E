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
use risc0_zkvm::guest::env;

const DATASET: [[f32; 3]; 4] = [
    [1.0, 2.0, 3.0],
    [2.0, 3.0, 4.0],
    [3.0, 4.0, 5.0],
    [4.0, 5.0, 6.0],
];


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

    let results: Vec<f32> = match model_type {
        1 => {
            assert!(weights.len() == 1, "Linear regression needs 1 weight (a)");
            DATASET
                .iter()
                .map(|row| linear_regression(row[0], weights[0], b))
                .collect()
        }
        2 => {
            let feat = DATASET[0].len();
            assert!(
                weights.len() == feat,
                "Multiple regression needs {} weights",
                feat
            );
            DATASET
                .iter()
                .map(|row| multiple_regression(row, &weights, b))
                .collect()
        }
        3 => {
            assert!(!weights.is_empty(), "Polynomial needs >= 1 coefficient");
            DATASET
                .iter()
                .map(|row| polynomial_regression(row[0], &weights))
                .collect()
        }
        4 => {
            let feat = DATASET[0].len();
            assert!(
                weights.len() == feat,
                "Logistic regression needs {} weights",
                feat
            );
            DATASET
                .iter()
                .map(|row| logistic_regression(row, &weights, b))
                .collect()
        }
        _ => panic!("Unknown model type {}", model_type),
    };

    env::commit(&results);
}

//     1 - Linear Regression
//     2 - Multiple Regression
//     3 - Polynomial Regression
//     4 - Logistic Regression
