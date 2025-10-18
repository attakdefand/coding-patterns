//! Benchmarks for race condition protection

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use race_condition_protection::{
    concurrent_array_search, concurrent_sorted_intersection, concurrent_string_compare,
    concurrent_two_sum,
};
use std::sync::Arc;
use std::thread;

fn bench_concurrent_two_sum(c: &mut Criterion) {
    let nums: Vec<i32> = (1..=1000).collect();

    c.bench_function("concurrent_two_sum_small", |b| {
        b.iter(|| concurrent_two_sum(black_box(&nums[..10]), black_box(15)))
    });

    c.bench_function("concurrent_two_sum_medium", |b| {
        b.iter(|| concurrent_two_sum(black_box(&nums[..100]), black_box(150)))
    });

    c.bench_function("concurrent_two_sum_large", |b| {
        b.iter(|| concurrent_two_sum(black_box(&nums[..1000]), black_box(1500)))
    });
}

fn bench_concurrent_string_compare(c: &mut Criterion) {
    let short_str1 = "hello";
    let short_str2 = "world";
    let short_str3 = "hello";

    let long_str1 = "a".repeat(1000);
    let long_str2 = "b".repeat(1000);
    let long_str3 = "a".repeat(1000);

    let mut group = c.benchmark_group("concurrent_string_compare");

    group.bench_function("short_different", |b| {
        b.iter(|| concurrent_string_compare(black_box(short_str1), black_box(short_str2)))
    });

    group.bench_function("short_same", |b| {
        b.iter(|| concurrent_string_compare(black_box(short_str1), black_box(short_str3)))
    });

    group.bench_function("long_different", |b| {
        b.iter(|| concurrent_string_compare(black_box(&long_str1), black_box(&long_str2)))
    });

    group.bench_function("long_same", |b| {
        b.iter(|| concurrent_string_compare(black_box(&long_str1), black_box(&long_str3)))
    });

    group.finish();
}

fn bench_concurrent_array_search(c: &mut Criterion) {
    let arr: Vec<i32> = (1..=1000).collect();

    c.bench_function("concurrent_array_search_beginning", |b| {
        b.iter(|| concurrent_array_search(black_box(&arr), black_box(1)))
    });

    c.bench_function("concurrent_array_search_middle", |b| {
        b.iter(|| concurrent_array_search(black_box(&arr), black_box(500)))
    });

    c.bench_function("concurrent_array_search_end", |b| {
        b.iter(|| concurrent_array_search(black_box(&arr), black_box(1000)))
    });
}

fn bench_concurrent_sorted_intersection(c: &mut Criterion) {
    let arr1: Vec<i32> = (1..=500).collect();
    let arr2: Vec<i32> = (250..=750).collect();

    c.bench_function("concurrent_sorted_intersection", |b| {
        b.iter(|| concurrent_sorted_intersection(black_box(&arr1), black_box(&arr2)))
    });
}

fn bench_concurrent_access(c: &mut Criterion) {
    let nums = Arc::new([2, 7, 11, 15, 20, 25]);
    let target = 9;

    c.bench_function("concurrent_access_10_threads", |b| {
        b.iter(|| {
            let handles: Vec<_> = (0..10)
                .map(|_| {
                    let nums_clone = Arc::clone(&nums);
                    thread::spawn(move || concurrent_two_sum(&nums_clone, target))
                })
                .collect();

            // Collect all results
            let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
            black_box(results)
        })
    });
}

criterion_group!(
    benches,
    bench_concurrent_two_sum,
    bench_concurrent_string_compare,
    bench_concurrent_array_search,
    bench_concurrent_sorted_intersection,
    bench_concurrent_access
);

criterion_main!(benches);
