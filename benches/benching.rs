use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use benching::{
    dashmap_prealloc, dashmap_simple, dashmap_threaded,
    hashmap_simple, hashmap_prealloc, hashmap_threaded, hashmap_rwlock_read, hashmap_rwlock_write,
    hashset_simple, hashset_prealloc, btreemap_simple,
    vecdeque_simple, vecdeque_prealloc, vecdeque_simple_insert,
    vec_prealloc_insert, vec_prealloc_push, vec_simple_insert, vec_simple_push
};

const ITER: usize = 9000;

fn vec_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Vec");
    group.bench_function("vec_simple_push", |b| {
        b.iter(|| vec_simple_push(black_box(ITER)));
    });

    group.bench_function("vec_prealloc_push", |b| {
        b.iter(|| vec_prealloc_push(black_box(ITER)));
    });

    group.bench_function("vec_simple_insert", |b| {
        b.iter(|| vec_simple_insert(black_box(ITER)));
    });

    group.bench_function("vec_prealloc_insert", |b| {
        b.iter(|| vec_prealloc_insert(black_box(ITER)));
    });

    group.finish();
}

fn hashmap_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("HashMap");

    group.bench_function("hashmap_simple", |b| {
        b.iter(|| hashmap_simple(black_box(ITER)));
    });

    group.bench_function("hashmap_prealloc", |b|{
        b.iter(|| hashmap_prealloc(black_box(ITER)));
    });

    group.finish();
}

fn dashmap_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("DashMap");

    group.bench_function("dashmap_simple", |b|{
        b.iter(|| dashmap_simple(black_box(ITER)));
    });

    group.bench_function("dashmap_prealloc", |b|{
        b.iter(|| dashmap_prealloc(black_box(ITER)));
    });
     group.finish();
}

fn vecdeque_bench(c: &mut Criterion) {
    let  mut  group = c.benchmark_group("VecDeque");

    group.bench_function("vecdeque_simple", |b| {
        b.iter(|| vecdeque_simple(black_box(ITER)));
    });

    group.bench_function("vecdeque_prealloc", |b| {
        b.iter(|| vecdeque_prealloc(black_box(ITER)));
    });

    group.bench_function("vecdeque_simple_insert", |b| {
        b.iter(|| vecdeque_simple_insert(black_box(ITER)));
    });

    group.finish();
}

fn hashset_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("HashSet");

    group.bench_function("hashset_prealloc", |b| {
        b.iter(|| hashset_prealloc(black_box(ITER)));
    });

    group.bench_function("hashset_simple", |b| {
        b.iter(|| hashset_simple(black_box(ITER)));
    });

    group.finish();
}

fn rayon_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Rayon");

    group.bench_function("dashmap_threaded", |b| {
        b.iter(|| dashmap_threaded(black_box(ITER)));
    });

    group.bench_function("hashmap_threaded", |b|{
        b.iter(|| hashmap_threaded(black_box(ITER)));
    });

    group.bench_function("hashmap_rwlock_read", |b|{
        b.iter(|| hashmap_rwlock_read(black_box(ITER)));
    });

    group.bench_function("hashmap_rwlock_write", |b|{
        b.iter(|| hashmap_rwlock_write(black_box(ITER)));
    });

    group.finish();
}

fn btreemap_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("BTreeMap");

    group.bench_function("btreemap_simple", |b|{
        b.iter(|| btreemap_simple(black_box(ITER)));
    });

    group.finish();
}

criterion_group!(vec, vec_bench);
criterion_group!(vecdeque, vecdeque_bench);
criterion_group!(hashset, hashset_bench);
criterion_group!(hashmap, hashmap_bench);
criterion_group!(dashmap, dashmap_bench);
criterion_group!(rayon, rayon_bench);
criterion_group!(btreemap, btreemap_bench);
criterion_main!(vec, vecdeque, hashset, hashmap, dashmap, btreemap, rayon);