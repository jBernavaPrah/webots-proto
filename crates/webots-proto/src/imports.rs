use std::collections::HashSet;
use std::path::{Path, PathBuf};

use webots_proto_ast::Proto;
use webots_proto_schema::{Diagnostic, DiagnosticSet, Severity};

use crate::Error;

pub(crate) fn validate_local_externproto_naming(
    root_proto_path: &Path,
    root_document: &Proto,
) -> Result<DiagnosticSet, Error> {
    let mut visited = HashSet::new();
    let mut diagnostics = DiagnosticSet::new();
    let canonical_root = root_proto_path.canonicalize()?;
    collect_mismatches_recursive(
        &canonical_root,
        root_document,
        &mut visited,
        &mut diagnostics,
    )?;
    Ok(diagnostics)
}

fn collect_mismatches_recursive(
    proto_path: &Path,
    document: &Proto,
    visited: &mut HashSet<PathBuf>,
    diagnostics: &mut DiagnosticSet,
) -> Result<(), Error> {
    let canonical_proto = proto_path.canonicalize()?;
    if !visited.insert(canonical_proto.clone()) {
        return Ok(());
    }

    let base_dir = canonical_proto.parent().ok_or_else(|| {
        std::io::Error::other(format!(
            "PROTO has no parent: {}",
            canonical_proto.display()
        ))
    })?;

    for extern_proto in &document.externprotos {
        let import_url = extern_proto.url.trim();
        if import_url.starts_with("webots://")
            || import_url.starts_with("http://")
            || import_url.starts_with("https://")
        {
            continue;
        }

        let import_path = if Path::new(import_url).is_absolute() {
            PathBuf::from(import_url)
        } else {
            base_dir.join(import_url)
        };
        let import_file_name = import_path
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| std::io::Error::other("Imported PROTO path is not valid UTF-8"))?
            .to_string();
        let canonical_import = import_path.canonicalize()?;
        let imported_content = std::fs::read_to_string(&canonical_import)?;
        let imported_document: Proto = imported_content.parse()?;
        let imported_proto = imported_document.proto.clone().ok_or_else(|| {
            std::io::Error::other(format!(
                "Imported file {} does not contain a PROTO definition",
                canonical_import.display()
            ))
        })?;
        let expected_file_name = format!("{}.proto", imported_proto.name);
        if import_file_name != expected_file_name {
            diagnostics.add(Diagnostic {
                span: extern_proto.span.clone(),
                severity: Severity::Error,
                message: format!(
                    "EXTERNPROTO '{}' resolves to '{}' declaring PROTO '{}', expected file name '{}'",
                    import_url,
                    canonical_import.display(),
                    imported_proto.name,
                    expected_file_name
                ),
                suggestion: Some(format!("Reference '{}'", expected_file_name)),
            });
        }

        collect_mismatches_recursive(&canonical_import, &imported_document, visited, diagnostics)?;
    }

    Ok(())
}
