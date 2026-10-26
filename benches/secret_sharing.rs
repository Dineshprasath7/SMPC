use criterion::{Criterion, black_box, criterion_group, criterion_main};

use smpc::maths::{reconstruct_secret, split_secret};

fn bench_split_secret(c: &mut Criterion) {
    let secret = vec![0u8; 32];

    // 3 shares minimum are needed for reconsturct
    c.bench_function("split_secret_32_bytes_3_of_5", |b| {
        b.iter(|| {
            split_secret(black_box(&secret), black_box(5), black_box(3));
        });
    });
}

fn bench_reconstruct_secret(c: &mut Criterion) {
    let secret = vec![0u8; 32];
    let shares = split_secret(&secret, 5, 3);

    c.bench_function("reconstruct_secret_32_bytes_3_of_5", |b| {
        b.iter(|| {
            reconstruct_secret(black_box(&shares[..5]));
        });
    });
}

criterion_group!(benches, bench_split_secret, bench_reconstruct_secret);

criterion_main!(benches);
