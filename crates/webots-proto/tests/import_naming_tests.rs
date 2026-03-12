use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use webots_proto::Proto;
use webots_proto::ProtoExt;

fn make_temp_dir(prefix: &str) -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock must be monotonic")
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("webots-proto-import-naming-{prefix}-{timestamp}"));
    std::fs::create_dir_all(&dir).expect("failed to create test directory");
    dir
}

#[test]
fn local_import_name_alignment_no_mismatch() {
    let dir = make_temp_dir("aligned");
    let child_path = dir.join("Child.proto");
    let root_path = dir.join("Root.proto");

    std::fs::write(
        &child_path,
        r#"#VRML_SIM R2025a utf8
PROTO Child [] { Group { children [] } }
"#,
    )
    .expect("write child proto");
    std::fs::write(
        &root_path,
        r#"#VRML_SIM R2025a utf8
EXTERNPROTO "Child.proto"
PROTO Root [] { Child { } }
"#,
    )
    .expect("write root proto");

    let document = Proto::from_file(&root_path).expect("parse imports");
    let diagnostics = document.validate().expect("validate imports");
    assert!(
        !diagnostics.has_errors(),
        "expected no validation errors, got {:?}",
        diagnostics.iter().collect::<Vec<_>>()
    );
}

#[test]
fn local_import_name_alignment_detects_mismatch() {
    let dir = make_temp_dir("mismatch");
    let child_path = dir.join("child.proto");
    let root_path = dir.join("Root.proto");

    std::fs::write(
        &child_path,
        r#"#VRML_SIM R2025a utf8
PROTO Child [] { Group { children [] } }
"#,
    )
    .expect("write child proto");
    std::fs::write(
        &root_path,
        r#"#VRML_SIM R2025a utf8
EXTERNPROTO "child.proto"
PROTO Root [] { Child { } }
"#,
    )
    .expect("write root proto");

    let document = Proto::from_file(&root_path).expect("parse imports");
    let diagnostics = document.validate().expect("validate imports");
    assert!(
        diagnostics.has_errors(),
        "expected mismatch validation error"
    );
    let rendered = diagnostics
        .iter()
        .map(|item| item.message.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    assert!(rendered.contains("declaring PROTO 'Child'"));
    assert!(rendered.contains("expected file name 'Child.proto'"));
}

#[test]
fn local_import_name_alignment_checks_nested_imports() {
    let dir = make_temp_dir("nested");
    let nested_path = dir.join("nested_case.proto");
    let child_path = dir.join("Child.proto");
    let root_path = dir.join("Root.proto");

    std::fs::write(
        &nested_path,
        r#"#VRML_SIM R2025a utf8
PROTO NestedCase [] { Group { children [] } }
"#,
    )
    .expect("write nested proto");
    std::fs::write(
        &child_path,
        r#"#VRML_SIM R2025a utf8
EXTERNPROTO "nested_case.proto"
PROTO Child [] { NestedCase { } }
"#,
    )
    .expect("write child proto");
    std::fs::write(
        &root_path,
        r#"#VRML_SIM R2025a utf8
EXTERNPROTO "Child.proto"
PROTO Root [] { Child { } }
"#,
    )
    .expect("write root proto");

    let document = Proto::from_file(&root_path).expect("parse imports");
    let diagnostics = document.validate().expect("validate imports");
    assert!(
        diagnostics.has_errors(),
        "expected nested mismatch validation error"
    );
    let rendered = diagnostics
        .iter()
        .map(|item| item.message.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    assert!(rendered.contains("nested_case.proto"));
    assert!(rendered.contains("expected file name 'NestedCase.proto'"));
}
