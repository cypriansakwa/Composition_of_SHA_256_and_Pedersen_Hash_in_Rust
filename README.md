##Overview
This Rust program illustrates a cryptographic approach that combines the SHA-256 hash function and the Pedersen hash function. The purpose is to first hash a string with SHA-256, then partition the output into smaller pieces, and then apply the Pedersen hash function to these pieces using a set of randomly generated group generators.
## Key Concepts:
- **SHA-256 Hashing:** The program hashes an input string into a 256-bit value using the widely used SHA-256 cryptographic hash function.
- **Pedersen Hash:** The Pedersen hash function is applied to a tuple of BigUint values (chunks of the SHA-256 hash) and a set of group generators modulo a large prime. This provides a cryptographically secure way of combining the hashed values.
## Functions:
- `sha256_hash(input: &str) -> BigUint`: This function takes a string input, hashes it using SHA-256, and converts the resulting byte array into a `BigUint` for further processing.
- `pedersen_hash(x: &[BigUint], g: &[BigUint], p: &BigUint) -> BigUint`: This function computes the Pedersen hash of a tuple of BigUint values `(x)` using a set of group generators `(g)` modulo a large prime `(p)`.
- `generate_generators(n: usize, p: &BigUint) -> Vec<BigUint>`: This function generates `n` random group generators within the cyclic group $\mathbb{Z}_p$ , which are used for the Pedersen hash.
## How it works
- **Hashing the Input String:** The input string is hashed using SHA-256 to generate a 256-bit digest. The result is then converted into a `BigUint` for compatibility with the Pedersen hash.
- **Chunking the Hash Value:** The 256-bit SHA-256 hash is split into smaller chunks, represented as `BigUint` values. This is necessary for the Pedersen hash, which operates on tuples of values.
- **Applying Pedersen Hash:** The program generates a set of group generators and computes the Pedersen hash over the previously generated chunks. This ensures the cryptographic security of the output.
## Example
- Consider the input string **``Hello, Pedersen Hash!''**. The program performs the following steps:
   - Hash the input string using SHA-256 to get a 256-bit digest.
   - Split the digest into 3 chunks.
   - Apply the Pedersen hash to the chunks using 3 random group generators.
   - The resulting Pedersen hash is then printed as output.
   >```
   >Pedersen Hashed Value: 75876359842462590399302946399849613651107875193080061420477004440640716125988  
## Contributing
  - If you intend to contribute to this project, fork the repository and make a pull request.

## Usage
- To use this code, you can clone the repository.
- Compile the Rust code using cargo:
>```
>cargo build
>cargo run

## Acknowledgments
- Rust
### Clone the repository or copy the source code into a Rust project.
```bash
   git clone https://github.com/Composition_of_SHA_256_and_Pedersen_Hash_in_Rust.git
   cd Composition_of_SHA_256_and_Pedersen_Hash_in_Rust
