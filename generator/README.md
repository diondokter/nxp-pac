# How to Use the Generator

## Preparing your environment

To be able to run the generator you need install locally:
- [form](https://crates.io/crates/form) - Used to format the generated code. If you try to install this tool via your package manager (apt, zypper, etc.) you may install a different tool. Please install it from cargo, as shown below.

```bash
cargo install form
```

## Running the Generator

You must switch to the root of the repository, which is the parent directory of
the generator crate directory.

```
cargo run -p generator
```
