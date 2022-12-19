you should see similar result like below:

```
running 19 tests
test bench_serialize_serde_arrays                                      ... bench:         940 ns/iter (+/- 123)
test bench_serialize_serde_arrays_normal                               ... bench:     629,628 ns/iter (+/- 75,473)
test bench_serialize_serde_as                                          ... bench:         966 ns/iter (+/- 655)
test bench_serialize_serde_as_normal                                   ... bench:     613,004 ns/iter (+/- 516,977)
test bench_serialize_vanilla                                           ... bench:         959 ns/iter (+/- 442)
test serde_as_bytes::bench_deserialize_from_serde_bytes_normal         ... bench:     133,168 ns/iter (+/- 51,316)
test serde_as_bytes::bench_deserialize_serde_bytes_normal              ... bench:     125,591 ns/iter (+/- 42,278)
test serde_as_bytes::bench_serialize_serde_as_bytes_normal             ... bench:      57,250 ns/iter (+/- 5,816)
test serde_bytes_cow::bench_deserialize_from_serde_bytes_normal        ... bench:     180,030 ns/iter (+/- 87,294)
test serde_bytes_cow::bench_deserialize_serde_bytes_normal             ... bench:     129,263 ns/iter (+/- 13,909)
test serde_bytes_cow::bench_serialize_serde_bytes_normal               ... bench:      57,515 ns/iter (+/- 4,154)
test serde_bytes_slice::bench_deserialize_from_serde_bytes_normal      ... FAILED
test serde_bytes_slice::bench_deserialize_serde_bytes_normal           ... bench:     121,329 ns/iter (+/- 43,854)
test serde_bytes_slice::bench_serialize_serde_bytes_normal             ... bench:      55,998 ns/iter (+/- 4,472)
test serde_bytes_slice_json::bench_deserialize_from_serde_bytes_normal ... FAILED
test serde_bytes_slice_json::bench_deserialize_serde_bytes_normal      ... FAILED
test serde_bytes_vec::bench_deserialize_from_serde_bytes_normal        ... bench:     208,701 ns/iter (+/- 15,006)
test serde_bytes_vec::bench_deserialize_serde_bytes_normal             ... bench:     194,379 ns/iter (+/- 27,701)
test serde_bytes_vec::bench_serialize_serde_bytes_normal               ... bench:      56,212 ns/iter (+/- 2,228)
```

and errors:

```
---- serde_bytes_slice::bench_deserialize_from_serde_bytes_normal stdout ----
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Custom("invalid type: byte array, expected a borrowed byte array")', src/main.rs:528:70
stack backtrace:
...
```

