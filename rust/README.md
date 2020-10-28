# Rust example for DNB Open Banking APIs

## Getting rust

Follow the instructions at [rustup.rs](https://rustup.rs/).

## Usage

See [the main readme][] of the repo for a description on how to
retrieve API key.

To configure the credentials use environment variables. You can
put them in a file called `.env` in this directory with the
following variables set.

```
API_KEY=
```

After adding the credentials you can run the example with

```shell
cargo run
```

## Running the tests

```shell
cargo test
```

[the main readme]: ../README.md
