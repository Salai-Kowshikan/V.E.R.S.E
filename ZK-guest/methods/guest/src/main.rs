
// // // // // #![no_main]
// // // // // #![no_main]
// // // // // use risc0_zkvm::guest::env;

// // // // // fn linear_regression(x: f32, a: f32, b: f32) -> f32 {
// // // // //     x * a + b
// // // // // }

// // // // // fn multiple_regression(xs: &[f32], weights: &[f32], b: f32) -> f32 {
// // // // //     xs.iter().zip(weights.iter()).map(|(x, w)| x * w).sum::<f32>() + b
// // // // // }

// // // // // fn polynomial_regression(x: f32, coeffs: &[f32]) -> f32 {
// // // // //     coeffs
// // // // //         .iter()
// // // // //         .enumerate()
// // // // //         .map(|(i, a)| a * x.powi(i as i32))
// // // // //         .sum()
// // // // // }

// // // // // fn logistic_regression(xs: &[f32], weights: &[f32], b: f32) -> f32 {
// // // // //     let z: f32 = xs.iter().zip(weights.iter()).map(|(x, w)| x * w).sum::<f32>() + b;
// // // // //     1.0 / (1.0 + (-z).exp())
// // // // // }

// // // // // risc0_zkvm::guest::entry!(main);
// // // // // fn main() {
// // // // //     let model_type: u32 = env::read();
// // // // //     let weights: Vec<f32> = env::read();
// // // // //     let b: f32 = env::read();

   
// // // // //     const DATASET: [([f32; 3], f32); 4] = [
// // // // //         ([1.0, 2.0, 3.0], 14.0),
// // // // //         ([2.0, 3.0, 4.0], 20.0),
// // // // //         ([3.0, 4.0, 5.0], 26.0),
// // // // //         ([4.0, 5.0, 6.0], 32.0),
// // // // //     ];

// // // // //     let results: Vec<(f32, f32)> = match model_type {
// // // // //         1 => {
// // // // //             assert!(weights.len() == 1, "Linear regression needs 1 weight (a)");
// // // // //             DATASET
// // // // //                 .iter()
// // // // //                 .map(|(features, y_true)| {
// // // // //                     let y_pred = linear_regression(features[0], weights[0], b);
// // // // //                     (y_pred, *y_true)
// // // // //                 })
// // // // //                 .collect()
// // // // //         }
// // // // //         2 => {
// // // // //             let feat = DATASET[0].0.len();
// // // // //             assert!(weights.len() == feat, "Multiple regression needs {} weights", feat);
// // // // //             DATASET
// // // // //                 .iter()
// // // // //                 .map(|(features, y_true)| {
// // // // //                     let y_pred = multiple_regression(features, &weights, b);
// // // // //                     (y_pred, *y_true)
// // // // //                 })
// // // // //                 .collect()
// // // // //         }
// // // // //         3 => {
// // // // //             assert!(!weights.is_empty(), "Polynomial needs >= 1 coefficient");
// // // // //             DATASET
// // // // //                 .iter()
// // // // //                 .map(|(features, y_true)| {
// // // // //                     let y_pred = polynomial_regression(features[0], &weights);
// // // // //                     (y_pred, *y_true)
// // // // //                 })
// // // // //                 .collect()
// // // // //         }
// // // // //         4 => {
// // // // //             let feat = DATASET[0].0.len();
// // // // //             assert!(weights.len() == feat, "Logistic regression needs {} weights", feat);
// // // // //             DATASET
// // // // //                 .iter()
// // // // //                 .map(|(features, y_true)| {
// // // // //                     let y_pred = logistic_regression(features, &weights, b);
// // // // //                     (y_pred, *y_true)
// // // // //                 })
// // // // //                 .collect()
// // // // //         }
// // // // //         _ => panic!("Unknown model type {}", model_type),
// // // // //     };

// // // // //     env::commit(&results);
// // // // // }


// #![no_main]
// use risc0_zkvm::guest::env;

// // ------------------ Fixed point configuration ------------------
// const SCALE_BITS: i32 = 16;            // 2^16 scaling (65536)
// const SCALE: i64 = 1 << SCALE_BITS;    // 65536

// // Set dataset size here for benchmarking (change manually)
// const DATASET_SIZE: usize = 300;

// // ------------------ Fixed helpers ------------------
// #[inline(always)]
// fn f32_to_fixed(x: f32) -> i64 {
//     ((x as f64) * (SCALE as f64)).round() as i64
// }

// #[inline(always)]
// fn fixed_to_f32(x: i64) -> f32 {
//     (x as f64 / SCALE as f64) as f32
// }

