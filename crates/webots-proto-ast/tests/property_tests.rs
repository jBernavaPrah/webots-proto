use proptest::prelude::*;
use std::fs;
use std::path::PathBuf;
use webots_proto_ast::proto::ast::*;
use webots_proto_ast::proto::parser::Parser;
use webots_proto_ast::proto::span::Span;

// Define strategies for generating random FieldValues
fn field_value_strategy() -> impl Strategy<Value = FieldValue> {
    prop_oneof![
        any::<bool>().prop_map(FieldValue::Bool),
        any::<i64>().prop_map(|i| FieldValue::Int(i, None)),
        any::<f64>().prop_map(|f| FieldValue::Float(f, None)),
        "[a-zA-Z0-9_]+".prop_map(FieldValue::String), // Simple alphanumeric strings for now
        prop::array::uniform2(any::<f64>()).prop_map(FieldValue::Vec2f),
        prop::array::uniform3(any::<f64>()).prop_map(FieldValue::Vec3f),
        prop::array::uniform4(any::<f64>()).prop_map(FieldValue::Rotation),
        prop::array::uniform3(any::<f64>()).prop_map(FieldValue::Color),
        // Simple Is binding
        "[a-zA-Z0-9_]+".prop_map(FieldValue::Is),
        // Just(FieldValue::Null), // Null behavior in writer might be specific
    ]
}

