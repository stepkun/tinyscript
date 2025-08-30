// Copyright © 2025 Stephan Kunz
#![allow(missing_docs)]

//! Benchmarks of scripting comparison

use std::time::Duration;

use criterion::{Criterion, criterion_group, criterion_main};
use tinyscript::{environment::DefaultEnvironment, Runtime, SHOULD_NOT_HAPPEN};

const SAMPLES: usize = 100;
const ITERATIONS: usize = 100;
const DURATION: Duration = Duration::from_secs(5);

fn comparison(c: &mut Criterion) {
	let mut group = c.benchmark_group("comparison");
	group
		.measurement_time(DURATION)
		.sample_size(SAMPLES);

	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();

	let chunk = runtime
		.parse("1<1; 3.1475<4.99999; -3.00987654321234>-3.00987654321234; 4>3.00987654321234;")
		.expect(SHOULD_NOT_HAPPEN);
	group.bench_function("double", |b| {
		b.iter(|| {
			for _ in 1..=ITERATIONS {
				runtime
					.execute(&chunk, &mut env)
					.expect(SHOULD_NOT_HAPPEN);
			}
			std::hint::black_box(());
		});
	});

	let chunk = runtime
		.parse("0x1<0x1; 0x1<0x2; 0x1>0x1; 0x2>0x1;")
		.expect(SHOULD_NOT_HAPPEN);
	group.bench_function("integer", |b| {
		b.iter(|| {
			for _ in 1..=ITERATIONS {
				runtime
					.execute(&chunk, &mut env)
					.expect(SHOULD_NOT_HAPPEN);
			}
			std::hint::black_box(());
		});
	});

	let chunk = runtime
		.parse("0x1<0x1; 3.1475<4.99999; 4>3.00987654321234; 0x2>1;")
		.expect(SHOULD_NOT_HAPPEN);
	group.bench_function("mixed", |b| {
		b.iter(|| {
			for _ in 1..=ITERATIONS {
				runtime
					.execute(&chunk, &mut env)
					.expect(SHOULD_NOT_HAPPEN);
			}
			std::hint::black_box(());
		});
	});
}

criterion_group!(benches, comparison,);

criterion_main!(benches);