// #[inline(always)]
// fn fixed_mul(a: i64, b: i64) -> i64 {
//     // use i128 transient to keep precision, then shift right
//     let prod = (a as i128) * (b as i128);
//     (prod >> SCALE_BITS) as i64
// }

// // clamp helpers in fixed domain
// #[inline(always)]
// fn clamp_fx(x: i64, lo: i64, hi: i64) -> i64 {
//     if x < lo { lo } else if x > hi { hi } else { x }
// }

// // ------------------ Fixed-model math primitives ------------------
// fn multiple_regression_fixed_accumulate(features_fx: &[i64], weights_fx: &[i64], b_fx: i64) -> i64 {
//     let mut acc: i64 = 0;
//     for (x_fx, w_fx) in features_fx.iter().zip(weights_fx.iter()) {
//         acc += fixed_mul(*x_fx, *w_fx);
//     }
//     acc + b_fx
// }

// // Horner in fixed domain for polynomial evaluation
// fn polynomial_fixed_horner(x_fx: i64, coeffs_fx: &[i64]) -> i64 {
//     let mut acc: i64 = 0;
//     for &c in coeffs_fx.iter().rev() {
//         acc = fixed_mul(acc, x_fx) + c;
//     }
//     acc
// }

// // Cubic sigmoid approximation in floats:
// // sigmoid(z) ≈ 0.5 + a1*z - a3*z^3  with a1=0.1963, a3=0.004375 (works for |z| approx <= ~6-8)
// // We implement in fixed arithmetic: result scaled by SCALE.
// fn sigmoid_fixed_approx(z_fx: i64) -> i64 {
//     // constants in fixed
//     const A1_F: f32 = 0.1963;
//     const A3_F: f32 = 0.004375;
//     let a1_fx = f32_to_fixed(A1_F);
//     let a3_fx = f32_to_fixed(A3_F);
//     let half_fx = f32_to_fixed(0.5);

//     // z_fx is scaled by SCALE
//     // compute z^2 and z^3 with fixed_mul
//     let z2 = fixed_mul(z_fx, z_fx);
//     let z3 = fixed_mul(z2, z_fx);

//     let term1 = fixed_mul(a1_fx, z_fx);
//     let term3 = fixed_mul(a3_fx, z3);

//     let mut y_fx = half_fx + term1 - term3;

//     // clamp between 0 and 1 in fixed domain
//     y_fx = clamp_fx(y_fx, 0, SCALE);
//     y_fx
// }

// // ------------------ Float math (used in float-mode and for fallback) ------------------
// fn linear_regression_f(x: f32, a: f32, b: f32) -> f32 { x * a + b }

// fn multiple_regression_f(xs: &[f32], weights: &[f32], b: f32) -> f32 {
//     xs.iter().zip(weights.iter()).map(|(x, w)| x * w).sum::<f32>() + b
// }

// // Horner in float
// fn polynomial_regression_f(x: f32, coeffs: &[f32]) -> f32 {
//     let mut acc = 0.0_f32;
//     for &c in coeffs.iter().rev() {
//         acc = acc * x + c;
//     }
//     acc
// }

// fn logistic_regression_f(xs: &[f32], weights: &[f32], b: f32) -> f32 {
//     let z = xs.iter().zip(weights.iter()).map(|(x,w)| x * w).sum::<f32>() + b;
//     1.0 / (1.0 + (-z).exp())
// }

// // ------------------ Deterministic RNG ------------------
// // Single LCG used for both modes. For fixed-mode we produce fixed ints directly w/out float ops.
// #[inline(always)]
// fn lcg(state: &mut u64) -> u64 {
//     *state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
//     *state
// }

// // Float RNG in [-10.0,10.0)
// #[inline(always)]
// fn randf_float(state: &mut u64, lo: f32, hi: f32) -> f32 {
//     let u = (lcg(state) >> 32) as u32;
//     (lo as f64 + (hi - lo) as f64 * (u as f64 / u32::MAX as f64)) as f32
// }

// // Fixed RNG: produce a fixed integer in [lo_fx, hi_fx)
// #[inline(always)]
// fn rand_fixed(state: &mut u64, lo_fx: i64, hi_fx: i64) -> i64 {
//     // use top 32 bits for uniform fraction
//     let u = (lcg(state) >> 32) as u64;
//     let range = (hi_fx as i128 - lo_fx as i128) as i128;
//     // scale u [0, 2^32-1] into range using i128 to avoid overflow
//     let v = ((u as i128 * range) >> 32) + lo_fx as i128;
//     v as i64
// }

