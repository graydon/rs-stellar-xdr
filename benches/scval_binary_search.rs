//! Benchmark: binary search over a sorted Vec of ScVals (eager vs lazy).
//!
//! Builds a sorted vector of ~1000 ScVals including U32s, Symbols, Bytes,
//! and Maps (ScMap full of ScMapEntry values). Benchmarks binary search
//! in both the eager (fully deserialized) and lazy (XDR-backed) representations.

use bencher::{benchmark_group, benchmark_main, Bencher};
use std::sync::Arc;

use stellar_xdr::{LazyScVal, Limits, ReadXdr, ScMap, ScMapEntry, ScSymbol, ScVal, WriteXdr};

// ---------------------------------------------------------------------------
// Data generation helpers
// ---------------------------------------------------------------------------

/// Build a deterministic, sorted `Vec<ScVal>` with a mix of types including maps.
fn build_sorted_scvals(n: usize) -> Vec<ScVal> {
    let mut vals = Vec::with_capacity(n);
    for i in 0..n {
        let val = match i % 5 {
            0 => ScVal::U32(i as u32 * 7),
            1 => ScVal::I32(i as i32 * 3),
            2 => ScVal::Symbol(ScSymbol(format!("sym_{i:05}").try_into().unwrap())),
            3 => {
                let b = (i as u32).to_be_bytes();
                ScVal::Bytes(stellar_xdr::ScBytes(b.to_vec().try_into().unwrap()))
            }
            4 => {
                let entries: Vec<ScMapEntry> = (0..3)
                    .map(|j| ScMapEntry {
                        key: ScVal::U32((i * 10 + j) as u32),
                        val: ScVal::U32((i * 10 + j + 1000) as u32),
                    })
                    .collect();
                ScVal::Map(Some(ScMap(entries.try_into().unwrap())))
            }
            _ => unreachable!(),
        };
        vals.push(val);
    }
    vals.sort();
    vals.dedup();
    vals
}

/// Serialize each ScVal to XDR bytes.
fn to_xdr_bytes(vals: &[ScVal]) -> Vec<Vec<u8>> {
    vals.iter()
        .map(|v| v.to_xdr(Limits::none()).unwrap())
        .collect()
}

/// Convert eager ScVals to lazy ScVals (each in its own Arc buffer).
fn to_lazy_scvals(eager: &[ScVal]) -> Vec<LazyScVal> {
    eager
        .iter()
        .map(|v| {
            let bytes = v.to_xdr(Limits::none()).unwrap();
            let arc: Arc<[u8]> = bytes.into();
            LazyScVal::try_from(arc).unwrap()
        })
        .collect()
}

/// Pick deterministic search targets at evenly-spaced positions.
fn pick_targets(vals: &[ScVal], count: usize) -> Vec<ScVal> {
    let step = vals.len().max(1) / count.max(1);
    (0..count).map(|i| vals[i * step].clone()).collect()
}

// Pre-computed test data shared across benchmarks via lazy_static-style init.
// The `bencher` crate calls each function many times, so we cache the setup.
struct TestData {
    eager_vals: Vec<ScVal>,
    lazy_vals: Vec<LazyScVal>,
    targets: Vec<ScVal>,
    lazy_targets: Vec<LazyScVal>,
    xdr_bytes: Vec<Vec<u8>>,
    target_bytes: Vec<Vec<u8>>,
}

fn test_data() -> &'static TestData {
    use std::sync::OnceLock;
    static DATA: OnceLock<TestData> = OnceLock::new();
    DATA.get_or_init(|| {
        let eager_vals = build_sorted_scvals(1_000);
        let lazy_vals = to_lazy_scvals(&eager_vals);
        let targets = pick_targets(&eager_vals, 20);
        let lazy_targets = to_lazy_scvals(&targets);
        let xdr_bytes = to_xdr_bytes(&eager_vals);
        let target_bytes = to_xdr_bytes(&targets);
        TestData {
            eager_vals,
            lazy_vals,
            targets,
            lazy_targets,
            xdr_bytes,
            target_bytes,
        }
    })
}

// ---------------------------------------------------------------------------
// Benchmarks: binary search on pre-built sorted vectors
// ---------------------------------------------------------------------------

/// Binary search 20 targets in a sorted Vec<ScVal> (eager, already in memory).
fn eager_binary_search(bench: &mut Bencher) {
    let d = test_data();
    bench.iter(|| {
        for t in &d.targets {
            let _ = d.eager_vals.binary_search(t);
        }
    });
}

