use crate::error::{Error, Result};
use crate::proto::ast::*;
use crate::proto::lexer::{Lexer, Token, TokenKind};
use crate::proto::span::Span;

pub struct Parser<'a> {
    input: &'a str,
    lexer: Lexer<'a>,
    current_token: Token,
    peek_token: Option<Token>,
    prev_span: Span,
    header: Option<Header>,
}

impl<'a> Parser<'a> {
    /// Creates a new Parser instance.
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer::new(input);
        let first_token = lexer.next_token();
        let mut parser = Self {
            input,
            lexer,
            current_token: first_token,
            peek_token: None,
            prev_span: Span::default(),
            header: Self::parse_header(input),
        };
        parser.ensure_non_trivia();
        parser
    }

    fn parse_header(input: &str) -> Option<Header> {
        let mut offset = 0usize;

        for (line_index, raw_line_with_ending) in input.split_inclusive('\n').enumerate() {
            let raw_line_without_newline = raw_line_with_ending
                .strip_suffix('\n')
                .unwrap_or(raw_line_with_ending);
            let raw_line = raw_line_without_newline
                .strip_suffix('\r')
                .unwrap_or(raw_line_without_newline);
            let trimmed = raw_line.trim_start();
            if trimmed.is_empty() {
                offset += raw_line_with_ending.len();
                continue;
            }
            if !trimmed.starts_with("#VRML_SIM") {
                return None;
            }
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            let version = parts.get(1).unwrap_or(&"").to_string();
            let encoding = parts.get(2).unwrap_or(&"").to_string();
            let leading_ws = raw_line.len() - trimmed.len();
            let start = offset + leading_ws;
            let end = start + trimmed.len();
            let start_col = leading_ws + 1;
            let end_col = start_col + trimmed.chars().count();
            return Some(Header::new(
                version,
                encoding,
                Some(trimmed.to_string()),
                Span::new(
                    start,
                    end,
                    line_index + 1,
                    start_col,
                    line_index + 1,
                    end_col,
                ),
            ));
        }

        None
    }

    fn is_trivia(kind: &TokenKind) -> bool {
        matches!(
            kind,
            TokenKind::Whitespace(_)
                | TokenKind::Comment(_)
                | TokenKind::Newline(_)
                | TokenKind::Comma
        )
    }

    fn ensure_non_trivia(&mut self) {
        while Self::is_trivia(&self.current_token.kind) {
            if let Some(token) = self.peek_token.take() {
                self.current_token = token;
            } else {
                self.current_token = self.lexer.next_token();
            }
        }
    }

    fn advance(&mut self) {
        self.prev_span = self.current_token.span.clone();
        if let Some(token) = self.peek_token.take() {
            self.current_token = token;
        } else {
            self.current_token = self.lexer.next_token();
        }
        self.ensure_non_trivia();
    }

    fn peek(&mut self) -> TokenKind {
        if self.peek_token.is_none() {
            let mut token = self.lexer.next_token();
            while Self::is_trivia(&token.kind) {
                token = self.lexer.next_token();
            }
            self.peek_token = Some(token);
        }
        self.peek_token.as_ref().unwrap().kind.clone()
    }

    /// Parses a complete PROTO document.
    pub fn parse_document(&mut self) -> Result<Proto> {
        let header = self.header.take();

        let mut externprotos = Vec::new();
        while let TokenKind::ExternProto = self.current_token.kind {
            externprotos.push(self.parse_extern_proto()?);
        }

        let mut proto = None;
        if let TokenKind::Proto = self.current_token.kind {
            proto = Some(self.parse_proto_definition()?);
        }

        let mut root_nodes = Vec::new();
        while self.current_token.kind != TokenKind::Eof {
            if let TokenKind::Proto = self.current_token.kind {
                return Err(Error::Syntax {
                    line: self.current_token.span.start_line,
                    col: self.current_token.span.start_col,
                    message: "Unexpected second PROTO definition".into(),
                });
            }
            root_nodes.push(self.parse_node()?);
        }

        let mut doc = Proto::new()
            .with_externprotos(externprotos)
            .with_root_nodes(root_nodes);

        doc.header = header;

        if let Some(proto) = proto {
            doc = doc.with_proto(proto);
        }

        doc.source_content = Some(self.input.to_string());
        Ok(doc)
    }

    fn parse_extern_proto(&mut self) -> Result<ExternProto> {
        let start_span = self.current_token.span.clone();

        self.expect(TokenKind::ExternProto)?;

        let url_token = self.current_token.clone();
        let url = if let TokenKind::Str(value) = &url_token.kind {
            value.clone()
        } else {
            return Err(self.error(format!(
                "Expected string for EXTERNPROTO url, got {:?}",
                url_token.kind
            )));
        };
        self.advance();

        let mut alias = None;
        if let TokenKind::Identifier(value) = &self.current_token.kind
            && self.current_token.span.start_line == url_token.span.end_line
        {
            alias = Some(value.clone());
            self.advance();
        }

        Ok(ExternProto::new(url, alias, self.span_from(&start_span)))
    }

    fn parse_proto_definition(&mut self) -> Result<ProtoDefinition> {
        let start_span = self.current_token.span.clone();

        self.expect(TokenKind::Proto)?;
        let name = self.parse_identifier()?;
        self.expect(TokenKind::OpenBracket)?;

        let fields = self.parse_interface()?;

        self.expect(TokenKind::CloseBracket)?;
        self.expect(TokenKind::OpenBrace)?;

        let body = self.parse_body()?;

        let close_brace_span = self.current_token.span.clone();
        self.expect(TokenKind::CloseBrace)?;

        let span = Span::new(
            start_span.start,
            close_brace_span.end,
            start_span.start_line,
            start_span.start_col,
            close_brace_span.end_line,
            close_brace_span.end_col,
        );

        Ok(ProtoDefinition::new(name, span)
            .with_fields(fields)
            .with_body(body))
    }

    fn parse_interface(&mut self) -> Result<Vec<ProtoField>> {
        let mut fields = Vec::new();
        while self.current_token.kind != TokenKind::CloseBracket
            && self.current_token.kind != TokenKind::Eof
        {
            fields.push(self.parse_proto_field()?);
        }
        Ok(fields)
    }

    fn parse_proto_field(&mut self) -> Result<ProtoField> {
        let start_span = self.current_token.span.clone();

        let keyword = if self.current_token.kind == TokenKind::Field {
            FieldKeyword::Field
        } else if self.current_token.kind == TokenKind::VrmlField {
            FieldKeyword::VrmlField
        } else if self.current_token.kind == TokenKind::HiddenField {
            FieldKeyword::HiddenField
        } else if self.current_token.kind == TokenKind::DeprecatedField {
            FieldKeyword::DeprecatedField
        } else {
            return Err(self.error(format!(
                "Expected field keyword, got {:?}",
                self.current_token.kind
            )));
        };
        self.advance();

        let field_type_str = self.parse_identifier()?;

        let field_type = match field_type_str.as_str() {
            "SFBool" => FieldType::SFBool,
            "SFInt32" => FieldType::SFInt32,
            "SFFloat" => FieldType::SFFloat,
            "SFString" => FieldType::SFString,
            "SFVec2f" => FieldType::SFVec2f,
            "SFVec3f" => FieldType::SFVec3f,
            "SFRotation" => FieldType::SFRotation,
            "SFColor" => FieldType::SFColor,
            "SFNode" => FieldType::SFNode,
            "MFBool" => FieldType::MFBool,
            "MFInt32" => FieldType::MFInt32,
            "MFFloat" => FieldType::MFFloat,
            "MFString" => FieldType::MFString,
            "MFVec2f" => FieldType::MFVec2f,
            "MFVec3f" => FieldType::MFVec3f,
            "MFRotation" => FieldType::MFRotation,
            "MFColor" => FieldType::MFColor,
            "MFNode" => FieldType::MFNode,
            value => FieldType::Unknown(value.to_string()),
        };

        let mut restrictions = None;
        if self.current_token.kind == TokenKind::OpenBrace {
            self.advance();
            let mut elements = Vec::new();
            while self.current_token.kind != TokenKind::CloseBrace
                && self.current_token.kind != TokenKind::Eof
            {
                elements.push(self.parse_field_value()?);
            }
            self.expect(TokenKind::CloseBrace)?;
            restrictions = Some(elements);
        }

        let name = self.parse_identifier()?;
        let default_value = self.parse_field_value()?;

        let mut field = ProtoField::new(name, field_type, keyword, self.span_from(&start_span))
            .with_default_value(default_value);

        if let Some(restrictions) = restrictions {
            field = field.with_restrictions(restrictions);
        }

        Ok(field)
    }

    pub fn parse_body(&mut self) -> Result<Vec<ProtoBodyItem>> {
        let mut items = Vec::new();
        while !matches!(
            self.current_token.kind,
            TokenKind::CloseBrace | TokenKind::Eof
        ) {
            if matches!(self.current_token.kind, TokenKind::Template { .. }) {
                items.push(ProtoBodyItem::Template(
                    self.parse_template_block("PROTO body")?,
                ));
            } else {
                items.push(ProtoBodyItem::Node(self.parse_node()?));
            }
        }
        Ok(items)
    }

    fn parse_node(&mut self) -> Result<AstNode> {
        let start_span = self.current_token.span.clone();

        let mut def_name = None;
        if self.current_token.kind == TokenKind::Def {
            self.advance();
            def_name = Some(self.parse_identifier()?);
        }

        if def_name.is_some() && self.current_token.kind == TokenKind::Use {
            return Err(self.error("DEF cannot be followed by USE".into()));
        }

        if self.current_token.kind == TokenKind::Use {
            self.advance();
            let use_name = self.parse_identifier()?;
            return Ok(AstNode::new(
                AstNodeKind::Use { use_name },
                self.span_from(&start_span),
            ));
        }

        let type_name = self.parse_identifier()?;
        self.expect(TokenKind::OpenBrace)?;

        let mut fields = Vec::new();
        while self.current_token.kind != TokenKind::CloseBrace
            && self.current_token.kind != TokenKind::Eof
        {
            if let TokenKind::Template { .. } = self.current_token.kind {
                fields.push(NodeBodyElement::Template(
                    self.parse_template_block("node body")?,
                ));
                continue;
            }

            if self.current_token.kind == TokenKind::Def {
                fields.push(NodeBodyElement::Raw(RawSyntax::new(
                    Self::token_text(&self.current_token),
                    self.current_token.span.clone(),
                )));
                self.advance();
                if matches!(self.current_token.kind, TokenKind::Identifier(_)) {
                    fields.push(NodeBodyElement::Raw(RawSyntax::new(
                        Self::token_text(&self.current_token),
                        self.current_token.span.clone(),
                    )));
                    self.advance();
                }
                continue;
            }

            if self.is_field_start() {
                fields.push(NodeBodyElement::Field(self.parse_node_field()?));
            } else {
                fields.push(NodeBodyElement::Raw(RawSyntax::new(
                    Self::token_text(&self.current_token),
                    self.current_token.span.clone(),
                )));
                self.advance();
            }
        }

        self.expect(TokenKind::CloseBrace)?;

        Ok(AstNode::new(
            AstNodeKind::Node {
                type_name,
                def_name,
                fields,
            },
            self.span_from(&start_span),
        ))
    }

    fn is_field_start(&mut self) -> bool {
        if matches!(self.current_token.kind, TokenKind::Identifier(_)) {
            !matches!(self.peek(), TokenKind::OpenBrace)
        } else {
            false
        }
    }

    fn parse_node_field(&mut self) -> Result<NodeField> {
        let start_span = self.current_token.span.clone();

        let name = self.parse_identifier()?;
        let value = self.parse_field_value_with_mode(NumberParseMode::Sequence, "field value")?;

        Ok(NodeField::new(name, value, self.span_from(&start_span)))
    }

    fn parse_field_value(&mut self) -> Result<FieldValue> {
        self.parse_field_value_with_mode(NumberParseMode::Sequence, "field value")
    }

    fn parse_field_value_with_mode(
        &mut self,
        number_mode: NumberParseMode,
        context: &str,
    ) -> Result<FieldValue> {
        if self.is_number() {
            return match number_mode {
                NumberParseMode::Sequence => self.parse_number_sequence(),
                NumberParseMode::Single => Ok(self.parse_single_number()),
            };
        }

        match &self.current_token.kind {
            TokenKind::True => {
                self.advance();
                Ok(FieldValue::Bool(true))
            }
            TokenKind::False => {
                self.advance();
                Ok(FieldValue::Bool(false))
            }
            TokenKind::Int(_, _) | TokenKind::Float(_, _) => match number_mode {
                NumberParseMode::Sequence => self.parse_number_sequence(),
                NumberParseMode::Single => Ok(self.parse_single_number()),
            },
            TokenKind::Str(value) => {
                let value = value.clone();
                self.advance();
                Ok(FieldValue::String(value))
            }
            TokenKind::Null => {
                self.advance();
                Ok(FieldValue::Null)
            }
            TokenKind::Is => {
                self.advance();
                let name = self.parse_identifier()?;
                Ok(FieldValue::Is(name))
            }
            TokenKind::OpenBracket => {
                self.advance();
                Ok(FieldValue::Array(self.parse_array_value()?))
            }
            TokenKind::Identifier(_) | TokenKind::Def | TokenKind::Use => {
                let node = self.parse_node()?;
                Ok(FieldValue::Node(Box::new(node)))
            }
            TokenKind::Template { .. } => {
                Ok(FieldValue::Template(self.parse_template_block(context)?))
            }
            TokenKind::Proto
            | TokenKind::ExternProto
            | TokenKind::Field
            | TokenKind::VrmlField
            | TokenKind::HiddenField
            | TokenKind::DeprecatedField
            | TokenKind::Whitespace(_)
            | TokenKind::Comment(_)
            | TokenKind::Newline(_)
            | TokenKind::OpenBrace
            | TokenKind::CloseBrace
            | TokenKind::CloseBracket
            | TokenKind::Comma
            | TokenKind::Eof
            | TokenKind::Unknown(_) => Err(self.error(format!(
                "Unexpected token in {}: {:?}",
                context, self.current_token.kind
            ))),
        }
    }

    fn parse_array_value(&mut self) -> Result<ArrayValue> {
        let mut elements = Vec::new();

        while self.current_token.kind != TokenKind::CloseBracket
            && self.current_token.kind != TokenKind::Eof
        {
            elements.push(ArrayElement::new(self.parse_field_value_in_array()?));
        }

        self.expect(TokenKind::CloseBracket)?;

        Ok(ArrayValue::new().with_elements(elements))
    }

    fn parse_field_value_in_array(&mut self) -> Result<FieldValue> {
        self.parse_field_value_with_mode(NumberParseMode::Single, "array value")
    }

    fn parse_template_block(&mut self, context: &str) -> Result<TemplateBlock> {
        let token = self.current_token.clone();
        self.advance();
        if let TokenKind::Template {
            content,
            is_expression,
            terminated,
        } = token.kind
        {
            if !terminated {
                return Err(self.error(format!("Unterminated template block in {}", context)));
            }
            Ok(TemplateBlock::new(content, is_expression, token.span))
        } else {
            Err(self.error("Expected template token after template start".to_string()))
        }
    }

    fn is_number(&self) -> bool {
        matches!(
            self.current_token.kind,
            TokenKind::Int(_, _) | TokenKind::Float(_, _)
        )
    }

    fn parse_single_number(&mut self) -> FieldValue {
        let token = self.current_token.clone();
        self.advance();
        self.token_to_field_value(&token)
    }

    fn parse_number_sequence(&mut self) -> Result<FieldValue> {
        let first = self.current_token.clone();
        self.advance();

        let mut elements = vec![NumberSequenceElement::new(
            self.token_to_field_value(&first),
        )];

        while self.is_number() {
            elements.push(NumberSequenceElement::new(
                self.token_to_field_value(&self.current_token),
            ));
            self.advance();
        }

        Ok(FieldValue::NumberSequence(
            NumberSequence::new().with_elements(elements),
        ))
    }

    fn token_to_field_value(&self, token: &Token) -> FieldValue {
        if let TokenKind::Int(value, raw) = &token.kind {
            FieldValue::Int(*value, Some(raw.clone()))
        } else if let TokenKind::Float(value, raw) = &token.kind {
            FieldValue::Float(*value, Some(raw.clone()))
        } else {
            FieldValue::Raw(format!("{:?}", token.kind))
        }
    }

    fn token_text(token: &Token) -> String {
        match &token.kind {
            TokenKind::Identifier(value) => value.clone(),
            TokenKind::Def => "DEF".to_string(),
            TokenKind::Use => "USE".to_string(),
            TokenKind::Proto => "PROTO".to_string(),
            TokenKind::ExternProto => "EXTERNPROTO".to_string(),
            TokenKind::Field => "field".to_string(),
            TokenKind::VrmlField => "vrmlField".to_string(),
            TokenKind::HiddenField => "hiddenField".to_string(),
            TokenKind::DeprecatedField => "deprecatedField".to_string(),
            TokenKind::Is => "IS".to_string(),
            TokenKind::Null => "NULL".to_string(),
            TokenKind::True => "TRUE".to_string(),
            TokenKind::False => "FALSE".to_string(),
            TokenKind::Float(_, raw) | TokenKind::Int(_, raw) => raw.clone(),
            TokenKind::Str(value) => format!("\"{}\"", value.escape_default()),
            TokenKind::OpenBrace => "{".to_string(),
            TokenKind::CloseBrace => "}".to_string(),
            TokenKind::OpenBracket => "[".to_string(),
            TokenKind::CloseBracket => "]".to_string(),
            TokenKind::Comma => ",".to_string(),
            TokenKind::Whitespace(value)
            | TokenKind::Comment(value)
            | TokenKind::Newline(value) => value.clone(),
            TokenKind::Template {
                content,
                is_expression,
                terminated,
            } => {
                if *is_expression {
                    if *terminated {
                        format!("%<={}>%", content)
                    } else {
                        format!("%<={}", content)
                    }
                } else if *terminated {
                    format!("%<{}>%", content)
                } else {
                    format!("%<{}", content)
                }
            }
            TokenKind::Eof => String::new(),
            TokenKind::Unknown(value) => value.to_string(),
        }
    }

    fn error(&self, msg: String) -> Error {
        Error::Syntax {
            line: self.current_token.span.start_line,
            col: self.current_token.span.start_col,
            message: msg,
        }
    }

    fn expect(&mut self, expected: TokenKind) -> Result<()> {
        if Self::token_kind_matches(&self.current_token.kind, &expected) {
            self.advance();
            Ok(())
        } else {
            Err(self.error(format!(
                "Expected {:?}, got {:?}",
                expected, self.current_token.kind
            )))
        }
    }

    fn token_kind_matches(current: &TokenKind, expected: &TokenKind) -> bool {
        match (current, expected) {
            (TokenKind::Identifier(_), TokenKind::Identifier(_)) => true,
            (TokenKind::Str(_), TokenKind::Str(_)) => true,
            (TokenKind::Int(_, _), TokenKind::Int(_, _)) => true,
            (TokenKind::Float(_, _), TokenKind::Float(_, _)) => true,
            (TokenKind::Template { .. }, TokenKind::Template { .. }) => true,
            (TokenKind::Unknown(_), TokenKind::Unknown(_)) => true,
            _ => current == expected,
        }
    }

    fn parse_identifier(&mut self) -> Result<String> {
        if let TokenKind::Identifier(value) = &self.current_token.kind {
            let value = value.clone();
            self.advance();
            Ok(value)
        } else {
            Err(self.error(format!(
                "Expected Identifier, got {:?}",
                self.current_token.kind
            )))
        }
    }

    fn span_from(&self, start: &Span) -> Span {
        Span::new(
            start.start,
            self.prev_span.end,
            start.start_line,
            start.start_col,
            self.prev_span.end_line,
            self.prev_span.end_col,
        )
    }
}

