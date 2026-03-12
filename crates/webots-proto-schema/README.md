# webots-proto-schema

Versioned schemas, diagnostics, and typed Webots nodes.

This crate owns:

- AST validation
- runtime semantic validation helpers
- typed node definitions
- versioned codecs such as `r2025a::R2025aCodec`

## Example

```rust
use webots_proto_ast::Proto;

let proto: Proto = r#"#VRML_SIM R2025a utf8
Robot { name "demo" }
"#
.parse()?;

let diagnostics = webots_proto_schema::validate(&proto);
assert!(!diagnostics.has_errors());
# Ok::<(), Box<dyn std::error::Error>>(())
```
