// //! Merkle tree implementation.

// use digest::Digest;
// use sha2::Sha256;
// use std::{collections::VecDeque, hash::Hasher};

// /// A node in the binary tree.
// #[derive(Debug)]
// struct Node<V: AsRef<[u8]>> {
//     value: V,
//     left: Option<Box<Node<V>>>,
//     right: Option<Box<Node<V>>>,
// }

// /// Merkle tree
// #[derive(Debug)]
// struct MerkleTree<H: Digest, Output: AsRef<u8>> {
//     nodes: VecDeque<Node<Output>>,
//     _marker: std::marker::PhantomData<H>,
// }

// impl<H: Digest, Output: AsRef<u8>> MerkleTree<H, Output> {
//     fn new() -> Self {
//         Self {
//             nodes: VecDeque::new(),
//             _marker: std::marker::PhantomData,
//         }
//     }

//     fn insert() {}
// }

// #[test]
// mod tests {
//     use super::*;
//     #[test]
//     fn test_basic_setup() {}
// }
