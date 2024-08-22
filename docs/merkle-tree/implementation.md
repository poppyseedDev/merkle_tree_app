# Implementing Merkle Trees in Rust 💻

In this section, we'll explore the practical implementation of Merkle trees in Rust using visual intuition. We’ll cover:

1. **Calculating the Merkle Root Recursively** 🌳
2. **Generating a Proof** 🔍
3. **Validating a Proof** ✅
4. **Working with Compact Multiproofs** 📦

## 1. Calculating the Merkle Root Recursively 🌳

The Merkle root is the top-most node of a Merkle tree, representing the combined hash of all the data blocks. Let's break down how you can calculate it recursively.

### Recursive Merkle Root Calculation

**Visual Steps:**

- **Step 1**: Start with your data blocks and hash each one to create the leaf nodes.
  
  ```text
  Data Blocks: [D1, D2, D3, '']  ← Pad the base layer if the number of data is not N**2
  Leaf Nodes:  [H1, H2, H3, H4]  ← Hash each data block
  ```

- **Step 2**: Combine each pair of hashes to create the next level of the tree.
  
  ```text
      Level 1:  [H1, H2, H3, H4]
      Combine:  H1 + H2 = A,  H3 + H4 = B
      Hash the combination: [Hash(A), Hash(B)]  ← List it to the parent level hashes
  ```

- **Step 3**: Recursively repeat this process until you have a single hash, the Merkle root.

  ```text
  Final Level:
        Root
         |
         A + B = Root Hash
  ```

### Visual Example

Here's what the recursive process looks like:

```text
  Level 1:   H1    H2    H3    H4    ← Hashes of data blocks
              \   /      \   /
  Level 2:   Hash(A)     Hash(B)           ← Combine hashes to form next level
                 \     /
  Root Level:     Root Hash          ← Combine recursively to get the final Merkle root
```

## 2. Generating a Proof 🔍

A Merkle proof allows you to verify that a specific data block is part of a Merkle tree. Let’s visually explore how to generate this proof.

### Generating a Proof

**Visual Steps:**

- **Step 1**: Identify the path from the target leaf node (your data block) to the root.
  
  - Example: To prove that `H2` is part of the tree:
  
  ```text
  Path: H2 → A → Root
  ```

- **Step 2**: Collect the hashes of the sibling nodes along this path. For each node, check the position of its sibling (either left or right).

  - Example:
  
  ```text
  Siblings Needed: H1 (sibling of H2), B (sibling of A)
  ```

- **Step 3**: Add the sibling hashes to a proof list, marking their positions (Left or Right).

  - Example:
  
  ```text
  Proof = [Left: H1, Right: B]
  ```

### Visual Example

Here's a diagram to illustrate generating a proof:

```text
        Root Hash
           /  \
         A      B
        / \    / \
      H1  H2  H3  H4

 Proof = [Left: H1, Right: B]

  Proof for H2:
  1. Start at H2.
  2. Collect sibling H1 (Left).
  3. Move to parent A and collect its sibling B (Right).
  4. Reach the root.

  Final Proof Path: [H1, B]
```



## 3. Validating a Proof ✅

After generating a proof, you can validate that a data block is indeed part of the Merkle tree. Here’s how this validation works visually.

### Validating a Proof

**Visual Steps:**

**Inputs:** 
 - Original Root Hash
 - Data Block Hash: `H2`
 - Proof: `[Left: H1, Right: B]`
 
```text

        Root Hash
           /  \
         A      B
        / \    / \
      H1  H2  H3  H4
```

- **Step 1**: Start with the hash of the data block you’re validating.
  
  - Example:
  
  ```text
  Start with: H2
  ```

- **Step 2**: Sequentially combine the data block hash with the sibling hashes from the proof, following the order used to create the tree.
  
  - Example:
  
  ```text
  Combine H2 with Left: H1 → A
  Combine A with Right: B → Root
  ```

- **Step 3**: Compare the resulting root hash with the original Merkle root.

  - If they match, the proof is valid, confirming that `H2` is part of the tree.

### Visual Example

```text
 Proof = [Left: H1, Right: B]

        Root Hash
           /  \
         A      B
        / \    / \
      H1  H2  H3  H4

  Validating H2:
  1. Start with H2.
  2. Combine H2 with H1 (Left) to get A.
  3. Add the hash A to the proof list.
  4. Combine A with B (Right) to get the Root.
  5. Compare the calculated Root with the original Root Hash.
  6. If the roots match, H2 is successfully validated as part of the tree.
```

## 4. Working with Compact Multiproofs 📦

Compact multiproofs allow you to efficiently prove the inclusion of multiple data blocks in a Merkle tree. Unlike traditional Merkle proofs, which require padding to make the number of leaves a power of two, compact multiproofs handle arbitrary numbers of leaves without padding. Let’s explore how to generate and validate compact multiproofs.

An example compact merkle tree:
```text
        Root Hash
           /  \
         A      H3
        / \   
      H1  H2  
```

### Generating a Compact Multiproof

**Visual Steps:**

- **Step 1**: Identify the indices of the data blocks you want to prove.

  - Example:
  
  ```text
  We want to prove the inclusion of blocks at indices 0, 1, and 6.
  leaf_indices = [0, 1, 6]
  ```

- **Step 2**: Collect only the necessary sibling hashes required to prove all the selected blocks. This ensures that the proof is as small as possible while still being valid.

  - Example:
  
  ```text
  Sibling hashes needed: H1 (for H2), H4 (for H3)
  ```

### Visual Example

Consider the following Merkle tree:

```text
                                      Root            
                                   /     \           
                                O           O     
                              /   \       /   \     
                             O    H_1   H_2    O  
                            / \   / \   / \   / \
                           X  X  O  O  O  O  X  H_0
```

For proving the blocks at indices 0, 1, and 6:

- **Leaf Nodes**: `[X, X, X]` represent the data blocks at indices 0, 1, and 6.
- **Sibling Nodes**: Collect the necessary siblings like `H_1` and `H_2`.
- **Proof Structure**: Combine the collected siblings to form the compact multiproof.

### Generating a Compact Multiproof

```text
Proof = [H_0, H_1, H_2]
```

In this example, the compact multiproof allows for proving multiple blocks with fewer hashes compared to generating separate proofs for each block.

---

### Validating a Compact Multiproof

**Visual Steps:**

- **Step 1**: Reconstruct the hashes for each subtree from the provided blocks and the proof.

  - Example:
  
  ```text
  Combine H1 + H2 to get A
  Combine H3 + H4 to get B
  ```

- **Step 2**: Compare the resulting root hash with the stored Merkle root to validate the proof.

### Visual Example

Let’s validate a compact multiproof:

```text
        Root Hash
           /  \
         A      B
        / \    / \
      H1  H2  H3  H4
```

For validating blocks H1, H2, and H3:

- **Step 1**: Reconstruct the hash `A` from `H1` and `H2`.
- **Step 2**: Reconstruct the hash `B` from `H3` and `H4`.
- **Step 3**: Combine `A` and `B` to form the root hash.
- **Step 4**: If the root matches the stored root hash, the proof is valid.

