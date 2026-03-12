use criterion::{Criterion, criterion_group, criterion_main};
use webots_proto_ast::Proto;

fn criterion_benchmark(c: &mut Criterion) {
    let input = r#"#VRML_SIM R2025a utf8
PROTO MyRobot [
  field SFVec3f translation 0 0 0
  field SFString name "robot"
]
{
  Robot {
    translation IS translation
    name IS name
    children [
        Shape {
            geometry Box { size 0.1 0.1 0.1 }
        }
    ]
  }
}
"#;
    let doc: Proto = input.parse().unwrap();
    c.bench_function("write_lossless_simple", |b| {
        b.iter(|| doc.to_lossless_string())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
