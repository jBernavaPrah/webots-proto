use std::error::Error;
use webots_proto_ast::Proto;
use webots_proto_template::RenderOptions;

fn main() -> Result<(), Box<dyn Error>> {
    let input = r#"#VRML_SIM R2025a utf8

PROTO BoxGrid [
  field SFInt32 rows 2
  field SFInt32 cols 2
]
{
  Group {
    children [
      %< for (let i = 0; i < fields.rows.value; ++i) { >%
        %< for (let j = 0; j < fields.cols.value; ++j) { >%
          Transform {
            translation %<= i >% 0 %<= j >%
            children [ Box { size 0.5 0.5 0.5 } ]
          }
        %< } >%
      %< } >%
    ]
  }
}
"#;

    let document: Proto = input.parse()?;
    let rendered = webots_proto_template::render(&document, &RenderOptions::default())?;
    println!("{rendered}");

    Ok(())
}
