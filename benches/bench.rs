#![allow(unused)]
use std::collections::BTreeSet;
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use byteset::ByteSet;

pub fn bench_collect(c: &mut Criterion) {
    let mut group = c.benchmark_group("collect");
    let range = &(0..=255);
    group.bench_function("ByteSet", |b| b.iter(|| {
        range.clone().collect::<ByteSet>()
    }));
    group.bench_function("BTreeSet", |b| b.iter(|| {
        range.clone().collect::<BTreeSet<u8>>()
    }));
    group.finish();
}

pub fn bench_contains(c: &mut Criterion) {
    let mut group = c.benchmark_group("contains");
    let set: ByteSet = (0..=150).collect();
    group.bench_function("ByteSet", |b| b.iter(|| {
        set.contains(42)
    }));
    let set: BTreeSet<u8> = (0..=150).collect();
    group.bench_function("BTreeSet", |b| b.iter(|| {
        set.contains(&42)
    }));
    group.finish();
}

pub fn bench_iter_long(c: &mut Criterion) {
    let mut group = c.benchmark_group("iter_long");
    let set: ByteSet = (0..=255).collect();
    group.bench_function("ByteSet", |b| b.iter(|| {
        set.iter().count()
    }));
    let set: BTreeSet<u8> = (0..=255).collect();
    group.bench_function("BTreeSet", |b| b.iter(|| {
        set.iter().count()
    }));
    group.finish();
}

pub fn bench_iter_short(c: &mut Criterion) {
    let mut group = c.benchmark_group("iter_short");
    let set: ByteSet = (0..10).collect();
    group.bench_function("ByteSet", |b| b.iter(|| {
        set.iter().count()
    }));
    let set: BTreeSet<u8> = (0..10).collect();
    group.bench_function("BTreeSet", |b| b.iter(|| {
        set.iter().count()
    }));
    group.finish();
}

pub fn bench_intersection(c: &mut Criterion) {
    let set1: ByteSet = (20..60).collect();
    let set2: ByteSet = (40..80).collect();
    let mut group = c.benchmark_group("intersection");
    group.bench_function("ByteSet", |b| b.iter(|| {
        set1.intersection(&set2)
    }));
    let set1: BTreeSet<u8> = (20..60).collect();
    let set2: BTreeSet<u8> = (40..80).collect();
    group.bench_function("BTreeSet", |b| b.iter(|| {
        set1.intersection(&set2)
    }));
    group.finish();
}

pub fn bench_union(c: &mut Criterion) {
    let set1: ByteSet = (20..60).collect();
    let set2: ByteSet = (40..80).collect();
    let mut group = c.benchmark_group("union");
    group.bench_function("ByteSet", |b| b.iter(|| {
        set1.union(&set2)
    }));
    let set1: BTreeSet<u8> = (20..60).collect();
    let set2: BTreeSet<u8> = (40..80).collect();
    group.bench_function("BTreeSet", |b| b.iter(|| {
        set1.union(&set2)
    }));
    group.finish();
}

pub fn bench_is_subset(c: &mut Criterion) {
    let set1: ByteSet = (20..60).collect();
    let set2: ByteSet = (10..70).collect();
    let mut group = c.benchmark_group("is_subset");
    group.bench_function("ByteSet", |b| b.iter(|| {
        set1.is_subset(&set2)
    }));
    let set1: BTreeSet<u8> = (20..60).collect();
    let set2: BTreeSet<u8> = (10..70).collect();
    group.bench_function("BTreeSet", |b| b.iter(|| {
        set1.is_subset(&set2)
    }));
    group.finish();
}

criterion_group!(
    benches,
    bench_collect,
    bench_contains,
    bench_iter_long,
    bench_iter_short,
    bench_intersection,
    bench_union,
    bench_is_subset,
);
criterion_main!(benches);
