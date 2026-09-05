use matrical::prelude::*;

const ROWS: usize = 16;
const COLUMNS: usize = 40;
const BRIGHTEN_BY: f64 = 72.0;
const MIN_PIXEL: f64 = 0.0;
const MAX_PIXEL: f64 = 255.0;
const LUMINANCE_RAMP: &[u8] = b" .:-=+*#%@";

fn main() -> Result<(), MatricalError> {
    let shape = Shape::new(ROWS, COLUMNS)?;
    let region = Region::new(shape, 5..12, 10..30)?;
    let pixels = build_security_camera_frame(shape, region);
    let mut matrix = Matrix::from_row_major(shape, pixels)?;
    let before = matrix.iter().copied().collect::<Vec<_>>();
    let selected_element_count = region.rows() * region.columns();

    let before_report = {
        let selected = matrix.lens(region)?;
        execute_read(
            &SumGear,
            &selected,
            &Cog::new(()),
            demo_tags(TagStage::Input, 1),
        )?
    };
    let mean_before = *before_report.output() / selected_element_count as f64;

    println!("BEFORE");
    render_frame(&matrix)?;
    println!();
    println!("SELECTED REGION");
    println!(
        "rows {}..{}, columns {}..{} ({} x {}, {} pixels)",
        region.start_row(),
        region.end_row(),
        region.start_column(),
        region.end_column(),
        region.rows(),
        region.columns(),
        selected_element_count
    );
    println!("underexposed mean brightness: {mean_before:.1} / 255.0");
    println!();
    println!("TRANSFORMATION");
    println!("AddScalarGear: +{BRIGHTEN_BY:.1} inside the selected LensMut only");
    println!("ClampGear: constrain selected pixels to {MIN_PIXEL:.0}..={MAX_PIXEL:.0}");

    let brighten_report = {
        let mut selected = matrix.lens_mut(region)?;
        execute_mut(
            &AddScalarGear,
            &mut selected,
            &Cog::new(ScalarPolicy::new(BRIGHTEN_BY)),
            demo_tags(TagStage::Transform, 2),
        )?
    };

    let clamp_report = {
        let mut selected = matrix.lens_mut(region)?;
        execute_mut(
            &ClampGear,
            &mut selected,
            &Cog::new(ClampPolicy::new(MIN_PIXEL, MAX_PIXEL)),
            demo_tags(TagStage::Transform, 3),
        )?
    };

    let after_report = {
        let selected = matrix.lens(region)?;
        execute_read(
            &SumGear,
            &selected,
            &Cog::new(()),
            demo_tags(TagStage::Output, 4),
        )?
    };
    let mean_after = *after_report.output() / selected_element_count as f64;
    let after = matrix.iter().copied().collect::<Vec<_>>();

    assert_eq!(matrix.shape(), shape);
    assert_correctness(
        shape,
        region,
        &before,
        &after,
        mean_before,
        mean_after,
        &before_report,
        &brighten_report,
        &clamp_report,
        &after_report,
    );

    println!();
    println!("AFTER");
    render_frame(&matrix)?;
    println!();
    println!("REPORT");
    print_report("before sum", &before_report);
    print_report("brighten", &brighten_report);
    print_report("clamp", &clamp_report);
    print_report("after sum", &after_report);
    println!("selected mean brightness: {mean_before:.1} -> {mean_after:.1} / 255.0");
    println!("outside selected Region: unchanged");

    Ok(())
}

fn build_security_camera_frame(shape: Shape, dark_region: Region) -> Vec<f64> {
    let mut pixels = Vec::with_capacity(shape.len());

    for row in 0..shape.rows() {
        for column in 0..shape.columns() {
            let mut value = if row < 4 {
                182.0 - row as f64 * 7.0 + (column % 8) as f64 * 2.5
            } else if row < 13 {
                116.0 + ((row + column) % 6) as f64 * 5.0
            } else {
                74.0 + ((column * 3 + row) % 7) as f64 * 5.0
            };

            if row == 4 || row == 12 {
                value += 24.0;
            }
            if (4..13).contains(&row) && (column == 5 || column == 34) {
                value += 34.0;
            }
            if (6..=9).contains(&row)
                && ((7..=9).contains(&column) || (31..=33).contains(&column))
            {
                value += 28.0;
            }
            if (7..=10).contains(&row) && (18..=21).contains(&column) {
                value += 38.0;
            }
            if (10..=12).contains(&row) && (14..=25).contains(&column) {
                value -= 28.0;
            }

            if contains(dark_region, row, column) {
                value *= 0.34;
            }

            pixels.push(value.max(MIN_PIXEL).min(MAX_PIXEL));
        }
    }

    pixels
}

