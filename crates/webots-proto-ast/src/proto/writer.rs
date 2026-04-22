use super::ProtoError;
use super::ast::*;
use std::fmt;

pub struct ProtoWriter;

#[derive(Copy, Clone)]
enum CanonicalNodeLayout {
    Inline,
    Block,
}

impl Default for ProtoWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl ProtoWriter {
    /// Creates a new ProtoWriter.
    pub fn new() -> Self {
        Self
    }

    /// Writes the document to a string preserving original formatting.
    ///
    /// If `source_content` is unavailable, this falls back to canonical output.
    pub fn write_lossless(&self, doc: &Proto) -> Result<String, ProtoError> {
        Ok(doc
            .source_content
            .clone()
            .unwrap_or_else(|| self.write_canonical(doc)))
    }

    /// Writes the document to a string using a canonical style.
    pub fn write_canonical(&self, doc: &Proto) -> String {
        let mut out = String::new();
        self.write_document_canonical(&mut out, doc).unwrap();
        out
    }

    pub fn write_document_canonical(&self, out: &mut dyn fmt::Write, doc: &Proto) -> fmt::Result {
        if let Some(header) = &doc.header {
            writeln!(out, "#VRML_SIM {} {}", header.version, header.encoding)?;
        }

        for (index, externproto) in doc.externprotos.iter().enumerate() {
            self.write_extern_proto_canonical(out, externproto)?;
            if index + 1 < doc.externprotos.len()
                || doc.proto.is_some()
                || !doc.root_nodes.is_empty()
            {
                writeln!(out)?;
            }
        }

        if let Some(proto) = &doc.proto {
            self.write_proto_definition_canonical(out, proto, 0)?;
            if !doc.root_nodes.is_empty() {
                writeln!(out)?;
            }
        }

        for (index, node) in doc.root_nodes.iter().enumerate() {
            self.write_node_canonical(out, node, 0, CanonicalNodeLayout::Block)?;
            if index + 1 < doc.root_nodes.len() {
                writeln!(out)?;
            }
        }

        Ok(())
    }

    fn write_extern_proto_canonical(
        &self,
        out: &mut dyn fmt::Write,
        externproto: &ExternProto,
    ) -> fmt::Result {
        write!(out, "EXTERNPROTO \"{}\"", externproto.url)?;
        if let Some(alias) = &externproto.alias {
            write!(out, " {}", alias)?;
        }
        Ok(())
    }

    fn write_proto_definition_canonical(
        &self,
        out: &mut dyn fmt::Write,
        proto: &ProtoDefinition,
        indent_level: usize,
    ) -> fmt::Result {
        self.write_indent(out, indent_level)?;
        writeln!(out, "PROTO {} [", proto.name)?;
        for field in &proto.fields {
            self.write_proto_field_canonical(out, field, indent_level + 1)?;
        }
        self.write_indent(out, indent_level)?;
        writeln!(out, "]")?;

        self.write_indent(out, indent_level)?;
        writeln!(out, "{{")?;
        for item in &proto.body {
            match item {
                ProtoBodyItem::Node(node) => {
                    self.write_node_canonical(
                        out,
                        node,
                        indent_level + 1,
                        CanonicalNodeLayout::Block,
                    )?;
                    writeln!(out)?;
                }
                ProtoBodyItem::Template(template) => {
                    self.write_indent(out, indent_level + 1)?;
                    self.write_template_canonical(out, template)?;
                    writeln!(out)?;
                }
            }
        }
        self.write_indent(out, indent_level)?;
        write!(out, "}}")?;
        Ok(())
    }

