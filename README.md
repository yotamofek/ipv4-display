A more performant implementation of [`fmt::Display`] (and [`fmt::Debug`])
for the standard library's [`Ipv4Addr`] struct.

## Benchmarks

Rough benchmarks (using Rust 1.59.0) show almost 3x improvements:

```bash
$ cargo version
cargo 1.59.0 (49d8809dc 2022-02-10)
$ cargo bench
std 23.24.25.26         time:   [101.75 ns 105.13 ns 108.69 ns]
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe

ipv4-display 23.24.25.26
                        time:   [35.706 ns 36.657 ns 37.710 ns]
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild

std 213.2.33.213        time:   [111.03 ns 113.84 ns 116.89 ns]
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high mild

ipv4-display 213.2.33.213
                        time:   [36.145 ns 37.188 ns 38.307 ns]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
```
