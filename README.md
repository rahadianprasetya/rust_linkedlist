# Huffman Encoding Algorithm in Rust

This repository contains a complete implementation of the Huffman Encoding algorithm written in Rust. It covers everything from character frequency counting and tree building to character/string encoding.



## Table of Contents
- [Overview](#overview)
- [Data Structures](#data-structures)
- [How It Works](#how-it-works)
- [Implementation Details](#implementation-details)
- [The Challenge](#the-challenge)

---

## Overview

Huffman coding is a popular algorithm used for lossless data compression. It assigns variable-length binary codes to input characters, lengths of the assigned codes are based on the frequencies of corresponding characters. The most frequent character gets the smallest code and the least frequent character gets the largest code.

---

## Data Structures

### 1. The Tree Node (`HuffNode`)
Unlike standard binary trees where data can exist at any level, a Huffman tree only stores data at its leaves (`Leaf`). Branch nodes (`Tree`) only serve to split the path and contain no data.

```rust
pub enum HuffNode {
    Leaf(char),
    Tree(Box<HuffNode>, Box<HuffNode>),
}
