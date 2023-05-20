[![crates.io](https://img.shields.io/crates/v/stainless_script?style=flat)](https://crates.io/crates/stainless_script) [![crates.io](https://img.shields.io/crates/d/stainless_script?style=flat)](https://crates.io/crates/stainless_script)

# Stainless Script

Stainless Script is a visual node-based programming language.

The structure is as follows: program contains classes, objects (constants) and nodes and their connections. Class desribes a data type and its associated methods, which are nodes and their variations. Nodes are executed one-by-one in sequence. Additionally, data can be passed from one node to other using connections that carry Objects. Node can specify what Class are the objects it outputs or inputs.

This repo contains basic data structures and logic for executing the code, as well as an executor binary (build instructions below) that can be modified to add native rust modules by forking the repo.

# Example programs

Example programs are located in `example_programs` directory. Currently there are only hello world programs in all three supported formats. They can be executed with the executor binary.

# Planned features

- Program-defined objects

# Executor

You can opt-out of the executor binary and its format features. To do so, set `default_features = false`.

Features (all enabled by default):
- `executor-binary` - Build executor binary
- `format-json` - Enable support for json program format (`.json.ssc`)
- `format-bincode` - Enable support for bincode program format (`.bin.ssc`)

The [ron](https://crates.io/crates/ron) program format (`.ron.ssc`) is always enabled as ron is also used as a standard for defining objects.

Executor binary can be installed from crates.io:
```
cargo install stainless_script
```
The resulting binary is `ssce`.

For manually building the executor, use the following command:
```
cargo build --release --bin ssce
```
The resulting binary would be located at `target/release/ssce`

Join the [discord server](https://discord.gg/ceudNhN6cr) for getting the latest info about development progress
