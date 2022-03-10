use std::net::Ipv4Addr;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ipv4_display::Ipv4AddrDisplay;

pub fn criterion_benchmark(c: &mut Criterion) {
    let addrs = [
        Ipv4Addr::new(23, 24, 25, 26),
        Ipv4Addr::new(213, 2, 33, 213),
    ];

    for addr in addrs {
        c.bench_function(&format!("std {addr}"), |b| {
            b.iter(|| black_box(addr).to_string())
        });

        c.bench_function(&format!("ipv4-display {addr}"), |b| {
            b.iter(|| black_box(Ipv4AddrDisplay::new(addr)).to_string())
        });

        #[cfg(feature = "ufmt")]
        c.bench_function(&format!("ufmt {addr}"), |b| {
            b.iter(|| black_box(Ipv4AddrDisplay::new(addr)).to_string_ufmt())
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