#[derive(Copy, Clone)]
enum NumberParseMode {
    Sequence,
    Single,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_parse_pedestrian() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("../../fixtures/Pedestrian.proto");
        let content = fs::read_to_string(path).expect("Failed to read Pedestrian.proto");

        let mut parser = Parser::new(&content);
        let doc = parser
            .parse_document()
            .expect("Failed to parse Pedestrian.proto");

        assert!(doc.header.is_some());
        let header = doc.header.as_ref().unwrap();
        assert_eq!(header.version, "R2025a");

        assert!(!doc.externprotos.is_empty());
        assert_eq!(doc.externprotos[0].url, "PedestrianTorso.proto");

        let proto = doc.proto.as_ref().expect("Proto definition missing");
        assert_eq!(proto.name, "Pedestrian");

        let field = &proto.fields[0];
        assert_eq!(field.name, "translation");

        if let Some(FieldValue::NumberSequence(sequence)) = &field.default_value {
            assert_eq!(sequence.elements.len(), 3);
            if let FieldValue::Int(0, _) = sequence.elements[0].value {
            } else {
                panic!("Expected 0");
            }
            if let FieldValue::Int(0, _) = sequence.elements[1].value {
            } else {
                panic!("Expected 0");
            }
            if let FieldValue::Float(value, _) = sequence.elements[2].value {
                assert!((value - 1.27).abs() < 1e-6);
            } else {
                panic!("Expected 1.27");
            }
        } else {
            panic!(
                "Expected NumberSequence for translation, got {:?}",
                field.default_value
            );
        }