proptest! {
    #[test]
    fn test_field_value_round_trip(val in field_value_strategy()) {
        // Skip IS and string containing double quotes for now to avoid escaping issues in this simple harness
        if let FieldValue::Is(_) = val {
             return Ok(());
        }
        if let FieldValue::String(ref s) = val
            && (s.contains('"') || s.contains('\\')) {
                return Ok(());
            }

        // Determine a compatible type for the field definition
        let field_type = if let FieldValue::Bool(_) = val {
            FieldType::SFBool
        } else if let FieldValue::Int(_, _) = val {
            FieldType::SFInt32
        } else if let FieldValue::Float(_, _) = val {
            FieldType::SFFloat
        } else if let FieldValue::String(_) = val {
            FieldType::SFString
        } else if let FieldValue::Vec2f(_) = val {
            FieldType::SFVec2f
        } else if let FieldValue::Vec3f(_) = val {
            FieldType::SFVec3f
        } else if let FieldValue::Rotation(_) = val {
            FieldType::SFRotation
        } else if let FieldValue::Color(_) = val {
            FieldType::SFColor
        } else {
            FieldType::SFString
        };

        // Construct a minimal AST:
        // PROTO Test [ field <TYPE> val <VALUE> ] {}

        let span = Span::default();
        let field = ProtoField::new(
            "testVal".to_string(),
            field_type,
            FieldKeyword::Field,
            span.clone()
        ).with_default_value(val.clone());

        let proto = ProtoDefinition::new("Test".to_string(), span.clone())
            .with_fields(vec![field]);

        let doc = Proto::new().with_proto(proto);

        let output = doc.to_lossless_string().unwrap();

        let mut parser = Parser::new(&output);
        let parsed_doc = parser.parse_document().unwrap();

        // Extract the value back
        let parsed_proto = parsed_doc.proto.unwrap();
        let parsed_field = &parsed_proto.fields[0];
        let parsed_val = parsed_field.default_value.as_ref().unwrap();

        // Compare (approximate for floats?)
        // PartialEq on FieldValue handles exact match.
        // For floats, generated values might have precision issues when printed/parsed?
        // Let's rely on standard Eq for now, if it fails we might need custom comparison.

        // Note: Int(1, None) vs Int(1, Some("1"))
        // parser produces values with raw strings attached. Our input `val` likely has None.
        // We need to strip raw strings for comparison or impl PartialEq that ignores them?
        // FieldValue PartialEq likely checks all fields.
        // Let's strip raw from parsed_val.

        fn strip_raw_and_normalize(v: FieldValue) -> FieldValue {
            if let FieldValue::Int(i, _) = v {
                FieldValue::Int(i, None)
            } else if let FieldValue::Float(f, _) = v {
                FieldValue::Float(f, None)
            } else if let FieldValue::Vec2f(a) = v {
                FieldValue::Vec2f(a)
            } else if let FieldValue::Vec3f(a) = v {
                FieldValue::Vec3f(a)
            } else if let FieldValue::Rotation(a) = v {
                FieldValue::Rotation(a)
            } else if let FieldValue::Color(a) = v {
                FieldValue::Color(a)
            } else if let FieldValue::NumberSequence(seq) = v {
                let stripped_elements: Vec<NumberSequenceElement> = seq
                    .elements
                    .into_iter()
                    .map(|element| NumberSequenceElement::new(strip_raw_and_normalize(element.value)))
                    .collect();
                FieldValue::NumberSequence(NumberSequence::new().with_elements(stripped_elements))
            } else {
                v
            }
        }

        let val_norm = strip_raw_and_normalize(val.clone());
        let parsed_norm = strip_raw_and_normalize(parsed_val.clone());

        // Custom equality checking: if val is Vec/Color/Rotation and parsed is NumberSequence with matching floats, accept it.
        fn equivalent(a: &FieldValue, b: &FieldValue) -> bool {
            if a == b {
                return true;
            }

            if let (FieldValue::Vec2f(arr), FieldValue::NumberSequence(seq)) = (a, b) {
                return seq_matches_floats(seq, arr);
            }
            if let (FieldValue::Vec3f(arr), FieldValue::NumberSequence(seq)) = (a, b) {
                return seq_matches_floats(seq, arr);
            }
            if let (FieldValue::Color(arr), FieldValue::NumberSequence(seq)) = (a, b) {
                return seq_matches_floats(seq, arr);
            }
            if let (FieldValue::Rotation(arr), FieldValue::NumberSequence(seq)) = (a, b) {
                return seq_matches_floats(seq, arr);
            }
            if let (FieldValue::Int(i, _), FieldValue::NumberSequence(seq)) = (a, b) {
                if seq.elements.len() == 1
                    && let FieldValue::Int(si, _) = &seq.elements[0].value
                {
                    return *i == *si;
                }
                return false;
            }
            if let (FieldValue::Float(f, _), FieldValue::NumberSequence(seq)) = (a, b) {
                if seq.elements.len() == 1 {
                    let value = &seq.elements[0].value;
                    if let FieldValue::Float(sf, _) = value {
                        return (*f - sf).abs() < 1e-6;
                    }
                    if let FieldValue::Int(si, _) = value {
                        return (*f - (*si as f64)).abs() < 1e-6;
                    }
                }
                return false;
            }

            false
        }

        fn seq_matches_floats(seq: &NumberSequence, floats: &[f64]) -> bool {
            if seq.elements.len() != floats.len() {
                return false;
            }
            for (element, &expected) in seq.elements.iter().zip(floats.iter()) {
                if let FieldValue::Float(f, _) = &element.value {
                    if (*f - expected).abs() > 1e-6 {
                        return false;
                    }
                } else if let FieldValue::Int(i, _) = &element.value {
                    if (*i as f64 - expected).abs() > 1e-6 {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            true
        }

        if !equivalent(&val_norm, &parsed_norm) {
             // Panic with details
             panic!("Round trip mismatch:\nOriginal: {:?}\nParsed: {:?}", val_norm, parsed_norm);
        }
    }
}

#[test]
fn test_fixture_idempotency_check() {
    let fixtures_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../fixtures");
    let paths: Vec<_> = fs::read_dir(fixtures_dir)
        .expect("Failed to read fixtures")
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().map(|e| e == "proto").unwrap_or(false))
        .collect();

    for path in paths {
        let content = fs::read_to_string(&path).expect("Failed to read fixture");
        let mut parser = Parser::new(&content);
        let doc = parser.parse_document().expect("Failed to parse fixture");

        let output1 = doc.to_lossless_string().unwrap();

        let mut parser2 = Parser::new(&output1);
        let doc2 = parser2.parse_document().expect("Failed to parse output1");

        let output2 = doc2.to_lossless_string().unwrap();

        // This is the canonical property: parse -> write -> parse -> write should be stable.
        // output1 might differ from content (formatting), but output2 should equal output1 (idempotency).
        if output1 != output2 {
            panic!(
                "Idempotency failure for {:?}.\nLen1: {}, Len2: {}",
                path,
                output1.len(),
                output2.len()
            );
        }
    }
}
