use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use risc0_zkvm::{sha::Digest, Receipt};

// Combined data structure for deserialization
#[derive(Debug, Deserialize, Serialize)]
struct ProofData {
    receipt: Receipt,
    image_id: Digest,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Specify the directory containing the JSON files
    let directory_path = "./proofs";

    // Iterate over all files in the directory
    for entry in fs::read_dir(directory_path)? {
        if let Ok(entry) = entry {
            let path = entry.path();
            
            // Check if the file has a ".json" extension
            if let Some(extension) = path.extension() {
                if extension == "json" {
                    // Read the file content
                    let content = fs::read_to_string(&path)?;

                    // Deserialize the JSON content into ProofData
                    let proof_data: ProofData = serde_json::from_str(&content)?;

                    // Access receipt and image_id for your logic
                    let receipt = proof_data.receipt;
                    let image_id = proof_data.image_id;

                    // Implement your logic here based on receipt and image_id
                    println!("File: {:?}", path);
                    println!("Receipt: {:?}", receipt);
                    println!("Image ID: {:?}", image_id);
                }
            }
        }
    }

    Ok(())
}
