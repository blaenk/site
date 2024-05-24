+++
title = "Rust"
date = 2021-11-08

[work]
kind = "contribution"
+++

Rust is a modern systems programming language that emphasizes memory safety and safe concurrency.

I was an early Rust adopter since a little before 2014, back when Rust was in a constant state of flux, as the language hadn't been finalized for the first stable release.

At a minimum I was very active in adapting Rust ecosystem packages to the latest syntax and semantics incurred by [RFCs](https://github.com/rust-lang/rfcs) which I read voraciously.

I was known to the Rust core team as an early adopter and contributor to the Rust ecosystem. The team knew I was working on a [static site generator](https://github.com/diecast/diecast) which drove me to:

- contribute to many diverse libraries I needed for my project
- [build solutions myself](https://github.com/blaenk/hoedown) to fill gaps in the nascent Rust ecosystem
- [report Rust compiler bugs](https://github.com/rust-lang/rust/issues?q=is%3Aissue+sort%3Aupdated-desc+author%3Ablaenk+is%3Aclosed) that I often discovered by using bleeding-edge nightly features
- provide helpful usability feedback
- provide support to other early adopters [by answering questions on StackOverflow](https://stackoverflow.com/search?q=user:101090+[rust]), IRC, and so on

During the crunch leading up to the 1.0 release, the [file system module `std::fs`](https://doc.rust-lang.org/std/fs/) and the [file path module `std::path`](https://doc.rust-lang.org/std/path/) were being redesigned and Rust core developers specifically reached out to me for my opinions and feedback on their proposals at certain points [[1]](https://github.com/rust-lang/rust/pull/22208#issuecomment-74022467)[[2]](https://github.com/rust-lang/rfcs/pull/529#issuecomment-88283983), since they knew me to be an active user of those APIs.

Immediately after the redesign was actually implemented, I identified a subtle edge-case that was not accounted for and [quickly fixed it](https://github.com/rust-lang/rust/pull/22351) in time for the 1.0 release. Although the change is trivial, it is a culmination of:

1. being noticed by Rust core developers as an early, widespread adopter of and contributor to the Rust ecosystem
1. Rust core developers specifically reaching out to me for my insight
1. very closely following Rust's development to learn of bleeding edge features
1. using bleeding edge features barely introduced by RFCs
1. knowing "where" to mark the "X", so to speak
