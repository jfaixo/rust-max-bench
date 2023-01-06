# Rust "max" benchmark

Quick sample benchmark to point out the fact that "max" or "min" functions on iterators are unable to leverage SIMD instructions in situations where it would be advantageous.

This is normal once you read the max_by implementation, but as an end-user, I would have expected otherwise.

Output on my computer:
```
❯ rustc -vV
rustc 1.68.0-nightly (388538fc9 2023-01-05)
binary: rustc
commit-hash: 388538fc963e07a94e3fc3ac8948627fd2d28d29
commit-date: 2023-01-05
host: x86_64-unknown-linux-gnu
release: 1.68.0-nightly
LLVM version: 15.0.6
❯ cargo bench
    Finished bench [optimized] target(s) in 0.00s
     Running unittests src/lib.rs (target/release/deps/rust_max_bench-a5d988f9520f9dde)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running benches/bench.rs (target/release/deps/bench-cf556ddbd1b864fb)

running 3 tests
test custom    ... bench:       8,052 ns/iter (+/- 385)
test itertools ... bench:      94,027 ns/iter (+/- 816)
test stdlib    ... bench:      94,477 ns/iter (+/- 1,545)

test result: ok. 0 passed; 0 failed; 0 ignored; 3 measured; 0 filtered out; finished in 2.40s
```