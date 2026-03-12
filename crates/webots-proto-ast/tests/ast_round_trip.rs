use std::fs;
use std::path::PathBuf;
use webots_proto_ast::proto::parser::Parser;

#[test]
fn test_lossless_round_trip_pedestrian() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/Pedestrian.proto");
    let content = fs::read_to_string(path).expect("Failed to read Pedestrian.proto");

    let mut parser = Parser::new(&content);
    let doc = parser
        .parse_document()
        .expect("Failed to parse Pedestrian.proto");

    let output = doc.to_lossless_string().expect("failed to write lossless");

    let mut parser2 = Parser::new(&output);
    if let Err(e) = parser2.parse_document() {
        let preview_len = output.len().min(500);
        panic!(
            "Failed to re-parse generated output: {:?}\nOutput start: {}",
            e,
            &output[..preview_len]
        );
    }

    assert_eq!(content, output, "Lossless output must match input exactly.");

    // Basic structural check
    assert!(output.contains("PROTO Pedestrian"));
    assert!(output.contains("EXTERNPROTO \"PedestrianTorso.proto\""));

    // Check if critical raw content is preserved
    assert!(output.contains("%< const rigid = fields.controllerArgs.value.length == 0; >%"));
    assert!(output.contains("DEF LEFT_ARM HingeJoint"));

    // If we want to debug the diff:
    if content != output {
        println!(
            "Content length: {}, Output length: {}",
            content.len(),
            output.len()
        );
        // Uncomment to see diff
        // println!("Diff:");
        // for diff in diff::lines(&content, &output) {
        //     match diff {
        //         diff::Result::Left(l) => println!("-{}", l),
        //         diff::Result::Right(r) => println!("+{}", r),
        //         diff::Result::Both(l, _) => println!(" {}", l),
        //     }
        // }
    }
}

#[test]
fn test_lossless_round_trip_samples() {
    let samples_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../fixtures");
    let mut sample_paths: Vec<_> = fs::read_dir(samples_dir)
        .expect("Failed to read samples directory")
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.extension().map(|ext| ext == "proto").unwrap_or(false))
        .collect();
    sample_paths.sort();

    for sample_path in sample_paths {
        let content = fs::read_to_string(&sample_path).expect("Failed to read sample file");
        // Ensure each sample parses, writes losslessly, and re-parses.
        let mut parser = Parser::new(&content);
        let document = parser
            .parse_document()
            .unwrap_or_else(|e| panic!("Failed to parse {:?}: {:?}", sample_path, e));

        let output = document
            .to_lossless_string()
            .expect("failed to write lossless");
        assert_eq!(
            content, output,
            "Lossless output must match input exactly for {:?}.",
            sample_path
        );

        let mut parser2 = Parser::new(&output);
        if let Err(e) = parser2.parse_document() {
            let preview_len = output.len().min(500);
            panic!(
                "Failed to re-parse {:?}: {:?}\nOutput start: {}",
                sample_path,
                e,
                &output[..preview_len]
            );
        }
    }
}

#[test]
fn test_lossless_round_trip_constrained_field_restrictions() {
    let input = r#"#VRML_SIM R2025a utf8
PROTO Sample [
  field SFString { "adult", "kid" } size "adult"
] {
  Robot {
    name "example"
  }
}
"#;
    let mut parser = Parser::new(input);
    let document = parser
        .parse_document()
        .expect("Failed to parse constrained field");

    let output = document
        .to_lossless_string()
        .expect("failed to write lossless");

    let normalized = output.split_whitespace().collect::<String>();
    assert!(normalized.contains("SFString"));
    assert!(normalized.contains("{\"adult\",\"kid\"}"));
}
