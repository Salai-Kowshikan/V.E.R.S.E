use risc0_zkvm::guest::env;

fn main() {
    // Linear regression model: y = x*a + b
    
    // Read the coefficients from host
    let a: f32 = env::read();  // slope
    let b: f32 = env::read();  // intercept
    
    // Define x value inside the guest (this remains private)
    let x: f32 = 5.0;
    
    // Compute linear regression: y = x*a + b
    let y: f32 = x * a + b;
    
    // Commit the result to the journal (this becomes public)
    env::commit(&y);
}