    fn write_proto_field_canonical(
        &self,
        out: &mut dyn fmt::Write,
        field: &ProtoField,
        indent_level: usize,
    ) -> fmt::Result {
        self.write_indent(out, indent_level)?;
        let keyword = match field.keyword {
            FieldKeyword::Field => "field",
            FieldKeyword::VrmlField => "vrmlField",
            FieldKeyword::HiddenField => "hiddenField",
            FieldKeyword::DeprecatedField => "deprecatedField",
        };
        write!(
            out,
            "{} {} {}",
            keyword,
            self.field_type_name(&field.field_type),
            field.name
        )?;

        if let Some(restrictions) = &field.restrictions {
            write!(out, " {{ ")?;
            for (index, element) in restrictions.iter().enumerate() {
                if index > 0 {
                    write!(out, ", ")?;
                }
                self.write_field_value_canonical(out, element, indent_level)?;
            }
            write!(out, " }}")?;
        }

        if let Some(value) = &field.default_value {
            write!(out, " ")?;
            self.write_field_value_canonical(out, value, indent_level)?;
        }
        writeln!(out)?;
        Ok(())
    }

    fn write_node_canonical(
        &self,
        out: &mut dyn fmt::Write,
        node: &AstNode,
        indent_level: usize,
        layout: CanonicalNodeLayout,
    ) -> fmt::Result {
        match &node.kind {
            AstNodeKind::Use { use_name } => {
                self.write_indent(out, indent_level)?;
                write!(out, "USE {}", use_name)?;
            }
            AstNodeKind::Node {
                type_name,
                def_name,
                fields,
            } => match layout {
                CanonicalNodeLayout::Inline => {
                    if let Some(def) = def_name {
                        write!(out, "DEF {} ", def)?;
                    }
                    write!(out, "{} {{", type_name)?;
                    for element in fields {
                        match element {
                            NodeBodyElement::Field(field) => {
                                write!(out, " {} ", field.name)?;
                                self.write_field_value_canonical(out, &field.value, indent_level)?;
                            }
                            NodeBodyElement::Template(template) => {
                                write!(out, " ")?;
                                self.write_template_canonical(out, template)?;
                            }
                            NodeBodyElement::Raw(raw) => {
                                write!(out, " ")?;
                                write!(out, "{}", raw.text)?;
                            }
                        }
                    }
                    write!(out, " }}")?;
                }
                CanonicalNodeLayout::Block => {
                    self.write_indent(out, indent_level)?;
                    if let Some(def) = def_name {
                        write!(out, "DEF {} ", def)?;
                    }
                    writeln!(out, "{} {{", type_name)?;
                    for element in fields {
                        match element {
                            NodeBodyElement::Field(field) => {
                                self.write_indent(out, indent_level + 1)?;
                                write!(out, "{} ", field.name)?;
                                self.write_field_value_canonical(
                                    out,
                                    &field.value,
                                    indent_level + 1,
                                )?;
                                writeln!(out)?;
                            }
                            NodeBodyElement::Template(template) => {
                                self.write_indent(out, indent_level + 1)?;
                                self.write_template_canonical(out, template)?;
                                writeln!(out)?;
                            }
                            NodeBodyElement::Raw(raw) => {
                                self.write_indent(out, indent_level + 1)?;
                                write!(out, "{}", raw.text)?;
                                writeln!(out)?;
                            }
                        }
                    }
                    self.write_indent(out, indent_level)?;
                    write!(out, "}}")?;
                }
            },
        }
        Ok(())
    }

