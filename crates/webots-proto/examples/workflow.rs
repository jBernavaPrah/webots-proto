use std::collections::HashMap;
use std::error::Error;
use webots_proto::{
    Proto, ProtoExt, RenderContext, RenderOptions, RenderWebotsVersion, TemplateField,
};

fn main() -> Result<(), Box<dyn Error>> {
    let input = r#"#VRML_SIM R2025a utf8

PROTO ContextAwareBox [
  field SFInt32 count 2
  field SFString label "default-label"
]
{
  Group {
    children [
      WorldInfo { title "%<= fields.label.value >%|%<= fields.label.defaultValue >%|%<= context.os >%" }
      %< for (let i = 0; i < fields.count.value; ++i) { >%
        Transform {
          translation %<= i >% 0 0
          children [ Box { size 0.5 0.5 0.5 } ]
        }
      %< } >%
    ]
  }
}
"#;

    let document: Proto = input.parse()?;
    let diagnostics = document.validate()?;
    println!("diagnostics: {}", diagnostics.len());

    let mut field_overrides = HashMap::new();
    field_overrides.insert("count".to_string(), TemplateField::SFInt32(3));
    field_overrides.insert(
        "label".to_string(),
        TemplateField::SFString("override-label".to_string()),
    );

    let context = RenderContext::default()
        .with_os("linux")
        .with_world("/workspace/worlds/example.wbt")
        .with_webots_version(RenderWebotsVersion::new(
            "R2025a".to_string(),
            "0".to_string(),
        ));

    let options = RenderOptions::default()
        .with_field_overrides(field_overrides)
        .with_context(context);

    let rendered = document.render(&options)?;
    println!("{rendered}");

    Ok(())
}
