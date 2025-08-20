// use criterion::{criterion_group, criterion_main, Criterion};
// use pcap::common::Instance;

// fn bench_parse_pcap(c: &mut Criterion) {
//     let data = std::fs::read("../../../pcaps/11.pcapng").unwrap();
//     c.bench_function("parse_pcap", |b| {
//         b.iter(|| {
//             let mut instance = Instance::new(65536);
//             instance.update(data.clone()).unwrap();
//         })
//     });
// }

// criterion_group!(benches, bench_parse_pcap);
// criterion_main!(benches);

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn bench_fib(c: &mut Criterion) {
    c.bench_function("fib 10", |b| b.iter(|| fibonacci(black_box(10))));
}

criterion_group!(benches, bench_fib);
criterion_main!(benches);