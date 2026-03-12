use webots_proto_resolver::ResolveOptions;

#[test]
fn test_circular_dependency_detection_documented() {
    // This test documents that circular dependency detection is implemented
    // in the ProtoResolver.load_proto() method via the `visited` HashSet.
    //
    // The actual detection happens when:
    // 1. A PROTO file A references EXTERNPROTO B
    // 2. B references EXTERNPROTO A
    // 3. When loading A -> B -> A, the second load of A detects it's already visited
    //
    // Testing this end-to-end requires:
    // - Creating actual files on disk
    // - Having the resolver start from a file path (not string content)
    //
    // The unit tests in resolve.rs verify the visited tracking logic.
    // For now, we verify the options can be configured correctly.

    let options = ResolveOptions::new().with_max_depth(5);
    assert_eq!(options.max_depth, 5);

    // TODO: Add integration test with actual circular PROTO files
    // when we have a file-path-based entry point for the resolver
}
