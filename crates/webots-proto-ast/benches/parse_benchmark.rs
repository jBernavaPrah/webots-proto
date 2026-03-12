use criterion::{Criterion, black_box, criterion_group, criterion_main};
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

    c.bench_function("parse_simple_proto", |b| {
        b.iter(|| black_box(input.parse::<Proto>()))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
