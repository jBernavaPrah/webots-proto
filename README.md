# webots-proto

[![Crates.io](https://img.shields.io/crates/v/webots-proto.svg)](https://crates.io/crates/webots-proto)
[![License](https://img.shields.io/crates/l/webots-proto.svg)](https://github.com/jBernavaPrah/webots-proto/blob/main/LICENSE)

Rust workspace for parsing, rendering, resolving, validating, and typing Webots PROTO files.

## Overview

This repository is split into focused crates:

- `webots-proto`: façade crate for most consumers
- `webots-proto-ast`: parser, AST, spans, writer
- `webots-proto-template`: JavaScript template evaluation
- `webots-proto-resolver`: `EXTERNPROTO` expansion
- `webots-proto-schema`: validation, typed nodes, and versioned codecs

For most use cases, depend on `webots-proto`.

```toml
[dependencies]
webots-proto = "0.1"
```

## Example

```rust
use webots_proto::{Proto, ProtoExt, RenderOptions};

let proto: Proto = r#"#VRML_SIM R2025a utf8
PROTO Demo [
  field SFInt32 count 2
] {
  WorldInfo { title "%<= fields.count.value >%" }
}
"#
.parse()?;

let rendered = proto.render(&RenderOptions::default())?;
let diagnostics = proto.validate()?;

assert!(!diagnostics.has_errors());
println!("{rendered}");
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Façade Features

`webots-proto` exposes optional layers:

- `template`: template rendering
- `resolver`: `EXTERNPROTO` expansion
- `schema`: diagnostics, schema exports, typed conversions
- `validation`: façade validation helpers
- `r2025a`: typed R2025a nodes and codec

Default features enable the full façade.

## Repository Layout

- `crates/`: publishable crates
- `fixtures/`: parser and resolver fixtures

Examples and benches live in the crates that own them.

## Development

```bash
cargo fmt --all
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
cargo test -q
```

Parser and writer benchmarks live in `webots-proto-ast`:

```bash
cargo bench -p webots-proto-ast
```

## Release Flow

- Pull requests run format, check, clippy, and tests in CI
- `release-plz` manages release PRs, tags, and crates.io publishing from `main`
