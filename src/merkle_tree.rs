//! Merkle tree implementation.

use std::collections::VecDeque;

use digest::Digest;
use sha2::Sha256;

/// A node in the binary tree.
#[derive(Debug, Clone)]
struct Node {
    value: [u8; 32],
    left_child: Option<Box<Node>>,
    right_child: Option<Box<Node>>,
}

/// Merkle tree
#[derive(Debug)]
struct MerkleTree {
    root: Option<Node>,
}

impl MerkleTree {
    fn new() -> Self {
        Self { root: None }
    }

    /// Very early, inefficient way of building the merkle tree
    fn build(data: &mut Vec<Vec<u8>>) -> Result<Self, ()> {
        // find the nearest power of 2
        let size = (data.len() as f64).log2() as usize;
        let size = if 1 << size == data.len() {
            1 << size
        } else {
            1 << (size + 1)
        };

        ensure_enough_items(data, size);

        let mut nodes = data
            .iter()
            .map(|v| {
                let mut hasher = Sha256::new();

                hasher.update(v.clone());

                Node {
                    value: hasher.finalize().into(),
                    left_child: None,
                    right_child: None,
                }
            })
            .collect::<VecDeque<_>>();

        let mut cursor = 0;
        while let (Some(l), Some(r)) = (nodes.get(cursor).cloned(), nodes.get(cursor + 1).cloned())
        {
            let hasher = Sha256::new();

            let new_node = generate_node(hasher, l, r);
            nodes.push_back(new_node);
            cursor += 2;
            println!("cursor :{}", cursor);
        }

        // and now, start popping and

        Ok(Self {
            root: nodes.pop_back(),
        })
    }

    fn verify(proof: Vec<[u8; 32]>, data: Vec<u8>) -> bool {
        false
    }
}

fn generate_node<H: Digest>(mut hasher: H, left: Node, right: Node) -> Node {
    let concat_data = [left.value, right.value].concat();
    hasher.update(concat_data);

    Node {
        value: hasher
            .finalize()
            .as_ref()
            .try_into()
            .expect("should be fine"),
        left_child: Some(Box::new(left)),
        right_child: Some(Box::new(right)),
    }
}

fn ensure_enough_items(data: &mut Vec<Vec<u8>>, n: usize) {
    for _ in 0..(data.len() - n) {
        data.push(vec![0u8])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_basic_setup() {
        let mut data_items = vec![vec![0], vec![1], vec![2], vec![3]];

        let hashes = [
            "5feceb66ffc86f38d952786c6d696c79c2dbc239dd4e91b46729d73a27fb57e9",
            "6b86b273ff34fce19d6b804eff5a3f5747ada4eaa22f1d49c01e52ddb7875b4b",
            "d4735e3a265e16eee03f59718b9b5d03019c07d8b6c51f90da3a666eec13ab35",
            "4e07408562bedb8b60ce05c1decfe3ad16b72230967de01f640b7e4729b49fce",
            "fa13bb36c022a6943f37c638126a2c88fc8d008eb5a9fe8fcde17026807feae4",
            "70311d9d203b2d7e4ff70d7fce219f82a4fcf73a110dc80187dfefb7c6e4bb87",
            "862532e6a3c9aafc2016810598ed0cc3025af5640db73224f586b6f1138385f4",
        ];

        let mt = MerkleTree::build(&mut data_items).expect("failed");
    }
}
