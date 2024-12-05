use aoc::day_four;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let char_array = day_four::get_input();
    let width = char_array[0].len();
    let height = char_array.len();

    c.bench_function("part_one", |b| {
        b.iter(|| black_box(day_four::part_one(width, height, &char_array)))
    });

    c.bench_function("part_two", |b| {
        b.iter(|| black_box(day_four::part_two(width, height, &char_array)));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
