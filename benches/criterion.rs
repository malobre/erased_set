use criterion::{black_box, criterion_group, criterion_main, Criterion};
use erased_set::ErasedSet;

fn insert_1() {
    let mut set = ErasedSet::new();

    struct A;
    set.insert(A);

    black_box(set);
}

fn insert_10() {
    let mut set = ErasedSet::new();

    for _ in 0..10 {
        struct A;
        set.insert(A);
    }

    black_box(set);
}

fn insert_100() {
    let mut set = ErasedSet::new();

    for _ in 0..100 {
        struct A;
        set.insert(A);
    }

    black_box(set);
}

pub fn insert_bench(c: &mut Criterion) {
    c.bench_function("insert 1", |b| b.iter(insert_1));
    c.bench_function("insert 10", |b| b.iter(insert_10));
    c.bench_function("insert 100", |b| b.iter(insert_100));
}

fn get_1() {
    let mut set = ErasedSet::new();

    struct A;
    set.insert(A);
    black_box(set.get::<A>());
}

fn get_10() {
    let mut set = ErasedSet::new();

    for _ in 0..10 {
        struct A;
        set.insert(A);
        black_box(set.get::<A>());
    }
}

fn get_100() {
    let mut set = ErasedSet::new();

    for _ in 0..100 {
        struct A;
        set.insert(A);
        black_box(set.get::<A>());
    }
}

pub fn get_bench(c: &mut Criterion) {
    c.bench_function("get 1", |b| b.iter(get_1));
    c.bench_function("get 10", |b| b.iter(get_10));
    c.bench_function("get 100", |b| b.iter(get_100));
}

criterion_group!(insert, insert_bench);
criterion_group!(get, get_bench);
criterion_main!(insert, get);