// // ------------------ Guest entry ------------------
// risc0_zkvm::guest::entry!(main);
// fn main() {
//     // Inputs from host
//     let use_opt_flag: u32 = env::read(); // 0 = float, 1 = fixed
//     let use_opt = use_opt_flag != 0;
//     let model_type: u32 = env::read();   // 1..4
//     let weights: Vec<f32> = env::read();
//     let b: f32 = env::read();

//     // Precompute fixed weights & bias if fixed-mode
//     let (weights_fx, b_fx) = if use_opt {
//         let wf: Vec<i64> = weights.iter().map(|&w| f32_to_fixed(w)).collect();
//         (wf, f32_to_fixed(b))
//     } else {
//         (Vec::new(), 0)
//     };

//     if use_opt {
//         // Fixed-mode: produce integer pipeline only (no floats inside hot path)
//         let mut out_fx: Vec<(i64,i64)> = Vec::with_capacity(DATASET_SIZE);
//         let mut seed: u64 = 0xC0FFEEu64;

//         for _ in 0..DATASET_SIZE {
//             if model_type == 1 {
//                 // Linear: single feature generated in fixed directly
//                 // choose range for x in [-10, 10)
//                 let lo_fx = f32_to_fixed(-10.0);
//                 let hi_fx = f32_to_fixed(10.0);
//                 let x0_fx = rand_fixed(&mut seed, lo_fx, hi_fx);

//                 let y_pred_fx = fixed_mul(weights_fx[0], x0_fx) + b_fx;
//                 // synthetic ground truth uses same formula
//                 let y_true_fx = y_pred_fx;
//                 out_fx.push((y_pred_fx, y_true_fx));
//             } else {
//                 // use 3 features
//                 let lo_fx = f32_to_fixed(-10.0);
//                 let hi_fx = f32_to_fixed(10.0);
//                 let fx0 = rand_fixed(&mut seed, lo_fx, hi_fx);
//                 let fx1 = rand_fixed(&mut seed, lo_fx, hi_fx);
//                 let fx2 = rand_fixed(&mut seed, lo_fx, hi_fx);
//                 let features_fx = [fx0, fx1, fx2];

//                 let y_pred_fx = match model_type {
//                     2 => multiple_regression_fixed_accumulate(&features_fx, &weights_fx, b_fx),
//                     3 => polynomial_fixed_horner(features_fx[0], &weights_fx),
//                     4 => {
//                         // fixed sigmoid approximation: compute linear combination z_fx and apply cubic approx
//                         let z_fx = multiple_regression_fixed_accumulate(&features_fx, &weights_fx, b_fx) - b_fx; // z = w·x
//                         sigmoid_fixed_approx(z_fx)
//                     }
//                     _ => panic!("Unknown model type {}", model_type),
//                 };

//                 let y_true_fx = y_pred_fx;
//                 out_fx.push((y_pred_fx, y_true_fx));
//             }
//         }

//         // convert outputs to f32 and commit
//         let out_float: Vec<(f32,f32)> = out_fx.into_iter()
//             .map(|(p_fx, t_fx)| (fixed_to_f32(p_fx), fixed_to_f32(t_fx)))
//             .collect();
//         env::commit(&out_float);
//     } else {
//         // Float-mode: produce float results
//         let mut out: Vec<(f32,f32)> = Vec::with_capacity(DATASET_SIZE);
//         let mut seed: u64 = 0xC0FFEEu64;

//         for _ in 0..DATASET_SIZE {
//             if model_type == 1 {
//                 let x0 = randf_float(&mut seed, -10.0, 10.0);
//                 let y_pred = linear_regression_f(x0, weights[0], b);
//                 out.push((y_pred, y_pred));
//             } else {
//                 let x0 = randf_float(&mut seed, -10.0, 10.0);
//                 let x1 = randf_float(&mut seed, -10.0, 10.0);
//                 let x2 = randf_float(&mut seed, -10.0, 10.0);
//                 let y_pred = match model_type {
//                     2 => multiple_regression_f(&[x0,x1,x2], &weights, b),
//                     3 => polynomial_regression_f(x0, &weights),
//                     4 => logistic_regression_f(&[x0,x1,x2], &weights, b),
//                     _ => panic!("Unknown model type {}", model_type),
//                 };
//                 out.push((y_pred, y_pred));
//             }
//         }
//         env::commit(&out);
//     }
// }


#![no_main]
use risc0_zkvm::guest::env;
use std::fs::File;
use std::env as std_env;
use std::io::{BufRead, BufReader};

