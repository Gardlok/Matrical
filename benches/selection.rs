use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use matrical::{execute_read, Cog, Matrix, Region, Shape, SumGear};
use ndarray::{s, Array2};
use std::hint::black_box;
use std::mem::size_of;
use std::time::Duration;

#[derive(Clone, Copy)]
struct Selection {
    name: &'static str,
    start_row: usize,
    end_row: usize,
    start_column: usize,
    end_column: usize,
}

impl Selection {
    fn len(self) -> usize {
        (self.end_row - self.start_row) * (self.end_column - self.start_column)
    }

    fn region(self, shape: Shape) -> Region {
        Region::new(
            shape,
            self.start_row..self.end_row,
            self.start_column..self.end_column,
        )
        .expect("benchmark selection must be valid")
    }
}

fn selections(rows: usize, columns: usize) -> [Selection; 5] {
    let middle_row = rows / 2;
    let middle_column = columns / 2;
    let small_start_row = middle_row.saturating_sub(2).min(rows - 4);
    let small_start_column = middle_column.saturating_sub(2).min(columns - 4);

    [
        Selection {
            name: "full",
            start_row: 0,
            end_row: rows,
            start_column: 0,
            end_column: columns,
        },
        Selection {
            name: "large_interior",
            start_row: rows / 4,
            end_row: rows - rows / 4,
            start_column: columns / 4,
            end_column: columns - columns / 4,
        },
        Selection {
            name: "single_row",
            start_row: middle_row,
            end_row: middle_row + 1,
            start_column: 0,
            end_column: columns,
        },
        Selection {
            name: "single_column",
            start_row: 0,
            end_row: rows,
            start_column: middle_column,
            end_column: middle_column + 1,
        },
        Selection {
            name: "small_4x4",
            start_row: small_start_row,
            end_row: small_start_row + 4,
            start_column: small_start_column,
            end_column: small_start_column + 4,
        },
    ]
}

fn values(len: usize) -> Vec<f64> {
    (0..len)
        .map(|index| (index % 1024) as f64 * 0.25)
        .collect()
}

fn bench_read_traversal(c: &mut Criterion) {
    for (rows, columns) in [(32, 24), (1_024, 64), (100_000, 64)] {
        let shape = Shape::new(rows, columns).expect("benchmark shape must be valid");
        let source = values(shape.len());

        for selection in selections(rows, columns) {
            let region = selection.region(shape);
            let group_name = format!("{rows}x{columns}/{}", selection.name);
            let mut group = c.benchmark_group(group_name);
            group.throughput(Throughput::Elements(selection.len() as u64));

            let direct = Array2::from_shape_vec((rows, columns), source.clone())
                .expect("ndarray benchmark source must fit");
            let direct_view = direct.slice(s![
                selection.start_row..selection.end_row,
                selection.start_column..selection.end_column
            ]);
            group.bench_function("direct_ndarray_sum", |b| {
                b.iter(|| {
                    let sum: f64 = direct_view.iter().copied().sum();
                    black_box(sum)
                })
            });

            let matrix = Matrix::from_row_major(shape, source.clone())
                .expect("Matrical benchmark source must fit");
            let lens = matrix.lens(region).expect("benchmark Lens must be valid");
            group.bench_function("lens_sum", |b| {
                b.iter(|| {
                    let sum: f64 = lens.iter().copied().sum();
                    black_box(sum)
                })
            });

            let cog = Cog::new(());
            group.bench_function("gear_sum", |b| {
                b.iter(|| {
                    let report =
                        execute_read(&SumGear, &lens, &cog, Vec::new()).expect("Gear must succeed");
                    black_box(*report.output())
                })
            });

            group.finish();
        }
    }
}

fn bench_lens_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("lens_creation");

    for (rows, columns) in [(32, 24), (1_024, 64), (100_000, 64)] {
        let shape = Shape::new(rows, columns).expect("benchmark shape must be valid");
        let selection = selections(rows, columns)[4];
        let region = selection.region(shape);
        let source = values(shape.len());

        let matrix = Matrix::from_row_major(shape, source.clone())
            .expect("Matrical benchmark source must fit");
        group.bench_with_input(
            BenchmarkId::new("lens", format!("{rows}x{columns}/small_4x4")),
            &(),
            |b, _| {
                b.iter(|| {
                    let lens = matrix.lens(region).expect("benchmark Lens must be valid");
                    black_box(lens.len())
                })
            },
        );

        let mut matrix_mut = Matrix::from_row_major(shape, source)
            .expect("Matrical benchmark source must fit");
        group.bench_with_input(
            BenchmarkId::new("lens_mut", format!("{rows}x{columns}/small_4x4")),
            &(),
            |b, _| {
                b.iter(|| {
                    let lens = matrix_mut
                        .lens_mut(region)
                        .expect("benchmark LensMut must be valid");
                    black_box(lens.len())
                })
            },
        );
    }

    group.finish();
}

fn bench_copy(c: &mut Criterion) {
    for (rows, columns) in [(32, 24), (1_024, 64), (100_000, 64)] {
        let shape = Shape::new(rows, columns).expect("benchmark shape must be valid");
        let source = values(shape.len());

        for selection in [selections(rows, columns)[0], selections(rows, columns)[4]] {
            let region = selection.region(shape);
            let group_name = format!("copy/{rows}x{columns}/{}", selection.name);
            let mut group = c.benchmark_group(group_name);
            group.throughput(Throughput::Bytes(
                (selection.len() * size_of::<f64>()) as u64,
            ));

            let matrix = Matrix::from_row_major(shape, source.clone())
                .expect("Matrical benchmark source must fit");
            let lens = matrix.lens(region).expect("benchmark Lens must be valid");
            group.bench_function("lens_to_row_major", |b| {
                b.iter(|| black_box(lens.to_row_major()))
            });

            let mut matrix_mut = Matrix::from_row_major(shape, source.clone())
                .expect("Matrical benchmark source must fit");
            let lens_mut = matrix_mut
                .lens_mut(region)
                .expect("benchmark LensMut must be valid");
            group.bench_function("lens_mut_to_row_major", |b| {
                b.iter(|| black_box(lens_mut.to_row_major()))
            });

            group.finish();
        }
    }
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .sample_size(10)
        .warm_up_time(Duration::from_millis(500))
        .measurement_time(Duration::from_secs(2));
    targets = bench_read_traversal, bench_lens_creation, bench_copy
}
criterion_main!(benches);
