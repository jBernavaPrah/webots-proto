# webots-proto-template

Template evaluation for Webots PROTO files.

This crate evaluates Webots `%< ... >%` and `%<= ... >%` template blocks on top of the AST from `webots-proto-ast`.

## Example

```rust
use webots_proto_ast::Proto;
use webots_proto_template::RenderOptions;

let proto: Proto = r#"#VRML_SIM R2025a utf8
PROTO Demo [
  field SFInt32 count 2
] {
  WorldInfo { title "%<= fields.count.value >%" }
}
"#
.parse()?;

let rendered = webots_proto_template::render(&proto, &RenderOptions::default())?;
println!("{rendered}");
# Ok::<(), Box<dyn std::error::Error>>(())
```
