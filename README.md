## Word chain game

Solution to word chain [game](https://github.com/birchmd/word-chain-game/tree/main) in Rust.

Each letter in the word is masked with a `*` to create a pattern that can group words that are one letter away from each other to make BFS search easier.

### How to run

```bash
cargo run -- <start_word> <end_word> <word_list_file>
```
