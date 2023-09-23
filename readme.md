# rrplug
[![crates.io](https://img.shields.io/crates/v/rrplug)](https://crates.io/crates/rrplug)
[![docs.rs](https://docs.rs/rrplug/badge.svg)](https://docs.rs/rrplug)
[![rrplug-test-build](https://github.com/R2NorthstarTools/rrplug/actions/workflows/rrplug-test-build.yml/badge.svg?branch=master)](https://github.com/R2NorthstarTools/rrplug/actions/workflows/rrplug-test-build.yml)

crate that provides function wappers and functions for [R2Northstar](https://github.com/R2Northstar/NorthstarLauncher) plugin creation.

rrplug uses compile time or sometimes runtime checks to guarantee safety in abstractions

## rrplug template

install cargo-generate if you don't have it
```bash
cargo install cargo-generate
```

```bash
cargo generate -g  https://github.com/catornot/rrplug.git
```

a git [template](https://github.com/catornot/rrplug-template) also exists but it may or not be maintained as well 

## versioning
rrplug had a major rewrite for each plugins version so versions that a `x` plugins version are `x.\*.\*`
| rrplug  | plugins |
| :-----: | :-----: |
| `3.*.*` |  `v3`   |
| `2.*.*` |  `v2`   |
| `0.1.*` |  `v1`   |