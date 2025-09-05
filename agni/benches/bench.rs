use criterion::{Criterion, black_box, criterion_group, criterion_main};
use hashbrown::HashMap;
use rustc_hash::FxHasher;
use std::hash::{BuildHasherDefault, Hash, Hasher};

// Define our coordinate type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct CoordDerive {
    x: i32,
    y: i32,
}

// Same struct, but we'll implement Hash manually
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CoordCustom {
    x: i32,
    y: i32,
}

// A cheap and effective way to combine two integers for hashing.
// This is often faster than deriving, which calls hash on each field sequentially.
impl Hash for CoordCustom {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // This is a common mixing pattern. Wrapping ops prevent panics on overflow.
        let hash = self.x.wrapping_mul(1_679_776_009) ^ self.y;
        state.write_i32(hash);
    }
}

// Type Aliases for clarity
type StdMap<K, V> = std::collections::HashMap<K, V>;
type HbMap<K, V> = HashMap<K, V>;
type FxMap<K, V> = HashMap<K, V, BuildHasherDefault<FxHasher>>;

const MAP_SIZE: u32 = 10_000;

fn bench_hashmaps(c: &mut Criterion) {
    let mut group = c.benchmark_group("HashMap Benchmarks");
    group.sample_size(100); // More samples for more stable results
    group.warm_up_time(std::time::Duration::from_secs(1));

    // Pre-generate the keys and values to avoid timing the RNG
    let data: Vec<(i32, i32)> = (0..MAP_SIZE)
        .map(|i| {
            // Create somewhat realistic coordinate data that isn't perfectly linear
            let x = i as i32 % 1000;
            let y = (i as i32 / 1000) * 2;
            (x, y)
        })
        .collect();

    // -- Benchmark Insertion --

    group.bench_function("std HashMap insert", |b| {
        b.iter(|| {
            let mut map: StdMap<(i32, i32), i32> = StdMap::default();
            for &(x, y) in &data {
                map.insert(black_box((x, y)), black_box(0));
            }
            black_box(map);
        })
    });

    group.bench_function("hashbrown HashMap insert", |b| {
        b.iter(|| {
            let mut map: HbMap<(i32, i32), i32> = HbMap::default();
            for &(x, y) in &data {
                map.insert(black_box((x, y)), black_box(0));
            }
            black_box(map);
        })
    });

    group.bench_function("FX HashMap insert", |b| {
        b.iter(|| {
            let mut map: FxMap<(i32, i32), i32> = FxMap::default();
            for &(x, y) in &data {
                map.insert(black_box((x, y)), black_box(0));
            }
            black_box(map);
        })
    });

    group.bench_function("FX HashMap insert (CoordDerive)", |b| {
        b.iter(|| {
            let mut map: FxMap<CoordDerive, i32> = FxMap::default();
            for &(x, y) in &data {
                map.insert(black_box(CoordDerive { x, y }), black_box(0));
            }
            black_box(map);
        })
    });

    group.bench_function("FX HashMap insert (CoordCustom)", |b| {
        b.iter(|| {
            let mut map: FxMap<CoordCustom, i32> = FxMap::default();
            for &(x, y) in &data {
                map.insert(black_box(CoordCustom { x, y }), black_box(0));
            }
            black_box(map);
        })
    });

    // -- Benchmark Lookup --

    // Pre-fill the maps for lookup benchmarks
    let mut std_map: StdMap<(i32, i32), i32> = StdMap::default();
    let mut hb_map: HbMap<(i32, i32), i32> = HbMap::default();
    let mut fx_map: FxMap<(i32, i32), i32> = FxMap::default();
    let mut fx_map_derive: FxMap<CoordDerive, i32> = FxMap::default();
    let mut fx_map_custom: FxMap<CoordCustom, i32> = FxMap::default();

    for &(x, y) in &data {
        std_map.insert((x, y), 0);
        hb_map.insert((x, y), 0);
        fx_map.insert((x, y), 0);
        fx_map_derive.insert(CoordDerive { x, y }, 0);
        fx_map_custom.insert(CoordCustom { x, y }, 0);
    }

    // Lookup each key in the map
    group.bench_function("std HashMap lookup", |b| {
        b.iter(|| {
            for &(x, y) in &data {
                black_box(std_map.get(&black_box((x, y))));
            }
        })
    });

    group.bench_function("hashbrown HashMap lookup", |b| {
        b.iter(|| {
            for &(x, y) in &data {
                black_box(hb_map.get(&black_box((x, y))));
            }
        })
    });

    group.bench_function("FX HashMap lookup", |b| {
        b.iter(|| {
            for &(x, y) in &data {
                black_box(fx_map.get(&black_box((x, y))));
            }
        })
    });

    group.bench_function("FX HashMap lookup (CoordDerive)", |b| {
        b.iter(|| {
            for &(x, y) in &data {
                black_box(fx_map_derive.get(&black_box(CoordDerive { x, y })));
            }
        })
    });

    group.bench_function("FX HashMap lookup (CoordCustom)", |b| {
        b.iter(|| {
            for &(x, y) in &data {
                black_box(fx_map_custom.get(&black_box(CoordCustom { x, y })));
            }
        })
    });

    group.finish();
}

criterion_group!(benches, bench_hashmaps);
criterion_main!(benches);