/// Binary search 20 targets in a sorted Vec<LazyScVal> (lazy, byte-backed).
fn lazy_binary_search(bench: &mut Bencher) {
    let d = test_data();
    bench.iter(|| {
        for t in &d.lazy_targets {
            let _ = d.lazy_vals.binary_search(t);
        }
    });
}

// ---------------------------------------------------------------------------
// Benchmarks: parse from XDR bytes, sort, then binary search
// ---------------------------------------------------------------------------

/// Parse 1000 ScVals from XDR, sort, then search 20 targets (eager path).
fn eager_parse_sort_search(bench: &mut Bencher) {
    let d = test_data();
    bench.iter(|| {
        let mut parsed: Vec<ScVal> = d
            .xdr_bytes
            .iter()
            .map(|b| ScVal::from_xdr(b.as_slice(), Limits::none()).unwrap())
            .collect();
        parsed.sort();
        for tb in &d.target_bytes {
            let target = ScVal::from_xdr(tb.as_slice(), Limits::none()).unwrap();
            let _ = parsed.binary_search(&target);
        }
    });
}

/// Wrap 1000 ScVals as lazy from XDR bytes, sort, then search 20 targets.
fn lazy_parse_sort_search(bench: &mut Bencher) {
    let d = test_data();
    bench.iter(|| {
        let mut wrapped: Vec<LazyScVal> = d
            .xdr_bytes
            .iter()
            .map(|b| {
                let arc: Arc<[u8]> = b.as_slice().into();
                LazyScVal::try_from(arc).unwrap()
            })
            .collect();
        wrapped.sort();
        for tb in &d.target_bytes {
            let arc: Arc<[u8]> = tb.as_slice().into();
            let target = LazyScVal::try_from(arc).unwrap();
            let _ = wrapped.binary_search(&target);
        }
    });
}

// ---------------------------------------------------------------------------
// Benchmarks: iterating ScMap entries
// ---------------------------------------------------------------------------

fn build_map_val() -> &'static (ScVal, LazyScVal) {
    use std::sync::OnceLock;
    static MAP: OnceLock<(ScVal, LazyScVal)> = OnceLock::new();
    MAP.get_or_init(|| {
        let entries: Vec<ScMapEntry> = (0..200)
            .map(|i| {
                let inner: Vec<ScMapEntry> = (0..5)
                    .map(|j| ScMapEntry {
                        key: ScVal::Symbol(ScSymbol(format!("k{j}").try_into().unwrap())),
                        val: ScVal::U64(i * 100 + j),
                    })
                    .collect();
                ScMapEntry {
                    key: ScVal::U32(i as u32),
                    val: ScVal::Map(Some(ScMap(inner.try_into().unwrap()))),
                }
            })
            .collect();
        let map_val = ScVal::Map(Some(ScMap(entries.try_into().unwrap())));
        let bytes = map_val.to_xdr(Limits::none()).unwrap();
        let lazy_val = LazyScVal::try_from(Arc::from(bytes.as_slice())).unwrap();
        (map_val, lazy_val)
    })
}

/// Iterate 200 ScMap entries, summing U32 keys (eager).
fn eager_map_iterate_keys(bench: &mut Bencher) {
    let (map_val, _) = build_map_val();
    bench.iter(|| {
        if let ScVal::Map(Some(ref map)) = map_val {
            let mut sum = 0u32;
            for entry in map.0.iter() {
                if let ScVal::U32(k) = &entry.key {
                    sum = sum.wrapping_add(*k);
                }
            }
            sum
        } else {
            0
        }
    });
}

/// Iterate 200 ScMap entries, summing U32 keys (lazy).
fn lazy_map_iterate_keys(bench: &mut Bencher) {
    let (_, lazy_val) = build_map_val();
    bench.iter(|| {
        if let Some(lazy_opt) = lazy_val.as_map() {
            if let Some(map) = lazy_opt.get() {
                let mut sum = 0u32;
                for entry in map.iter() {
                    let key = entry.key();
                    if let Some(k) = key.as_u32() {
                        sum = sum.wrapping_add(k);
                    }
                }
                return sum;
            }
        }
        0
    });
}

benchmark_group!(
    search,
    eager_binary_search,
    lazy_binary_search,
    eager_parse_sort_search,
    lazy_parse_sort_search
);
benchmark_group!(map_access, eager_map_iterate_keys, lazy_map_iterate_keys);
benchmark_main!(search, map_access);
