use std::error::Error;
use std::fs;
use std::path::PathBuf;
use webots_proto_ast::Proto;

fn main() -> Result<(), Box<dyn Error>> {
    let fixtures = [
        "Pedestrian.proto",
        "Pioneer3dx.proto",
        "RobocupSoccerField.proto",
        "Simple.proto",
    ];

    let fixtures_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../fixtures");

    for filename in fixtures {
        let file_path = fixtures_dir.join(filename);
        println!("Parsing {}...", filename);

        let input = fs::read_to_string(&file_path)?;
        let document: Proto = input.parse()?;

        if let Some(header) = &document.header {
            println!("  Header: {} {}", header.version, header.encoding);
        }

        if let Some(proto) = &document.proto {
            println!("  PROTO Name: {}", proto.name);
            println!("  Fields: {}", proto.fields.len());
        }
    }

    Ok(())
}