    fn write_field_value_canonical(
        &self,
        out: &mut dyn fmt::Write,
        value: &FieldValue,
        indent_level: usize,
    ) -> fmt::Result {
        match value {
            FieldValue::Bool(value) => write!(out, "{}", if *value { "TRUE" } else { "FALSE" })?,
            FieldValue::Int(value, _) => write!(out, "{}", value)?,
            FieldValue::Float(value, _) => write!(out, "{}", value)?,
            FieldValue::String(value) => write!(out, "\"{}\"", value.escape_default())?,
            FieldValue::Vec2f(value) => write!(out, "{} {}", value[0], value[1])?,
            FieldValue::Vec3f(value) => write!(out, "{} {} {}", value[0], value[1], value[2])?,
            FieldValue::Rotation(value) => {
                write!(out, "{} {} {} {}", value[0], value[1], value[2], value[3])?
            }
            FieldValue::Color(value) => write!(out, "{} {} {}", value[0], value[1], value[2])?,
            FieldValue::Node(node) => {
                self.write_node_canonical(out, node, indent_level, CanonicalNodeLayout::Inline)?
            }
            FieldValue::Array(array) => self.write_array_canonical(out, array, indent_level)?,
            FieldValue::NumberSequence(sequence) => {
                for (index, element) in sequence.elements.iter().enumerate() {
                    if index > 0 {
                        write!(out, " ")?;
                    }
                    self.write_field_value_canonical(out, &element.value, indent_level)?;
                }
            }
            FieldValue::Is(value) => write!(out, "IS {}", value)?,
            FieldValue::Null => write!(out, "NULL")?,
            FieldValue::Template(template) => self.write_template_canonical(out, template)?,
            FieldValue::Raw(value) => write!(out, "{}", value)?,
        }
        Ok(())
    }

    fn write_array_canonical(
        &self,
        out: &mut dyn fmt::Write,
        array: &ArrayValue,
        indent_level: usize,
    ) -> fmt::Result {
        if array.elements.is_empty() {
            write!(out, "[]")?;
            return Ok(());
        }

        let multiline = array
            .elements
            .iter()
            .any(|element| matches!(element.value, FieldValue::Node(_) | FieldValue::Template(_)));

        if multiline {
            writeln!(out, "[")?;
            for element in &array.elements {
                self.write_array_element_canonical(out, element, indent_level + 1)?;
                writeln!(out)?;
            }
            self.write_indent(out, indent_level)?;
            write!(out, "]")?;
        } else {
            write!(out, "[")?;
            for (index, element) in array.elements.iter().enumerate() {
                if index > 0 {
                    write!(out, ", ")?;
                }
                self.write_field_value_canonical(out, &element.value, indent_level)?;
            }
            write!(out, "]")?;
        }
        Ok(())
    }

    fn write_array_element_canonical(
        &self,
        out: &mut dyn fmt::Write,
        element: &ArrayElement,
        indent_level: usize,
    ) -> fmt::Result {
        if let FieldValue::Node(node) = &element.value {
            self.write_node_canonical(out, node, indent_level, CanonicalNodeLayout::Block)
        } else if let FieldValue::Template(template) = &element.value {
            self.write_indent(out, indent_level)?;
            self.write_template_canonical(out, template)
        } else {
            self.write_indent(out, indent_level)?;
            self.write_field_value_canonical(out, &element.value, indent_level)
        }
    }

    fn write_template_canonical(
        &self,
        out: &mut dyn fmt::Write,
        template: &TemplateBlock,
    ) -> fmt::Result {
        if template.is_expression {
            write!(out, "%<={}>%", template.content)?;
        } else {
            write!(out, "%<{}>%", template.content)?;
        }
        Ok(())
    }

    fn field_type_name(&self, field_type: &FieldType) -> String {
        if let FieldType::Unknown(raw) = field_type {
            raw.clone()
        } else {
            format!("{:?}", field_type)
        }
    }

    fn write_indent(&self, out: &mut dyn fmt::Write, indent_level: usize) -> fmt::Result {
        for _ in 0..indent_level {
            write!(out, "  ")?;
        }
        Ok(())
    }

    pub fn write_node(&self, out: &mut dyn fmt::Write, node: &AstNode) -> fmt::Result {
        self.write_node_canonical(out, node, 0, CanonicalNodeLayout::Block)
    }

    pub fn write_template(
        &self,
        out: &mut dyn fmt::Write,
        template: &TemplateBlock,
    ) -> fmt::Result {
        self.write_template_canonical(out, template)
    }
}
