use erased_set::ErasedSet;
use iai::black_box;

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

iai::main! {
    insert_1,
    insert_10,
    insert_100,
    get_1,
    get_10,
    get_100
}
