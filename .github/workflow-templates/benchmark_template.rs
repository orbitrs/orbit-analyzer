use criterion::{criterion_group, criterion_main, Criterion};

fn parse_benchmark(c: &mut Criterion) {
    c.bench_function("parse orbit file", |b| {
        b.iter(|| {
            let content = r#"
<template>
  <div>{{ label }}</div>
</template>
<script>
component Button {
  props {
    label: string;
  }
}
</script>
<style>
.button {}
</style>
"#;
            orbit_analyzer::parser::parse_orbit_file(content, "Button.orbit").unwrap()
        });
    });
}

criterion_group!(benches, parse_benchmark);
criterion_main!(benches);
