use std::hint::black_box;
use std::time::Duration;

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use vrl::{compiler, stdlib};

fn make_assignment_chain(statements: usize) -> String {
    let mut source = String::with_capacity(statements * 24);
    for i in 0..statements {
        source.push_str(&format!(".field_{i} = {i}\n"));
    }
    source
}

fn make_branch_merge_program(branches: usize) -> String {
    let mut source = String::with_capacity(branches * 88);
    for i in 0..branches {
        source.push_str(&format!(
            "if true {{ .branch_{i}.left = {i} }} else {{ .branch_{i}.right = {i} }}\n"
        ));
    }
    source
}

fn make_delete_miss_program(deletes: usize) -> String {
    let mut source = String::with_capacity(deletes * 28 + 16);
    source.push_str(".known = 1\n");
    for i in 0..deletes {
        source.push_str(&format!("del(.missing_{i})\n"));
    }
    source
}

fn bench_compile_assignment_chain(c: &mut Criterion) {
    let fns = stdlib::all();
    let mut group = c.benchmark_group("vrl_compiler/typestate/assignment_chain");

    for size in [100_usize, 1_000, 5_000] {
        let source = make_assignment_chain(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &source, |b, src| {
            b.iter(|| {
                let result = compiler::compile(black_box(src), &fns)
                    .expect("assignment-chain source should compile");
                black_box(result.program);
            })
        });
    }
}

fn bench_compile_branch_merge(c: &mut Criterion) {
    let fns = stdlib::all();
    let mut group = c.benchmark_group("vrl_compiler/typestate/branch_merge");

    for size in [50_usize, 500, 2_000] {
        let source = make_branch_merge_program(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &source, |b, src| {
            b.iter(|| {
                let result =
                    compiler::compile(black_box(src), &fns).expect("branch-merge source should compile");
                black_box(result.program);
            })
        });
    }
}

fn bench_compile_delete_miss(c: &mut Criterion) {
    let fns = stdlib::all();
    let mut group = c.benchmark_group("vrl_compiler/typestate/delete_miss");

    for size in [100_usize, 1_000, 5_000] {
        let source = make_delete_miss_program(size);
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &source, |b, src| {
            b.iter(|| {
                let result =
                    compiler::compile(black_box(src), &fns).expect("delete-miss source should compile");
                black_box(result.program);
            })
        });
    }
}

criterion_group!(
    name = compiler_typestate_benches;
    config = Criterion::default()
        .warm_up_time(Duration::from_secs(3))
        .measurement_time(Duration::from_secs(15))
        .sample_size(40)
        .noise_threshold(0.05);
    targets =
        bench_compile_assignment_chain,
        bench_compile_branch_merge,
        bench_compile_delete_miss
);
criterion_main!(compiler_typestate_benches);