        let item = &proto.body[0];
        if let ProtoBodyItem::Template(template) = item {
            assert!(
                template
                    .content
                    .contains("const rigid = fields.controllerArgs.value.length == 0;")
            );
        } else {
            panic!("Expected template block as first body item");
        }
    }

    #[test]
    fn test_parse_constrained_sfstring_field() {
        let input = r#"#VRML_SIM R2025a utf8
PROTO Sample [
  field SFString{"adult", "kid"} size "adult"
] {
  Robot {
    name "example"
  }
}
"#;
        let mut parser = Parser::new(input);
        let doc = parser
            .parse_document()
            .expect("Failed to parse constrained SFString field");

        let proto = doc.proto.as_ref().expect("Proto definition missing");
        assert_eq!(proto.fields.len(), 1);
        assert_eq!(proto.fields[0].name, "size");
    }

    #[test]
    fn test_parse_externproto_webots_url() {
        let input = r#"#VRML_SIM R2025a utf8
EXTERNPROTO "webots://projects/appearances/protos/Grass.proto"

PROTO Sample [
  field SFString name "example"
] {
  Robot {
    name IS name
  }
}
"#;
        let mut parser = Parser::new(input);
        let doc = parser
            .parse_document()
            .expect("Failed to parse EXTERNPROTO with webots URL");

        assert_eq!(doc.externprotos.len(), 1);
        assert_eq!(
            doc.externprotos[0].url,
            "webots://projects/appearances/protos/Grass.proto"
        );
    }

    #[test]
    fn test_parse_world_after_externproto_block() {
        let input = r#"#VRML_SIM R2025a utf8

EXTERNPROTO "https://raw.githubusercontent.com/cyberbotics/webots/R2025a/projects/objects/floors/protos/RectangleArena.proto"
EXTERNPROTO "https://raw.githubusercontent.com/cyberbotics/webots/R2025a/projects/appearances/protos/Parquetry.proto"
EXTERNPROTO "https://raw.githubusercontent.com/cyberbotics/webots/R2025a/projects/objects/backgrounds/protos/TexturedBackground.proto"
EXTERNPROTO "https://raw.githubusercontent.com/cyberbotics/webots/R2025a/projects/objects/backgrounds/protos/TexturedBackgroundLight.proto"

WorldInfo {
  basicTimeStep 12
  contactProperties [
    ContactProperties {
      material1 "wheel_rubber"
      material2 "asphalt"
    }
  ]
}
Viewpoint {
  orientation 0 0 1 0
  position 0 0 1
}
"#;
        let mut parser = Parser::new(input);
        let doc = parser
            .parse_document()
            .expect("Failed to parse world after EXTERNPROTO block");

        assert_eq!(doc.externprotos.len(), 4);
        assert_eq!(doc.root_nodes.len(), 2);
        assert_eq!(
            doc.externprotos[3].url,
            "https://raw.githubusercontent.com/cyberbotics/webots/R2025a/projects/objects/backgrounds/protos/TexturedBackgroundLight.proto"
        );
    }
}
