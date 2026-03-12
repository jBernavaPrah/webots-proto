use std::error::Error;
use webots_proto_ast::Proto;

fn main() -> Result<(), Box<dyn Error>> {
    let input = r#"#VRML_SIM R2025a utf8
PROTO  MyRobot  [ ] { Robot { } }"#;

    let document: Proto = input.parse()?;
    println!("--- Lossless ---");
    println!("{}", document.to_lossless_string()?);

    println!("\n--- Canonical ---");
    println!("{}", document.to_canonical_string()?);

    Ok(())
}