// ------------------ Fixed point configuration ------------------
const SCALE_BITS: i32 = 16;            // 2^16 scaling
const SCALE: i64 = 1 << SCALE_BITS;    // 65536

// ------------------ Fixed helpers ------------------
#[inline(always)]
fn f32_to_fixed(x: f32) -> i64 {
    ((x as f64) * (SCALE as f64)).round() as i64
}

#[inline(always)]
fn fixed_to_f32(x: i64) -> f32 {
    (x as f64 / SCALE as f64) as f32
}

#[inline(always)]
fn fixed_mul(a: i64, b: i64) -> i64 {
    // use i128 transient to keep precision, then shift right
    let prod = (a as i128) * (b as i128);
    (prod >> SCALE_BITS) as i64
}

#[inline(always)]
fn clamp_fx(x: i64, lo: i64, hi: i64) -> i64 {
    if x < lo { lo } else if x > hi { hi } else { x }
}

// ------------------ Fixed-model math primitives ------------------
fn multiple_regression_fixed_accumulate(features_fx: &[i64], weights_fx: &[i64], b_fx: i64) -> i64 {
    let mut acc: i64 = 0;
    for (x_fx, w_fx) in features_fx.iter().zip(weights_fx.iter()) {
        acc += fixed_mul(*x_fx, *w_fx);
    }
    acc + b_fx
}

// Horner in fixed domain for polynomial evaluation
fn polynomial_fixed_horner(x_fx: i64, coeffs_fx: &[i64]) -> i64 {
    let mut acc: i64 = 0;
    for &c in coeffs_fx.iter().rev() {
        acc = fixed_mul(acc, x_fx) + c;
    }
    acc
}

// Cubic sigmoid approximation in fixed:
// sigmoid(z) ≈ 0.5 + a1*z - a3*z^3  with a1=0.1963, a3=0.004375
fn sigmoid_fixed_approx(z_fx: i64) -> i64 {
    const A1_F: f32 = 0.1963;
    const A3_F: f32 = 0.004375;
    let a1_fx = f32_to_fixed(A1_F);
    let a3_fx = f32_to_fixed(A3_F);
    let half_fx = f32_to_fixed(0.5);

    let z2 = fixed_mul(z_fx, z_fx);
    let z3 = fixed_mul(z2, z_fx);

    let term1 = fixed_mul(a1_fx, z_fx);
    let term3 = fixed_mul(a3_fx, z3);

    let mut y_fx = half_fx + term1 - term3;
    y_fx = clamp_fx(y_fx, 0, SCALE); // clamp between 0 and 1
    y_fx
}

// ------------------ Float math (used in float-mode and for fallback) ------------------
fn linear_regression_f(x: f32, a: f32, b: f32) -> f32 { x * a + b }

fn multiple_regression_f(xs: &[f32], weights: &[f32], b: f32) -> f32 {
    xs.iter().zip(weights.iter()).map(|(x, w)| x * w).sum::<f32>() + b
}

// Horner in float for polynomial (coeff[0] + coeff[1]*x + coeff[2]*x^2 ...)
fn polynomial_regression_f(x: f32, coeffs: &[f32]) -> f32 {
    let mut acc = 0.0_f32;
    for &c in coeffs.iter().rev() {
        acc = acc * x + c;
    }
    acc
}

fn logistic_regression_f(xs: &[f32], weights: &[f32], b: f32) -> f32 {
    let z = xs.iter().zip(weights.iter()).map(|(x,w)| x * w).sum::<f32>() + b;
    1.0 / (1.0 + (-z).exp())
}

// ------------------ CSV Loader ------------------
// Expects last column to be y (label). Skips empty lines; allows optional header.
fn load_csv_dataset(path: &str) -> Vec<(Vec<f32>, f32)> {
    let file = File::open(path).expect("CSV file not found at given path");
    let reader = BufReader::new(file);

    let mut dataset: Vec<(Vec<f32>, f32)> = Vec::new();
    for (i, line_res) in reader.lines().enumerate() {
        let line = match line_res {
            Ok(l) => l.trim().to_string(),
            Err(_) => continue,
        };
        if line.is_empty() { continue; }
        // try to skip header if first line contains non-numeric values
        if i == 0 {
            let tokens: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
            let mut maybe_header = false;
            for t in &tokens {
                if t.is_empty() { maybe_header = true; break; }
                if t.parse::<f32>().is_err() {
                    maybe_header = true;
                    break;
                }
            }
            if maybe_header { continue; }
        }

        let parts: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
        if parts.len() < 2 { continue; } // need at least one feature + label

        // last column is label
        let y_str = parts[parts.len() - 1];
        let y = y_str.parse::<f32>().expect("Failed to parse label as f32");

        // all preceding are features
        let mut features: Vec<f32> = Vec::with_capacity(parts.len() - 1);
        for p in &parts[0..parts.len()-1] {
            features.push(p.parse::<f32>().expect("Failed to parse feature as f32"));
        }

        dataset.push((features, y));
    }

    dataset
}

