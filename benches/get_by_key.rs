use std::collections::{BTreeMap, HashMap};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use contains_bench::map::{PicoSortedMap, SmallSortedMap}; 

fn setup_maps<const N: usize>() -> (Vec<u64>, HashMap<u64, usize>, BTreeMap<u64, usize>, SmallSortedMap<u64, usize>, PicoSortedMap<u64, usize, N>) {
    let mut hash_map: HashMap<u64, _> = HashMap::with_capacity(N);
    let mut btree_map: BTreeMap<u64, _> = BTreeMap::new();

    let mut keys = Vec::with_capacity(N);
    let mut values = Vec::with_capacity(N);
    for i in 0..(N as u64) {
        let k = 4096 - (i * 2);
        let v = (i * 2 - 1) as usize;
        keys.push(k);
        values.push(v);

        hash_map.insert(k, v);
        btree_map.insert(k, v);
    }

    let small_map = SmallSortedMap::from_vecs(keys.clone(), values.clone());
    let pico_map = PicoSortedMap::from_vecs(keys.clone(), values.clone());

    (keys, hash_map, btree_map, small_map, pico_map)
}


// Define a macro to handle each size
macro_rules! bench_for_size {
    ($c:expr, $group:expr, $size:literal) => {
        {
            const SIZE: usize = $size;
            let (mut keys, hash_map, btree_map, small_map, pico_map) = setup_maps::<SIZE>();
            let keys_slice: &mut [u64] = keys.as_mut_slice();
            keys_slice.sort_by(|a, b| a.cmp(b).reverse());
            let keys_slice: &[u64] = keys_slice;

            $group.bench_function(format!("hash_map_get_{}", SIZE), |b| {
                b.iter(|| {
                    for k in keys_slice {
                        black_box(hash_map.get(k));
                    }
                    for k in MAX..MAX_PLUS32 {
                        black_box(hash_map.get(&k));
                    }
                })
            });

            $group.bench_function(format!("btree_map_get_{}", SIZE), |b| {
                b.iter(|| {
                    for k in keys_slice {
                        black_box(btree_map.get(k));
                    }
                    for k in MAX..MAX_PLUS32 {
                        black_box(btree_map.get(&k));
                    }
                })
            });

            $group.bench_function(format!("small_sorted_map_get_{}", SIZE), |b| {
                b.iter(|| {
                    for k in keys_slice {
                        black_box(small_map.get(k));
                    }
                    for k in MAX..MAX_PLUS32 {
                        black_box(small_map.get(&k));
                    }
                })
            });

            $group.bench_function(format!("pico_sorted_map_get_{}", SIZE), |b| {
                b.iter(|| {
                    for k in keys_slice {
                        black_box(pico_map.get(k));
                    }
                    for k in MAX..MAX_PLUS32 {
                        black_box(pico_map.get(&k));
                    }
                })
            });
        }
    };
}

fn bench_get_by_key_parameterized(c: &mut Criterion) {
    const MAX: u64 = 256;
    const MAX_PLUS32: u64 = MAX + 32;

    let mut group = c.benchmark_group("get_by_key_by_size");
    
    // Call the macro for each size you want to benchmark
    bench_for_size!(c, group, 4);
    bench_for_size!(c, group, 8);
    bench_for_size!(c, group, 12);
    bench_for_size!(c, group, 16);
    bench_for_size!(c, group, 24);
    bench_for_size!(c, group, 32);
    bench_for_size!(c, group, 48);
    bench_for_size!(c, group, 64);
    bench_for_size!(c, group, 92);
    bench_for_size!(c, group, 128);
    bench_for_size!(c, group, 192);
    bench_for_size!(c, group, 256);

    group.finish();
}

criterion_group!(benches, bench_get_by_key_parameterized);
criterion_main!(benches);