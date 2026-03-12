use std::error::Error;
use webots_proto_ast::Proto;

fn main() -> Result<(), Box<dyn Error>> {
    let input = r#"#VRML_SIM R2025a utf8
PROTO Broken [
  field SFInt32 number "not a number"
]
{
  Group {}
}
"#;

    let document: Proto = input.parse()?;
    let diagnostics = webots_proto_schema::validate(&document);

    if diagnostics.has_errors() {
        println!("Validation found errors:");
        for diagnostic in diagnostics.iter() {
            println!("  [{:?}] {}", diagnostic.severity, diagnostic.message);
        }
    } else {
        println!("Validation successful");
    }

    Ok(())
}
