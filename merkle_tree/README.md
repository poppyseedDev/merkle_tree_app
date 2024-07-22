# Merkle Tree Implementation with Proofs and Multiproofs

This library contains an implementation of a Merkle tree in Rust that supports both single proofs and compact multiproofs. The implementation allows for efficient and secure verification of data integrity within a tree structure.

This implementation demonstrates how to create a Merkle tree, generate proofs for individual data blocks, and validate those proofs. It also includes a compact multiproof mechanism for proving the inclusion of multiple data blocks efficiently. This structure is useful for verifying data integrity in various applications, such as blockchain and file verification systems.


## Overview

A Merkle tree is a binary tree in which each leaf node contains a hash of a data block, and each non-leaf node contains a hash of its children. This structure enables efficient and secure verification of large data sets. The root hash, also known as the Merkle root, uniquely represents the data contained in the tree.

### Features

- **Merkle Root Calculation**: Calculate the Merkle root of a given set of data blocks.
- **Proof Generation**: Generate proofs for individual data blocks to verify their presence in the tree.
- **Proof Validation**: Validate proofs against the Merkle root.
- **Compact Multiproofs**: Generate and validate compact multiproofs for multiple data blocks.

## Implementation Details

### Hashing

The implementation uses Rust's built-in `DefaultHasher`, which returns a `u64` hash value. The `hash` function is a helper function to make the hashing interface easier to understand.

### Padding

To ensure that the number of leaf nodes is a power of two, the `pad_base_layer` function pads the input data blocks with empty strings.

### Concatenation of Hashes

The `concatenate_hash_values` function combines two hash values by hex-encoding them, concatenating the strings, and then hashing the result.

### Merkle Root Calculation

The `calculate_merkle_root_rec` function calculates the Merkle root recursively by hashing pairs of nodes until a single root is obtained. The `calculate_merkle_root` function splits a sentence into words, hashes each word, pads the base layer, and then calculates the Merkle root recursively.

### Proofs and Multiproofs

- **Merkle Proof**: The `generate_proof` function generates a proof for a specific word in a sentence, and the `validate_proof` function validates the proof against the Merkle root.
- **Compact Merkle Multiproof**: The `generate_compact_multiproof` function generates a compact multiproof for multiple words in a sentence, and the `validate_compact_multiproof` function validates the multiproof against the Merkle root.

## Example Usage

### Calculating the Merkle Root

```rust
let sentence = "You trust me, right?";
let merkle_root = calculate_merkle_root(sentence);
println!("Merkle Root: {}", merkle_root);
```

### Generating and Validating a Proof

```rust
let sentence = "You trust me, right?";
let (root, proof) = generate_proof(sentence, 1);
println!("Merkle Root: {}", root);
println!("Proof: {:?}", proof);

let word = "trust";
let is_valid = validate_proof(&root, word, proof);
println!("Is the proof valid? {}", is_valid);
```

### Generating and Validating a Compact Multiproof

```rust
let sentence = "Here's an eight word sentence, special for you.";
let indices = vec![0, 1, 6];
let (root, compact_proof) = generate_compact_multiproof(sentence, indices);
println!("Merkle Root: {}", root);
println!("Compact Multiproof: {:?}", compact_proof);

let words = vec!["Here's", "an", "for"];
let is_valid = validate_compact_multiproof(&root, words, compact_proof);
println!("Is the compact multiproof valid? {}", is_valid);
```

## Testing

This implementation includes unit tests to verify the correctness of the Merkle tree construction, proof generation, and proof validation.

To run the tests, use the following command:

```sh
cargo test
```

### Example Tests

#### Test Merkle Root Calculation

```rust
#[test]
fn calculate_merkle_root_sanity_check() {
    let sentence = "You trust me, right?";
    assert_eq!(4373588283528574023, calculate_merkle_root(sentence));
}
```

#### Test Proof Generation

```rust
#[test]
fn proof_generation_sanity_check() {
    let sentence = "You trust me, right?";
    let expected = (
        4373588283528574023,
        vec![
            SiblingNode::Left(4099928055547683737),
            SiblingNode::Right(2769272874327709143),
        ],
    );
    assert_eq!(expected, generate_proof(sentence, 1));
}
```

#### Test Proof Validation

```rust
#[test]
fn validate_proof_sanity_check() {
    let word = "trust";
    let root = 4373588283528574023;
    let proof = vec![
        SiblingNode::Left(4099928055547683737),
        SiblingNode::Right(2769272874327709143),
    ];
    assert!(validate_proof(&root, word, proof));
}
```