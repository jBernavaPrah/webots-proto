# webots-proto

Facade crate for working with Webots PROTO files.

Use this crate when you want one dependency that exposes:

- parsing via `Proto`
- template rendering via `ProtoExt::render()`
- validation via `ProtoExt::validate()`
- `EXTERNPROTO` expansion via `ProtoResolver`
- typed R2025a nodes via `r2025a`

## Features

- `template`
- `resolver`
- `schema`
- `validation`
- `r2025a`

Default features enable the full façade.

## Example

```rust
use webots_proto::{Proto, ProtoExt, RenderOptions};

let proto: Proto = r#"#VRML_SIM R2025a utf8
PROTO Demo [] { Group {} }
"#
.parse()?;

let rendered = proto.render(&RenderOptions::default())?;
let diagnostics = proto.validate()?;
assert!(!diagnostics.has_errors());
# Ok::<(), Box<dyn std::error::Error>>(())
```

Use the companion crates directly if you only want one layer of functionality.
