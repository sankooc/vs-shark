use criterion::{criterion_group, criterion_main, Criterion};
// use pcap::common::Instance;
// use util::FileBatchReader;

fn bench_batch(c: &mut Criterion) {
    
    c.bench_function("parse_perform", |_b: &mut criterion::Bencher<'_>| {
        // let fname = "../../../pcaps/11.pcapng";

        // let batch_size = 1024 * 1024 * 4;
        // let mut batcher = FileBatchReader::new(fname.to_string(), batch_size);
        // let mut ins = Instance::new(batch_size as usize);
        // let count = batcher.count();
        // let mut list = vec![];
        // for _ in 0..count {
        //     list.push(batcher.read().unwrap().1);
        // }
        // b.iter(|| {
        //     for data in &list {
        //         ins.update(data.clone()).unwrap();
        //     }
        // });
        // // let setup = || batcher.read();
        // // let routine = |rs: Result<(u64, Vec<u8>), std::io::Error>| {
        // //     if let Ok((left, data)) = rs {
        // //         ins.update(data).unwrap();
        // //         return left == 0;
        // //     }
        // //     return true
        // // };
        
        // // b.iter_batched(setup, routine, BatchSize::NumBatches(count));
    });
}

criterion_group!(benches, bench_batch);

criterion_main!(benches);
