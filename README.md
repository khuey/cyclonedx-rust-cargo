[![Build Status](https://github.com/CycloneDX/cyclonedx-rust-cargo/workflows/Rust%20CI/badge.svg)](https://github.com/CycloneDX/cyclonedx-rust-cargo/actions?workflow=Rust+CI)
[![Crates.io](https://img.shields.io/crates/v/cyclonedx-bom.svg)](https://crates.io/crates/cyclonedx-bom)
[![License](https://img.shields.io/badge/license-Apache%202.0-brightgreen.svg)][License]
[![Website](https://img.shields.io/badge/https://-cyclonedx.org-blue.svg)](https://cyclonedx.org/)
[![Slack Invite](https://img.shields.io/badge/Slack-Join-blue?logo=slack&labelColor=393939)](https://cyclonedx.org/slack/invite)
[![Group Discussion](https://img.shields.io/badge/discussion-groups.io-blue.svg)](https://groups.io/g/CycloneDX)
[![Twitter](https://img.shields.io/twitter/url/http/shields.io.svg?style=social&label=Follow)](https://twitter.com/CycloneDX_Spec)

# CycloneDX Rust (Cargo) Plugin
The CycloneDX module for Rust (Cargo) creates a valid CycloneDX Software Bill-of-Material (SBOM) containing an 
aggregate of all project dependencies. CycloneDX is a lightweight SBOM specification that is easily created, human 
and machine readable, and simple to parse. 

## Usage
Execute CycloneDX from within a Rust project directory containing Cargo.toml.

#### Installing
```bash
cargo install cyclonedx-bom
```

#### Executing binary
```bash
~/.cargo/bin/cargo-cyclonedx cyclonedx
```

#### Executing from cargo
```bash
cargo cyclonedx
```


## Copyright & License

CycloneDX Rust Cargo is Copyright (c) OWASP Foundation. All Rights Reserved.

Permission to modify and redistribute is granted under the terms of the Apache 2.0 license. See the [LICENSE] file for the full license.

[License]: https://github.com/CycloneDX/cyclonedx-rust-cargo/blob/master/LICENSE
