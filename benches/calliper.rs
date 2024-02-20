use calliper::{utils::black_box, Runner, Scenario, ScenarioConfig};
use erased_set::ErasedSet;

#[no_mangle]
#[inline(never)]
fn insert_1() {
    let mut set = ErasedSet::new();

    struct A;
    set.insert(A);

    black_box(set);
}

#[no_mangle]
#[inline(never)]
fn insert_10() {
    let mut set = ErasedSet::new();

    for _ in 0..10 {
        struct A;
        set.insert(A);
    }

    black_box(set);
}

#[no_mangle]
#[inline(never)]
fn insert_100() {
    let mut set = ErasedSet::new();

    for _ in 0..100 {
        struct A;
        set.insert(A);
    }

    black_box(set);
}

#[no_mangle]
#[inline(never)]
fn get_1() {
    let mut set = ErasedSet::new();

    struct A;
    set.insert(A);
    black_box(set.get::<A>());
}

#[no_mangle]
#[inline(never)]
fn get_10() {
    let mut set = ErasedSet::new();

    for _ in 0..10 {
        struct A;
        set.insert(A);
        black_box(set.get::<A>());
    }
}

#[no_mangle]
#[inline(never)]
fn get_100() {
    let mut set = ErasedSet::new();

    for _ in 0..100 {
        struct A;
        set.insert(A);
        black_box(set.get::<A>());
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let benches = [
        Scenario::new(insert_1),
        Scenario::new(insert_10),
        Scenario::new(insert_100),
        Scenario::new(get_1),
        Scenario::new(get_10),
        Scenario::new(get_100),
    ];

    let runner =
        Runner::default().config(ScenarioConfig::default().branch_sim(true).collect_bus(true));

    if let Some(results) = runner.run(&benches)? {
        for res in results.into_iter() {
            println!("{}", res.parse());
        }
    }

    Ok(())
}
