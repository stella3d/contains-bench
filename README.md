# microbenchmarks

This repo is for running paramaterized benchmarks to satisfy my curiousity.

Right now, it's primarily used to test the effect of using the `release-lto` profile (1 codegen unit & "thin" LTO enabled) vs regular `release`.

## `HashMap<K,V>::(get/contains_key)` bench 

to run this benchmark:

```sh
# establish performance baseline
cargo bench hash_map --profile release
# compare the LTO'd version with the baseline
cargo bench hash_map --profile release-lto
```
