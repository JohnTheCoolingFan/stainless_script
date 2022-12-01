# Stainless Script

Stainless Script is a visual node-based programming language.

The structure is as follows: program contains classes and objects (constants). Class desribes a data type and its associated methods, which are nodes and their variations. Nodes are executed one-by-one in sequence. Additionally, data can be passed from one node to other using connections that carry Objects. Node can specify what Class are the objects it outputs or inputs.

This repo contains basic data structures and logic for executing the code, as well as an executor binary that can be modified to add native rust modules by forking the repo.

For building the executor, use the following command:
```
cargo build --release --bin ssce
```
The resulting binary would be located at `target/release/ssce`

Join the [discord server](https://discord.gg/ceudNhN6cr) for getting the latest info about development progress
