# GEMINI.md - ofcp_data

## Project Overview
`ofcp_data` is a high-performance Rust library 
providing pre-computed data for Open Face Chinese Poker (OFCP). 
It is primarily a data crate containing lookup tables 
for hand rankings, placement strategies, and card combinations.

- **Main Technologies:** Rust (Edition 2024).
- **Domain:** Poker hand evaluation and game theory data for Chinese Poker.

## Architecture
The project is designed for maximum lookup efficiency:
- **Data Modules:** Numerous modules (e.g., `src/rank11.rs`, `src/rank55.rs`) contain large static arrays of pre-computed ranks.
- **Indexed Arrays (`Ia`):** A custom data structure used to manage slices of values with an index lookup, 
  reducing the need for complex nested structures.
- **Ranks Struct:** Encapsulates ranking data and implements Eytzinger binary search 
  for O(log N) lookup with better cache locality.
- **Unchecked Access:** Most lookup functions provide a `*_unchecked` variant 
  for performance-critical code where the caller guarantees bounds safety.

## Building and Running
As a standard Rust library, the following commands apply:

- **Build:** `cargo build`
- **Test:** `cargo test` (Note: Currently, the project consists mainly of data definitions).
- **Documentation:** `cargo doc --open`

## Development Conventions
- **Code Generation:** Many source files in `src/` are likely generated programmatically. 
  Exercise caution when editing them manually.
- **Performance First:** Preference is given to `inline(always)` functions and static data to minimize runtime overhead.
- **Naming Convention:** Functions are named based on the round and/or specific ranking data they provide 
  (e.g., `rank53_11`, `ROUND5_PLACE3`).
- **Safety:** `unsafe` code is used selectively for unchecked array indexing to optimize performance. 
  Ensure that any manual calls to `*_unchecked` functions strictly adhere to the documented safety requirements.
