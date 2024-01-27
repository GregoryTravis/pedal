extern crate std;
extern crate alloc;

use alloc::boxed::Box;
use alloc::string::ToString;
use std::format;
use std::ops::Range;
use std::println;
use std::vec;
use plotters::prelude::full_palette::GREY_500;
use plotters::prelude::*;

pub fn graph_2d_fun(
    filename: &str,
    width: u32,
    height: u32,
    x_range: Range<f32>,
    y_range: Range<f32>,
    fun: fn(f32) -> f32,
) -> Result<(), Box<dyn std::error::Error>> {
    let root_area = BitMapBackend::new(filename, (width, height)).into_drawing_area();

    root_area.fill(&WHITE)?;

    let step: f32 = (x_range.end - x_range.start) / (width as f32);
    let x_axis = x_range.clone().step(step);

    let mut cc = ChartBuilder::on(&root_area)
        .margin(5)
        .set_all_label_area_size(50)
        .caption("Sine and Cosine", ("sans-serif", 40))
        .build_cartesian_2d(x_range, y_range)?;

    cc.configure_mesh()
        .x_labels(10)
        .y_labels(10)
        //.disable_mesh()
        .x_label_formatter(&|v| format!("{:.1}", v))
        .y_label_formatter(&|v| format!("{:.1}", v))
        .draw()?;

    cc.draw_series(LineSeries::new(x_axis.values().map(|x| (x, fun(x))), &RED))?
        .label("Sine")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));

    cc.configure_series_labels().border_style(BLACK).draw()?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root_area.present().expect("");
    println!("Result has been saved to {}", filename);
    Ok(())
}

// fn wall_projections(fun: fn(f32) -> (f32, f32, f32), x_range: Range<f32>, y_range: Range<f32>, z_range: Range<f32>) {
// }

pub fn graph_3d_line_fun(
    filename: &str,
    width: u32,
    height: u32,
    x_range: Range<f32>,
    y_range: Range<f32>,
    z_range: Range<f32>,
    fun: fn(f32) -> (f32, f32, f32),
    num_points: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let area = SVGBackend::new(filename, (width, height)).into_drawing_area();

    area.fill(&WHITE)?;

    let x_range_end = x_range.clone().start;
    let y_range_end = y_range.clone().start;
    let z_range_end = z_range.clone().start;

    let x_axis = x_range.step(0.1);
    let y_axis = y_range.step(0.1);
    let z_axis = z_range.step(0.1);

    let mut chart = ChartBuilder::on(&area)
        .caption("3D Plot Test".to_string(), ("sans", 20))
        .build_cartesian_3d(x_axis.clone(), y_axis.clone(), z_axis.clone())?;

    chart.with_projection(|mut pb| {
        pb.yaw = 0.5;
        pb.scale = 0.9;
        pb.into_matrix()
    });

    chart
        .configure_axes()
        .light_grid_style(BLACK.mix(0.15))
        .max_light_lines(3)
        .draw()?;

    chart
        .draw_series(LineSeries::new(
            (0.0f32..1.0f32)
                .step(1.0 / (num_points as f32))
                .values()
                .map(fun),
            &BLACK,
        ))?
        .label("Line")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLACK));

    let x_projector = |(x, y, _z): (f32, f32, f32)| (x, y, z_range_end);
    let y_projector = |(x, _y, z): (f32, f32, f32)| (x, y_range_end, z);
    let z_projector = |(_x, y, z): (f32, f32, f32)| (x_range_end, y, z);

    chart.draw_series(LineSeries::new(
        (0.0f32..1.0f32)
            .step(1.0 / (num_points as f32))
            .values()
            .map(fun)
            .map(x_projector),
        &GREY_500,
    ))?;
    chart.draw_series(LineSeries::new(
        (0.0f32..1.0f32)
            .step(1.0 / (num_points as f32))
            .values()
            .map(fun)
            .map(y_projector),
        &GREY_500,
    ))?;
    chart.draw_series(LineSeries::new(
        (0.0f32..1.0f32)
            .step(1.0 / (num_points as f32))
            .values()
            .map(fun)
            .map(z_projector),
        &GREY_500,
    ))?;

    chart.configure_series_labels().border_style(BLACK).draw()?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    area.present().expect("");
    println!("Result has been saved to {}", filename);
    Ok(())
}
