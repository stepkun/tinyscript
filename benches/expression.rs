// Copyright © 2025 Stephan Kunz
#![allow(missing_docs)]

//! Benchmarks of scripting expressions

use std::time::Duration;

use criterion::{Criterion, criterion_group, criterion_main};
use tinyscript::{environment::DefaultEnvironment, Runtime, SHOULD_NOT_HAPPEN};

const SAMPLES: usize = 100;
const ITERATIONS: usize = 100;
const DURATION: Duration = Duration::from_secs(5);

fn expression(c: &mut Criterion) {
	let mut group = c.benchmark_group("expression");
	group
		.measurement_time(DURATION)
		.sample_size(SAMPLES);

	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();

	let chunk = runtime
		.parse("(3 + 2) * (4 - 1);")
		.expect(SHOULD_NOT_HAPPEN);
	group.bench_function("simple", |b| {
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
		.parse("!(5 - 4 > 3 * 2 == !nil);")
		.expect(SHOULD_NOT_HAPPEN);
	group.bench_function("moderate", |b| {
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
		.parse("'this is a ' + 'test string';")
		.expect(SHOULD_NOT_HAPPEN);
	group.bench_function("strings", |b| {
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

criterion_group!(benches, expression);

criterion_main!(benches);
