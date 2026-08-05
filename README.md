# Qubit Model

[![Rust CI](https://github.com/qubit-ltd/rs-model/actions/workflows/ci.yml/badge.svg)](https://github.com/qubit-ltd/rs-model/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://qubit-ltd.github.io/rs-model/coverage-badge.json)](https://qubit-ltd.github.io/rs-model/coverage/)
[![Crates.io](https://img.shields.io/crates/v/qubit-model.svg?color=blue)](https://crates.io/crates/qubit-model)
[![Rust](https://img.shields.io/badge/rust-1.94+-blue.svg?logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![中文文档](https://img.shields.io/badge/文档-中文版-blue.svg)](README.zh_CN.md)

`qubit-model` is the home for domain objects migrated to the Qubit Rust platform. It is being established as a separate crate so domain models can evolve independently from the metadata runtime and derive macro crates.

## Intended Users

This repository is intended for Qubit developers who migrate or maintain domain objects used by Rust applications and shared infrastructure.

## Installation

The crate is currently a local scaffold and is not yet published to crates.io. Once domain objects are added, local development can use a path dependency:

```toml
[dependencies]
qubit-model = { version = "0.1", path = "../rs-model" }
```

## Current Starting Point

The repository currently contains only the minimum Rust library layout and standard project tooling. It does not expose domain types yet.

## Planned Scope

Domain objects will be added here incrementally, with model declarations designed to consume the shared capabilities of `rs-model-metadata` and `rs-model-derive`.

## Known Limits

- No domain object has been migrated yet.
- No public model API, persistence mapping, validation API, or generated schema is provided.
- The repository does not currently provide DAO, Service, random-object, or contract-test implementations.

## Testing

```bash
# Run tests with the default feature set
cargo test

# Run tests with all declared features
cargo test --all-features

# Project CI checks
./ci-check.sh

# Check code coverage
./coverage.sh
```

## License

Copyright (c) 2025 - 2026. Haixing Hu. All rights reserved.

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for the
full license text.

## Contributing

Contributions are welcome. Please follow the Rust API guidelines, keep public
API documentation and tests current, and run `./align-ci.sh` to format code and
`./ci-check.sh` to satisfy CI requirements before submitting a pull request.

## Author

**Haixing Hu** - *Qubit Co. Ltd.*

Repository: [https://github.com/qubit-ltd/rs-model](https://github.com/qubit-ltd/rs-model)
