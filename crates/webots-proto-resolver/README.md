# webots-proto-resolver

`EXTERNPROTO` resolution for Webots PROTO files.

This crate expands referenced PROTO files into a resolved AST tree. It uses `webots-proto-ast` for parsing and `webots-proto-template` when nested PROTO bodies contain template blocks.

## Example

```rust
use std::path::Path;
use webots_proto_resolver::{ProtoResolver, ResolveOptions};

let input = r#"#VRML_SIM R2025a utf8
PROTO Demo [] { Group {} }
"#;

let root = ProtoResolver::new(ResolveOptions::new())
    .to_root_node(input, Some(Path::new(".")))?;
println!("{:?}", root.kind);
# Ok::<(), Box<dyn std::error::Error>>(())
```
