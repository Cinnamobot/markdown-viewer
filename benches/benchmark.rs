use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mdv::tui::ui::truncate_with_markers;

fn bench_truncate_short(c: &mut Criterion) {
    let text = "Hello ⟨INLINE_CODE⟩code⟨/INLINE_CODE⟩ world";
    c.bench_function("truncate_short", |b| {
        b.iter(|| truncate_with_markers(black_box(text), 20))
    });
}

fn bench_truncate_medium(c: &mut Criterion) {
    let text = "This is a test string with ⟨INLINE_CODE⟩inline code⟨/INLINE_CODE⟩ and more text that continues here.";
    c.bench_function("truncate_medium", |b| {
        b.iter(|| truncate_with_markers(black_box(text), 30))
    });
}

fn bench_truncate_long(c: &mut Criterion) {
    let text = "A".repeat(1000);
    c.bench_function("truncate_long_1000", |b| {
        b.iter(|| truncate_with_markers(black_box(&text), 500))
    });
}

fn bench_truncate_very_long(c: &mut Criterion) {
    let text = "A".repeat(10000);
    c.bench_function("truncate_long_10000", |b| {
        b.iter(|| truncate_with_markers(black_box(&text), 5000))
    });
}

fn bench_truncate_cjk(c: &mut Criterion) {
    let text = "日本語のテスト⟨INLINE_CODE⟩コード⟨/INLINE_CODE⟩文章";
    c.bench_function("truncate_cjk", |b| {
        b.iter(|| truncate_with_markers(black_box(text), 15))
    });
}

fn bench_truncate_no_markers(c: &mut Criterion) {
    let text = "This is a long text without any markers that should be truncated properly.";
    c.bench_function("truncate_no_markers", |b| {
        b.iter(|| truncate_with_markers(black_box(text), 30))
    });
}

criterion_group!(
    benches,
    bench_truncate_short,
    bench_truncate_medium,
    bench_truncate_long,
    bench_truncate_very_long,
    bench_truncate_cjk,
    bench_truncate_no_markers
);
criterion_main!(benches);
