use std::collections::{BTreeSet, HashSet};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn setup_collections(size: usize) -> (HashSet<usize>, BTreeSet<usize>, Vec<usize>) {
    let range = 0..size;
    let hash: HashSet<_> = range.clone().collect();
    let btree: BTreeSet<_> = range.clone().collect();
    let mut vec: Vec<_> = range.collect();
    vec.sort();
    (hash, btree, vec)
}

fn bench_contains_parameterized(c: &mut Criterion) {
    let sizes = vec![2, 4, 8, 12, 16, 20, 24, 28, 32];
    
    let mut group = c.benchmark_group("contains_by_size");
    
    for size in sizes {
        let (hash, btree, vec) = setup_collections(size);
        
        group.bench_function(format!("hash_set_{}", size), |b| {
            b.iter(|| {
                // Test existing elements
                for i in 0..size {
                    black_box(hash.contains(&i));
                }
                // Test non-existing elements
                for i in size..(size*2) {
                    black_box(hash.contains(&i));
                }
            })
        });
        
        group.bench_function(format!("btree_set_{}", size), |b| {
            b.iter(|| {
                for i in 0..size {
                    black_box(btree.contains(&i));
                }
                for i in size..(size*2) {
                    black_box(btree.contains(&i));
                }
            })
        });
        
        group.bench_function(format!("vec_binary_search_{}", size), |b| {
            b.iter(|| {
                for i in 0..size {
                    black_box(vec.binary_search(&i).is_ok());
                }
                for i in size..(size*2) {
                    black_box(vec.binary_search(&i).is_ok());
                }
            })
        });

        group.bench_function(format!("vec_{}", size), |b| {
            b.iter(|| {
                for i in 0..size {
                    black_box(vec.contains(&i));
                }
                for i in size..(size*2) {
                    black_box(vec.contains(&i));
                }
            })
        });
    }
    
    group.finish();
}

criterion_group!(benches, bench_contains_parameterized);
criterion_main!(benches);