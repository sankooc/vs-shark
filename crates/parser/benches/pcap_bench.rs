use criterion::{criterion_group, criterion_main, Criterion};
use pcap::common::Instance;

fn bench_parse_pcap(c: &mut Criterion) {
    let data = std::fs::read("../../../pcaps/11.pcapng").unwrap();
    c.bench_function("parse_pcap", |b| {
        b.iter(|| {
            let mut instance = Instance::new(65536);
            instance.update(data.clone()).unwrap();
        })
    });
}

criterion_group!(benches, bench_parse_pcap);
criterion_main!(benches);