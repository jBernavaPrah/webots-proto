# webots-proto

Rust workspace for parsing, rendering, resolving, validating, and typing Webots PROTO files.

## Crates

- `webots-proto`: façade crate for most consumers
- `webots-proto-ast`: parser, AST, spans, writer
- `webots-proto-template`: JavaScript template evaluation
- `webots-proto-resolver`: `EXTERNPROTO` expansion
- `webots-proto-schema`: typed nodes, validation, and versioned codecs

## Recommended Dependency

```toml
[dependencies]
webots-proto = "0.1"
```

Use the façade crate unless you explicitly want one layer only.

### Façade Features

`webots-proto` supports optional layers:

- `template`: template rendering
- `resolver`: `EXTERNPROTO` expansion
- `schema`: diagnostics, typed conversions, schema exports
- `validation`: façade `validate()` support
- `r2025a`: typed R2025a codec and node exports

The default feature set enables all of them.

## Quick Start

```rust
use webots_proto::Proto;

let input = r#"#VRML_SIM R2025a utf8
PROTO MyRobot [ ] { Robot { } }
"#;

let proto: Proto = input.parse()?;
let canonical = proto.to_canonical_string()?;
println!("{canonical}");
```

```rust
use webots_proto::{Proto, ProtoExt, RenderOptions};

let proto: Proto = std::fs::read_to_string("robot.proto")?.parse()?;
let rendered = proto.render(&RenderOptions::default())?;
let diagnostics = proto.validate()?;
```

## Versioned Typed Nodes

```rust
use webots_proto::r2025a::{R2025aCodec, Robot};

let codec = R2025aCodec::new();
let robot: Robot = codec.decode(
    r#"#VRML_SIM R2025a utf8
Robot { name "my_robot" }
"#,
)?;
assert_eq!(robot.name, "my_robot");
```

## Repository Layout

- `crates/`: publishable crates
- `examples/`: per-crate example programs
- `fixtures/`: parsing and resolver fixtures

## Development

```bash
cargo fmt --all
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
cargo test
```

Parser and writer benchmarks live in `webots-proto-ast`:

```bash
cargo bench -p webots-proto-ast
```
