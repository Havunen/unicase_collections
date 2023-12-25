use criterion::{black_box, Criterion, criterion_group, criterion_main};
use unicase::UniCase;
use unicase_collections::unicase_btree_map::UniCaseBTreeMap;

fn unicase_btree_map_benchmark(c: &mut Criterion) {
    let mut map = UniCaseBTreeMap::new();
    for i in 0..1000 {
        map.insert(UniCase::new(i.to_string()), i);
    }

    c.bench_function("UniCaseBTreeMap insert", |b| {
        b.iter(|| {
            let mut map = UniCaseBTreeMap::new();
            for i in 0..1000 {
                map.insert(black_box(UniCase::new(i.to_string())), black_box(i));
            }
        })
    });

    c.bench_function("UniCaseBTreeMap get", |b| {
        b.iter(|| {
            for i in 0..1000 {
                let _ = map.get(black_box(UniCase::new(i.to_string())));
            }
        })
    });

    c.bench_function("UniCaseBTreeMap contains_key", |b| {
        b.iter(|| {
            for i in 0..1000 {
                let _ = map.contains_key(black_box(UniCase::new(i.to_string())));
            }
        })
    });
}

criterion_group!(benches, unicase_btree_map_benchmark);
criterion_main!(benches);
