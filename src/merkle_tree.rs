//! Merkle tree implementation.

use std::{collections::VecDeque, hash::Hasher};

/// A node in the binary tree.
#[derive(Debug)]
struct Node {
    value: [u8; 32],
    left: Subtree,
    right: Subtree,
}

/// A possible empty subtree
#[derive(Debug)]
struct Subtree(Option<Box<Node>>);

/// Merkle tree
#[derive(Debug)]
struct MerkleTree<H: Hasher> {
    nodes: VecDeque<Node<H>>,
}
