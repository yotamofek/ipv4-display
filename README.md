A more performant implementation of [`fmt::Display`] (and [`fmt::Debug`])
for the standard library's [`Ipv4Addr`] struct.

## Benchmarks

Rough benchmarks (using Rust 1.59.0) show almost 3x improvements:

```bash
$ cargo version
cargo 1.59.0 (49d8809dc 2022-02-10)
$ RUSTFLAGS="-C target-cpu=broadwell" cargo bench --features ufmt
std 23.24.25.26         time:   [113.38 ns 115.94 ns 118.48 ns]
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) low mild
  5 (5.00%) high mild

ipv4-display 23.24.25.26
                        time:   [42.332 ns 43.791 ns 45.442 ns]
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe

ufmt 23.24.25.26        time:   [47.831 ns 49.307 ns 50.811 ns]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

std 213.2.33.213        time:   [117.14 ns 119.85 ns 122.56 ns]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

ipv4-display 213.2.33.213
                        time:   [34.021 ns 35.096 ns 36.313 ns]
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high mild

ufmt 213.2.33.213       time:   [42.231 ns 43.505 ns 44.739 ns]
```
