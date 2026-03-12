use std::error::Error;
use std::path::PathBuf;
use webots_proto_ast::Proto;
use webots_proto_resolver::{ProtoResolver, ResolveOptions};

fn main() -> Result<(), Box<dyn Error>> {
    let fixtures_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures")
        .join("resolve");
    let input_path = fixtures_dir.join("ParentUsesChildDefaults.proto");
    let input = std::fs::read_to_string(&input_path)?;

    let expanded_node = ProtoResolver::new(ResolveOptions::new().with_max_depth(8))
        .to_root_node(&input, Some(&fixtures_dir))?;

    let expanded_document = Proto::new().with_root_nodes(vec![expanded_node]);
    println!("{}", expanded_document.to_canonical_string()?);

    Ok(())
}
