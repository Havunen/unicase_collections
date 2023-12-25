use criterion::{black_box, criterion_group, criterion_main, Criterion};
use unicase::UniCase;
use unicase_collections::unicase_btree_map::UniCaseBTreeMap;

fn unicase_btree_map_benchmark(c: &mut Criterion) {
    let mut map = UniCaseBTreeMap::new();
    for i in 0..1000 {
        map.insert(UniCase::new(i.to_string()), i);
    }

    c.bench_function("UniCaseBTreeMap get str", |b| {
        b.iter(|| {
            for _i in 0..1000 {
                let _ = map.get(black_box("dffd"));
            }
        })
    });

    c.bench_function("UniCaseBTreeMap get String", |b| {
        b.iter(|| {
            for i in 0..1000 {
                let _ = map.get(black_box(i.to_string()));
            }
        })
    });

    c.bench_function("UniCaseBTreeMap get new UniCase", |b| {
        b.iter(|| {
            for i in 0..1000 {
                let _ = map.get(black_box(UniCase::new(i.to_string())));
            }
        })
    });

    c.bench_function("UniCaseBTreeMap get &UniCase", |b| {
        b.iter(|| {
            for i in 0..1000 {
                let _ = map.get(black_box(&UniCase::new(i.to_string())));
            }
        })
    });
}

criterion_group!(benches, unicase_btree_map_benchmark);
criterion_main!(benches);
