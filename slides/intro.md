---
title: Parsing text data with SIMD in Rust
description: Sometimes we need to extract very limited amount of data from very large text arrays.
  Traditional parsing libraries are not well suited for this particular task.
  In this talk we quickly deep into Rust feature which allows to write fast efficient parsers utilizing
  CPU SIMD capabilities.
author: Maxim Vorobjov
keywords: simd,rust,parsing,high performance
---

![bg height:100%](slide0.jpg)

---

# Processing texts with SIMD in Rust

Maxim Vorobjov
@ 
Volition Technologies

![bg right](img/highway.png)

---

# Plan

- What and why
- A few common use cases
- Examples presentation
- Disclaimer

