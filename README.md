# Huffman coding implemented in Rust

<img src="https://i.imgur.com/NQPKogg.png" width="800">

---

## Known problemes
---
The current implementation searches linearly through the data array. This could be done more efficiently.

Also there are no real self-referencing structs. They hold an `Identifier` property, which is just a `usize`

<img src="https://i.imgur.com/j5LI4V3.png" width="300">