fn render_frame(matrix: &Matrix<f64>) -> Result<(), MatricalError> {
    println!("+{}+", "-".repeat(matrix.columns()));
    for row in 0..matrix.rows() {
        let mut line = String::with_capacity(matrix.columns());
        for column in 0..matrix.columns() {
            line.push(luminance_char(*matrix.get(Index::new(row, column))?));
        }
        println!("|{line}|");
    }
    println!("+{}+", "-".repeat(matrix.columns()));
    Ok(())
}

fn luminance_char(value: f64) -> char {
    let normalized = value.max(MIN_PIXEL).min(MAX_PIXEL) / MAX_PIXEL;
    let index = (normalized * (LUMINANCE_RAMP.len() - 1) as f64).round() as usize;
    LUMINANCE_RAMP[index] as char
}

fn contains(region: Region, row: usize, column: usize) -> bool {
    row >= region.start_row()
        && row < region.end_row()
        && column >= region.start_column()
        && column < region.end_column()
}

fn demo_tags(stage: TagStage, sequence: u64) -> Vec<Tag> {
    vec![
        Tag::source("security-camera-frame"),
        Tag::stage(stage),
        Tag::sequence(sequence),
    ]
}

fn print_report<O: std::fmt::Debug>(label: &str, report: &ExecutionReport<O>) {
    println!(
        "{label}: gear={} effect={:?} region={:?} output={:?} tags={:?}",
        report.gear(),
        report.effect(),
        report.region(),
        report.output(),
        report.tags()
    );
}

#[allow(clippy::too_many_arguments)]
fn assert_correctness(
    shape: Shape,
    region: Region,
    before: &[f64],
    after: &[f64],
    mean_before: f64,
    mean_after: f64,
    before_report: &ExecutionReport<f64>,
    brighten_report: &ExecutionReport<usize>,
    clamp_report: &ExecutionReport<usize>,
    after_report: &ExecutionReport<f64>,
) {
    let selected_element_count = region.rows() * region.columns();

    assert_eq!(before.len(), shape.len());
    assert_eq!(after.len(), shape.len());
    assert!(after
        .iter()
        .all(|value| (MIN_PIXEL..=MAX_PIXEL).contains(value)));
    assert!(mean_after > mean_before);

    for row in 0..shape.rows() {
        for column in 0..shape.columns() {
            let index = row * shape.columns() + column;
            if contains(region, row, column) {
                let expected = (before[index] + BRIGHTEN_BY)
                    .max(MIN_PIXEL)
                    .min(MAX_PIXEL);
                assert_eq!(after[index], expected);
            } else {
                assert_eq!(after[index], before[index]);
            }
        }
    }

    assert_eq!(before_report.gear(), "sum");
    assert_eq!(before_report.region(), region);
    assert_eq!(before_report.effect(), GearEffect::ReadOnly);

    assert_eq!(brighten_report.gear(), "add_scalar");
    assert_eq!(brighten_report.region(), region);
    assert_eq!(brighten_report.effect(), GearEffect::Mutating);
    assert_eq!(*brighten_report.output(), selected_element_count);
    assert_eq!(
        brighten_report.tags(),
        &[
            Tag::source("security-camera-frame"),
            Tag::stage(TagStage::Transform),
            Tag::sequence(2),
        ]
    );

    assert_eq!(clamp_report.gear(), "clamp");
    assert_eq!(clamp_report.region(), region);
    assert_eq!(clamp_report.effect(), GearEffect::Mutating);
    assert_eq!(*clamp_report.output(), selected_element_count);

    assert_eq!(after_report.gear(), "sum");
    assert_eq!(after_report.region(), region);
    assert_eq!(after_report.effect(), GearEffect::ReadOnly);
}
