use std::collections::{BTreeMap, HashMap};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

// Modified setup_maps to be generic over key type K and a key generator closure.
fn setup_maps<K, F, const N: usize>(key_gen: F) -> (Vec<K>, Vec<K>, HashMap<K, usize>, BTreeMap<K, usize>)
where 
    F: Fn(u64) -> K, 
    K: Ord + Clone + Eq + std::hash::Hash
{
    let mut hash_map: HashMap<K, _> = HashMap::with_capacity(N);
    let mut btree_map: BTreeMap<K, _> = BTreeMap::new();

    let mut keys = Vec::with_capacity(N);
    let mut values = Vec::with_capacity(N);
    for i in 0..(N as u64) {
        let computed = 4096 - (i * 2);
        let k = key_gen(computed);
        let v = (i * 2 - 1) as usize;
        keys.push(k.clone());
        values.push(v);
        hash_map.insert(k.clone(), v);
        btree_map.insert(k, v);
    }

    // make sure the number of non-present keys we request is equal to present ones
    let non_present_keys = ((N as u64)..(N as u64 * 2))
        .map(key_gen)
        .collect::<Vec<K>>();

    // reverse the order we request the keys in later
    keys.sort_by(|a, b| a.cmp(b).reverse());

    (keys, non_present_keys, hash_map, btree_map)
}

fn type_name_helper<T>(_t: &T) -> &'static str {
    std::any::type_name::<T>()
}

// Modified bench_for_size macro to accept an extra key generator parameter.
macro_rules! bench_for_size {
    ($c:expr, $group:expr, $size:literal, $key_gen:expr) => {{
        const SIZE: usize = $size;
        let (keys, non_keys, hash_map, btree_map) = setup_maps::<_, _, SIZE>($key_gen);
        let keys_slice: & [ _ ] = keys.as_slice();
        let non_keys_slice: & [ _ ] = non_keys.as_slice();
        let keys_slice: &[ _ ] = keys_slice;

        // so we can guarantee unique type names for the benchmarks
        let type_name = type_name_helper(keys_slice.first().unwrap());

        $group.bench_function(format!("hash_map_get_{}_{}", SIZE, type_name), |b| {
            b.iter(|| {
                for k in keys_slice {
                    black_box(hash_map.get(k));
                }
                for k in non_keys_slice {
                    black_box(hash_map.get(k));
                }
            })
        });

        $group.bench_function(format!("btree_map_get_{}_{}", SIZE, type_name), |b| {
            b.iter(|| {
                for k in keys_slice {
                    black_box(btree_map.get(k));
                }
                for k in non_keys_slice {
                    black_box(btree_map.get(k));
                }
            })
        });
    }};
}

fn bench_get_by_key_parameterized(c: &mut Criterion) {
    let mut group = c.benchmark_group("get_by_key_by_size");
    
    // String key benches
    bench_for_size!(c, group, 8, |x| x.to_string());
    bench_for_size!(c, group, 16, |x| x.to_string());
    bench_for_size!(c, group, 32, |x| x.to_string());
    bench_for_size!(c, group, 64, |x| x.to_string());

    // &str key benches
    bench_for_size!(c, group, 8, |x| { let s = Box::leak(x.to_string().into_boxed_str()); &*s });
    bench_for_size!(c, group, 16, |x| { let s = Box::leak(x.to_string().into_boxed_str()); &*s });
    bench_for_size!(c, group, 32, |x| { let s = Box::leak(x.to_string().into_boxed_str()); &*s });
    bench_for_size!(c, group, 64, |x| { let s = Box::leak(x.to_string().into_boxed_str()); &*s });

    // u64 key benches
    bench_for_size!(c, group, 8, |x| x);
    bench_for_size!(c, group, 16, |x| x);
    bench_for_size!(c, group, 32, |x| x);
    bench_for_size!(c, group, 64, |x| x);

    group.finish();
}

criterion_group!(benches, bench_get_by_key_parameterized);
criterion_main!(benches);