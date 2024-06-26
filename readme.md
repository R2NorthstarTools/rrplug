# rrplug
[![crates.io](https://img.shields.io/crates/v/rrplug)](https://crates.io/crates/rrplug)
[![docs.rs](https://docs.rs/rrplug/badge.svg)](https://docs.rs/rrplug)
[![rrplug-test-build](https://github.com/R2NorthstarTools/rrplug/actions/workflows/rrplug-test-build.yml/badge.svg?branch=master)](https://github.com/R2NorthstarTools/rrplug/actions/workflows/rrplug-test-build.yml)

framework for working with [R2Northstar](https://northstar.tf/)'s plugin system.

this crate provides convenient abstractions with compile time checks while not limiting unsafe access to any parts of plugin sys or the titanfall 2 engine.

## rrplug template

plugins v4 removed some dependencies on external files so now templates are redundant but a [template](https://github.com/catornot/rrplug-template) still exists

## cross compiling plugins

To compile a plugin from a host machine that's not using windows, you need to install the required target.

```bash
rustup target add x86_64-pc-windows-gnu
```

Then create a cargo config to always compile your project for windows.

```bash
mkdir .cargo
echo "[build]\ntarget = \"x86_64-pc-windows-gnu\"" > .cargo/config.toml
```

## versioning
rrplug had a major rewrite for each plugins version so versions that a `x` plugins version are `x.\*.\*`
| rrplug  | plugins |
| :-----: | :-----: |
| `4.*.*` |  `v4`   |
| `3.*.*` |  `v3`   |
| `2.*.*` |  `v2`   |
| `0.1.*` |  `v1`   |
