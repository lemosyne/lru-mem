use std::time::Duration;

use criterion::Criterion;

use lru_mem::LruCache;

use rand::rngs::ThreadRng;

fn run_iter_benchmark(cache: &mut LruCache<u64, String>, keys: &[u64],
        _: &mut ThreadRng) {
    for ((k, v), expected_key) in cache.iter().zip(keys.iter()) {
        assert_eq!(expected_key, k);
        assert_eq!(crate::VALUE_LEN, v.len());
    }
}

pub(crate) fn iter_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("iter");
    group.sample_size(100).measurement_time(Duration::from_secs(60));

    for &size in crate::CONSTANT_TIME_SIZES {
        crate::bench_cache_function(
            &mut group, size, run_iter_benchmark);
    }
}