// ------------------ Guest entry ------------------
risc0_zkvm::guest::entry!(main);

fn main() {
    // Host-provided parameters (same as before)
    let use_opt_flag: u32 = env::read(); // 0 = float, 1 = fixed
    let use_opt = use_opt_flag != 0;
    let model_type: u32 = env::read();   // 1..4
    let weights: Vec<f32> = env::read();
    let b: f32 = env::read();

    
    // const CSV_PATH: &str = "/home/revathi/V.E.R.S.E/dataset.csv";

    let args: Vec<String> = std_env::args().collect();
    if args.len() < 2 {
        panic!("❌ CSV path not provided. Usage: cargo run --release -- <dataset.csv>");
    }
    let csv_path = &args[1];
    println!("📂 Loading dataset from {csv_path}...");
    let dataset = load_csv_dataset(csv_path); 
    //let dataset = load_csv_dataset(CSV_PATH);

    
    assert!(!dataset.is_empty(), "Dataset loaded is empty");

    if use_opt {
       
        let weights_fx: Vec<i64> = weights.iter().map(|&w| f32_to_fixed(w)).collect();
        let b_fx = f32_to_fixed(b);

        let mut out_fx: Vec<(i64, i64)> = Vec::with_capacity(dataset.len());

        for (features, y_true_f) in dataset.iter() {
            // convert features to fixed
            let features_fx: Vec<i64> = features.iter().map(|&x| f32_to_fixed(x)).collect();

            // compute predicted value in fixed domain
            let y_pred_fx = match model_type {
                1 => {
                    // linear: uses first feature & weights[0]
                    assert!(weights_fx.len() >= 1, "Linear model requires 1 weight");
                    fixed_mul(weights_fx[0], features_fx[0]) + b_fx
                }
                2 => {
                    // multiple regression: requires weights.len() == features.len()
                    assert!(weights_fx.len() == features_fx.len(), "Multiple regression: weights length must match feature length");
                    multiple_regression_fixed_accumulate(&features_fx, &weights_fx, b_fx)
                }
                3 => {
                    // polynomial: use first feature as x, coeffs = weights_fx
                    polynomial_fixed_horner(features_fx[0], &weights_fx)
                }
                4 => {
                    // logistic: z = w·x + b, then sigmoid approx
                    assert!(weights_fx.len() == features_fx.len(), "Logistic regression: weights length must match feature length");
                    let z_fx = multiple_regression_fixed_accumulate(&features_fx, &weights_fx, b_fx) - b_fx;
                    sigmoid_fixed_approx(z_fx)
                }
                _ => panic!("Unknown model type {}", model_type),
            };

            let y_true_fx = f32_to_fixed(*y_true_f);
            out_fx.push((y_pred_fx, y_true_fx));
        }

        // convert outputs to f32 and commit
        let out_float: Vec<(f32, f32)> = out_fx.into_iter()
            .map(|(p_fx, t_fx)| (fixed_to_f32(p_fx), fixed_to_f32(t_fx)))
            .collect();
        env::commit(&out_float);
    } else {
        // Float-mode: produce float results
        let mut out: Vec<(f32, f32)> = Vec::with_capacity(dataset.len());

        for (features, y_true) in dataset.iter() {
            let y_pred = match model_type {
                1 => {
                    assert!(weights.len() >= 1, "Linear model requires 1 weight");
                    linear_regression_f(features[0], weights[0], b)
                }
                2 => {
                    assert!(weights.len() == features.len(), "Multiple regression: weights length must match feature length");
                    multiple_regression_f(&features, &weights, b)
                }
                3 => {
                    assert!(!weights.is_empty(), "Polynomial needs >= 1 coefficient");
                    polynomial_regression_f(features[0], &weights)
                }
                4 => {
                    assert!(weights.len() == features.len(), "Logistic regression: weights length must match feature length");
                    logistic_regression_f(&features, &weights, b)
                }
                _ => panic!("Unknown model type {}", model_type),
            };
            out.push((y_pred, *y_true));
        }

        env::commit(&out);
    }
}
