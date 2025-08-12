// Copyright © 2025 Stephan Kunz
#![allow(missing_docs)]

//! Benchmarks of scripting equality

use std::time::Duration;

use criterion::{Criterion, criterion_group, criterion_main};
use tinyscript::{DefaultEnvironment, Runtime, SHOULD_NOT_HAPPEN};

const SAMPLES: usize = 100;
const ITERATIONS: usize = 100;
const DURATION: Duration = Duration::from_secs(5);

fn equality(c: &mut Criterion) {
	let mut group = c.benchmark_group("equality");
	group
		.measurement_time(DURATION)
		.sample_size(SAMPLES);

	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();

	let chunk = runtime
		.parse("true==true; true==false; false==true; false==false;")
		.expect(SHOULD_NOT_HAPPEN);
	group.bench_function("boolean", |b| {
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
		.parse("1==1; 3.1475==4.99999; -3.00987654321234==-3.00987654321234; 3.00987654321234==4;")
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
		.parse("0x1==0x1; 0xFF321==0x56adf; -0x34==-0x34; 0xabcdef==0x1;")
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

	let chunk = runtime.parse("'short'=='short'; 'short'=='sho'; 'medium'=='this is a little bit longer'; 'this is a little bit longer'=='this is a little bit longer';").expect(SHOULD_NOT_HAPPEN);
	group.bench_function("string", |b| {
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
		.parse("'short'==true; 'short'==1; 'medium'==nil; 'this is a little bit longer'==0x15;")
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

criterion_group!(benches, equality,);

criterion_main!(benches);
