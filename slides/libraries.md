# Libraries

There are libraries that simplify using SIMD and allow you to make it cross-platform:
- [memchr](https://crates.io/crates/memchr/2.7.4) - allows for fast searching of characters and substrings in strings. Used by regex, ripgrep and other popular crates
- [wide](https://crates.io/crates/wide) - vector types for cross-platform SIMD, similar to portable_simd on nightly but works with stable
- [aho-corasick](https://crates.io/crates/aho-corasick) - library for fast searching of patterns in strings, also used by regex
