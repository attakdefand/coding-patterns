//! Benchmarks for timing side-channel protection

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use timing_sidechannel_protection::{
    secure_array_search, secure_sorted_intersection, secure_string_compare, secure_two_sum,
};

fn bench_secure_two_sum(c: &mut Criterion) {
    let nums: Vec<i32> = (1..=1000).collect();

    c.bench_function("secure_two_sum_small", |b| {
        b.iter(|| secure_two_sum(black_box(&nums[..10]), black_box(15)))
    });

    c.bench_function("secure_two_sum_medium", |b| {
        b.iter(|| secure_two_sum(black_box(&nums[..100]), black_box(150)))
    });

    c.bench_function("secure_two_sum_large", |b| {
        b.iter(|| secure_two_sum(black_box(&nums[..1000]), black_box(1500)))
    });
}

fn bench_secure_string_compare(c: &mut Criterion) {
    let short_str1 = "hello";
    let short_str2 = "world";
    let short_str3 = "hello";

    let long_str1 = "a".repeat(1000);
    let long_str2 = "b".repeat(1000);
    let long_str3 = "a".repeat(1000);

    let mut group = c.benchmark_group("secure_string_compare");

    group.bench_function("short_different", |b| {
        b.iter(|| secure_string_compare(black_box(short_str1), black_box(short_str2)))
    });

    group.bench_function("short_same", |b| {
        b.iter(|| secure_string_compare(black_box(short_str1), black_box(short_str3)))
    });

    group.bench_function("long_different", |b| {
        b.iter(|| secure_string_compare(black_box(&long_str1), black_box(&long_str2)))
    });

    group.bench_function("long_same", |b| {
        b.iter(|| secure_string_compare(black_box(&long_str1), black_box(&long_str3)))
    });

    group.finish();
}

fn bench_secure_array_search(c: &mut Criterion) {
    let arr: Vec<i32> = (1..=1000).collect();

    c.bench_function("secure_array_search_beginning", |b| {
        b.iter(|| secure_array_search(black_box(&arr), black_box(1)))
    });

    c.bench_function("secure_array_search_middle", |b| {
        b.iter(|| secure_array_search(black_box(&arr), black_box(500)))
    });

    c.bench_function("secure_array_search_end", |b| {
        b.iter(|| secure_array_search(black_box(&arr), black_box(1000)))
    });
}

fn bench_secure_sorted_intersection(c: &mut Criterion) {
    let arr1: Vec<i32> = (1..=500).collect();
    let arr2: Vec<i32> = (250..=750).collect();

    c.bench_function("secure_sorted_intersection", |b| {
        b.iter(|| secure_sorted_intersection(black_box(&arr1), black_box(&arr2)))
    });
}

criterion_group!(
    benches,
    bench_secure_two_sum,
    bench_secure_string_compare,
    bench_secure_array_search,
    bench_secure_sorted_intersection
);

criterion_main!(benches);
