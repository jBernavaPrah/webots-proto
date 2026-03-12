# webots-proto-ast

Parser, AST, spans, and writer for Webots PROTO files.

This crate owns the raw syntax tree and round-trip serialization layer. It does not do template evaluation, `EXTERNPROTO` resolution, or semantic validation.

## Example

```rust
use webots_proto_ast::Proto;

let proto: Proto = r#"#VRML_SIM R2025a utf8
PROTO Demo [] { Group {} }
"#
.parse()?;

let canonical = proto.to_canonical_string()?;
println!("{canonical}");
# Ok::<(), Box<dyn std::error::Error>>(())
```
