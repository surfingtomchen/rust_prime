use criterion::{Criterion, criterion_group, criterion_main};
use weiji::find_n_prime;

fn bench_simple(c: &mut Criterion) {

    let mut group = c.benchmark_group("My Group");

    // Now we can perform benchmarks with this group
    group.bench_function("Bench 1", |b| b.iter(|| find_n_prime(186647) ));

    group.finish();
}

criterion_group!(benches, bench_simple);
criterion_main!(benches);