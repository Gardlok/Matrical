use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use matrical::{execute_mut, AddScalarGear, Cog, Matrix, Region, ScalarPolicy, Shape};
use ndarray::{s, Array2};
use std::hint::black_box;
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

fn bench_mutation(c: &mut Criterion) {
    const ADDEND: f64 = 0.125;

    for (rows, columns) in [(32, 24), (1_024, 64), (100_000, 64)] {
        let shape = Shape::new(rows, columns).expect("benchmark shape must be valid");
        let source = values(shape.len());

        for selection in selections(rows, columns) {
            let region = selection.region(shape);
            let group_name = format!("{rows}x{columns}/{}", selection.name);
            let mut group = c.benchmark_group(group_name);
            group.throughput(Throughput::Elements(selection.len() as u64));

            let mut direct = Array2::from_shape_vec((rows, columns), source.clone())
                .expect("ndarray benchmark source must fit");
            let mut direct_view = direct.slice_mut(s![
                selection.start_row..selection.end_row,
                selection.start_column..selection.end_column
            ]);
            group.bench_function("direct_ndarray_add_scalar", |b| {
                b.iter(|| {
                    let mut affected = 0usize;
                    for value in direct_view.iter_mut() {
                        *value += ADDEND;
                        affected += 1;
                    }
                    black_box(&direct_view);
                    black_box(affected)
                })
            });

            let mut matrix = Matrix::from_row_major(shape, source.clone())
                .expect("Matrical benchmark source must fit");
            let mut lens = matrix
                .lens_mut(region)
                .expect("benchmark LensMut must be valid");
            group.bench_function("lens_mut_add_scalar", |b| {
                b.iter(|| {
                    let mut affected = 0usize;
                    for value in lens.iter_mut() {
                        *value += ADDEND;
                        affected += 1;
                    }
                    black_box(&lens);
                    black_box(affected)
                })
            });

            let mut gear_matrix = Matrix::from_row_major(shape, source.clone())
                .expect("Matrical benchmark source must fit");
            let mut gear_lens = gear_matrix
                .lens_mut(region)
                .expect("benchmark LensMut must be valid");
            let cog = Cog::new(ScalarPolicy::new(ADDEND));
            group.bench_function("gear_add_scalar", |b| {
                b.iter(|| {
                    let report = execute_mut(&AddScalarGear, &mut gear_lens, &cog, Vec::new())
                        .expect("Gear must succeed");
                    black_box(&gear_lens);
                    black_box(*report.output())
                })
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
    targets = bench_mutation
}
criterion_main!(benches);
