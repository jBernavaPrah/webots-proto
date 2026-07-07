use webots_proto_ast::proto::parser::Parser;

#[test]
fn importable_externproto_parses_and_canonical_round_trips() {
    let src = "IMPORTABLE EXTERNPROTO \"Robot.proto\"\n";
    let doc = Parser::new(src).parse_document().expect("parse");

    assert_eq!(doc.externprotos.len(), 1);
    let ep = &doc.externprotos[0];
    assert!(ep.importable, "IMPORTABLE keyword sets the importable flag");
    assert_eq!(ep.url, "Robot.proto");

    // Canonical output re-emits the IMPORTABLE keyword.
    let canonical = doc.to_canonical_string().expect("canonical");
    assert!(
        canonical.contains("IMPORTABLE EXTERNPROTO \"Robot.proto\""),
        "canonical output should keep IMPORTABLE: {canonical}"
    );

    // Re-parsing canonical output preserves the flag.
    let reparsed = Parser::new(&canonical).parse_document().expect("reparse");
    assert!(reparsed.externprotos[0].importable);
}

#[test]
fn plain_externproto_is_not_importable() {
    let src = "EXTERNPROTO \"Robot.proto\"\n";
    let doc = Parser::new(src).parse_document().expect("parse");

    assert!(!doc.externprotos[0].importable);
    let canonical = doc.to_canonical_string().expect("canonical");
    assert!(
        !canonical.contains("IMPORTABLE"),
        "a plain EXTERNPROTO must not gain IMPORTABLE: {canonical}"
    );
}

#[test]
fn importable_and_plain_externprotos_coexist_in_one_document() {
    let src = "EXTERNPROTO \"Arena.proto\"\nIMPORTABLE EXTERNPROTO \"Robot.proto\"\n";
    let doc = Parser::new(src).parse_document().expect("parse");

    assert_eq!(doc.externprotos.len(), 2);
    assert!(
        !doc.externprotos[0].importable,
        "Arena is a plain EXTERNPROTO"
    );
    assert!(doc.externprotos[1].importable, "Robot is IMPORTABLE");
}
