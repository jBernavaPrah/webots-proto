use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use webots_proto::Proto;
use webots_proto::{ProtoExt, Severity};

fn make_temp_dir(prefix: &str) -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock must be monotonic")
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("webots-proto-runtime-{prefix}-{timestamp}"));
    std::fs::create_dir_all(&dir).expect("failed to create test directory");
    dir
}

#[test]
fn validate_file_reports_runtime_warnings_from_expanded_externproto_tree() {
    let dir = make_temp_dir("expanded-runtime");
    let wheel_path = dir.join("Wheel.proto");
    let root_path = dir.join("Root.proto");

    std::fs::write(
        &wheel_path,
        r#"#VRML_SIM R2025a utf8
PROTO Wheel [] {
  Solid {
    physics Physics {
      mass 1
    }
  }
}
"#,
    )
    .expect("write Wheel.proto");

    std::fs::write(
        &root_path,
        r#"#VRML_SIM R2025a utf8
EXTERNPROTO "Wheel.proto"
PROTO Root [] {
  Solid {
    children [
      Wheel { }
      Wheel { }
    ]
  }
}
"#,
    )
    .expect("write Root.proto");

    let document = Proto::from_file(&root_path).expect("parse Root.proto");
    let diagnostics = document.validate().expect("validate Root.proto");
    let warning_messages: Vec<_> = diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.severity == Severity::Warning)
        .map(|diagnostic| diagnostic.message.as_str())
        .collect();

    assert!(
        warning_messages
            .iter()
            .any(|message| message.contains("Undefined inertia matrix")),
        "expected inertia warning in expanded tree; warnings: {warning_messages:?}"
    );
    assert!(
        warning_messages
            .iter()
            .any(|message| message.contains("unique among sibling Solid nodes")),
        "expected sibling Solid.name warning in expanded tree; warnings: {warning_messages:?}"
    );
}
