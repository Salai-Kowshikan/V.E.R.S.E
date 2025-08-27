use risc0_zkvm::Receipt;
use serde_json;
use std::fs;

fn main() {
    const METHOD_ID: [u32; 8] =  [1452707642, 2073294129, 1860126072, 2724424061, 2984836555, 809201159, 920081389, 159827681];

    //  Load the proof.json
    let data = fs::read_to_string("proof.json")
        .expect("Failed to read proof.json");

    let receipt: Receipt = serde_json::from_str(&data)
        .expect("Failed to parse receipt");

    //  Verify against method_id
    match receipt.verify(METHOD_ID) {
        Ok(_) => println!("✅ Proof verified successfully!"),
        Err(e) => println!("❌ Verification failed: {:?}", e),
    }
}
