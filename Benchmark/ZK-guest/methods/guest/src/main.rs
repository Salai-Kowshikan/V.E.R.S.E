#![no_main]
#![no_main]
use risc0_zkvm::guest::env;

// ----- Fixed-point helpers (Q-scale) -----
const SCALE: i64 = 1_000_000; // 6 decimal places of precision

#[inline]
fn to_fx(x: f32) -> i64 { (x as f64 * SCALE as f64).round() as i64 }

#[inline]
fn from_fx(x: i64) -> f32 { (x as f64 / SCALE as f64) as f32 }

#[inline]
fn mul_fx(a: i64, b: i64) -> i64 { ((a as i128 * b as i128) / SCALE as i128) as i64 }

fn linear_regression_int(x: f32, a: f32, b: f32) -> f32 {
    let x_fx = to_fx(x); let a_fx = to_fx(a); let b_fx = to_fx(b);
    let y_fx = mul_fx(x_fx, a_fx) + b_fx; from_fx(y_fx)
}

fn multiple_regression_int(xs: &[f32], weights: &[f32], b: f32) -> f32 {
    let weights_fx: Vec<i64> = weights.iter().map(|w| to_fx(*w)).collect();
    let b_fx = to_fx(b);
    let mut acc: i128 = 0;
    for (x, w_fx) in xs.iter().zip(weights_fx.iter()) {
        let xi_fx = to_fx(*x) as i128; let wi_fx = *w_fx as i128;
        acc += (xi_fx * wi_fx) / SCALE as i128;
    }
    let y_fx = acc as i64 + b_fx; from_fx(y_fx)
}

// Floating-point (unoptimized) versions for benchmarking
fn linear_regression(x: f32, a: f32, b: f32) -> f32 { x * a + b }
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

// ----- Deterministic PRNG and dataset generation -----
#[inline]
fn lcg(state: &mut u64) -> u64 {
    // 64-bit LCG parameters (Numerical Recipes variant)
    *state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
    *state
}

#[inline]
fn randf(state: &mut u64, lo: f32, hi: f32) -> f32 {
    let x = ((lcg(state) >> 32) as u32) as f64 / (u32::MAX as f64);
    (lo as f64 + (hi - lo) as f64 * x) as f32
}

fn make_dataset(n: usize, weights: &[f32], b: f32, model_type: u32, use_opt: bool) -> Vec<([f32; 3], f32)> {
    let mut seed: u64 = 0xC0FFEEu64;
    let mut data: Vec<([f32; 3], f32)> = Vec::with_capacity(n);
    for _ in 0..n {
        let x0 = randf(&mut seed, -10.0, 10.0);
        let x1 = randf(&mut seed, -10.0, 10.0);
        let x2 = randf(&mut seed, -10.0, 10.0);
        let features = [x0, x1, x2];
        let y_true = match model_type {
            1 => if use_opt { linear_regression_int(features[0], weights[0], b) } else { linear_regression(features[0], weights[0], b) },
            2 => if use_opt { multiple_regression_int(&features, weights, b) } else { multiple_regression(&features, weights, b) },
            3 => polynomial_regression(features[0], &weights),
            4 => logistic_regression(&features, &weights, b),
            _ => 0.0,
        };
        data.push((features, y_true));
    }
    data
}

risc0_zkvm::guest::entry!(main);
fn main() {
    let use_opt_flag: u32 = env::read();
    let use_opt: bool = use_opt_flag != 0;
    let model_type: u32 = env::read();
    let weights: Vec<f32> = env::read();
    let b: f32 = env::read();

    // Build a randomized dataset of size 1000 deterministically
    let dataset = make_dataset(1000, &weights, b, model_type, use_opt);

    let results: Vec<(f32, f32)> = match model_type {
        1 => {
            assert!(weights.len() == 1, "Linear regression needs 1 weight (a)");
            dataset
                .iter()
                .map(|(features, y_true)| {
                    let y_pred = if use_opt { linear_regression_int(features[0], weights[0], b) } else { linear_regression(features[0], weights[0], b) };
                    (y_pred, *y_true)
                })
                .collect()
        }
        2 => {
            let feat = dataset[0].0.len();
            assert!(weights.len() == feat, "Multiple regression needs {} weights", feat);
            dataset
                .iter()
                .map(|(features, y_true)| {
                    let y_pred = if use_opt { multiple_regression_int(features, &weights, b) } else { multiple_regression(features, &weights, b) };
                    (y_pred, *y_true)
                })
                .collect()
        }
        3 => {
            assert!(!weights.is_empty(), "Polynomial needs >= 1 coefficient");
            dataset
                .iter()
                .map(|(features, y_true)| {
                    let y_pred = polynomial_regression(features[0], &weights);
                    (y_pred, *y_true)
                })
                .collect()
        }
        4 => {
            let feat = dataset[0].0.len();
            assert!(weights.len() == feat, "Logistic regression needs {} weights", feat);
            dataset
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
