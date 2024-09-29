extern crate sha2;
extern crate num_bigint;
extern crate num_traits;
extern crate rand;

use sha2::{Sha256, Digest};
use num_bigint::{BigUint, RandBigInt};
use num_traits::One;
use std::str;

// Function to hash a string using SHA-256
fn sha256_hash(input: &str) -> BigUint {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    
    // Convert hash result to BigUint
    BigUint::from_bytes_be(&result)
}

// Pedersen hash function
fn pedersen_hash(x: &[BigUint], g: &[BigUint], p: &BigUint) -> BigUint {
    let mut result = BigUint::one();
    
    for (xi, gi) in x.iter().zip(g.iter()) {
        // Compute (g_i^x_i) % p
        let temp = gi.modpow(xi, p);
        result = (result * temp) % p;
    }
    
    result
}

// Function to generate random group generators
fn generate_generators(n: usize, p: &BigUint) -> Vec<BigUint> {
    let mut rng = rand::thread_rng();
    let mut generators = Vec::with_capacity(n);
    
    for _ in 0..n {
        // Generate random generator g_i in Z_p
        generators.push(rng.gen_biguint_range(&BigUint::one(), p));
    }
    
    generators
}

fn main() {
    // Define prime modulus p for the group Z_p
    let p = BigUint::parse_bytes(b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F", 16).unwrap();

    // Generate some random group generators g_1, g_2, ..., g_n
    let generators = generate_generators(3, &p);  // 3 generators

    // Example input string
    let input = "Hello, Pedersen Hash!";

    // Step 1: Hash the input string using SHA-256
    let hashed_value = sha256_hash(input);

    // Step 2: Split hashed value into smaller chunks (as BigUint values)
    // Let's split into 3 chunks for the Pedersen hash
    let chunk_size = 256 / 3;  // Assume 3 chunks, 256-bit hash
    let mut chunks = Vec::new();
    let mut value = hashed_value.clone();
    for _ in 0..3 {
        let chunk: BigUint = &value & ((BigUint::one() << chunk_size) - BigUint::one());  // Take chunk_size bits
        chunks.push(chunk.clone());
        value = value >> chunk_size;  // Shift value for the next chunk
    }

    // Step 3: Apply Pedersen hash on the tuple of chunks
    let pedersen_hashed_value = pedersen_hash(&chunks, &generators, &p);

    // Output result
    println!("Pedersen Hashed Value: {}", pedersen_hashed_value);
}
