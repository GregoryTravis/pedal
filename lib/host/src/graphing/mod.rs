use std::ops::Range;

use plotters::prelude::*;

pub fn graph_2d_fun(filename: &str, width: u32, height: u32, x_range: Range<f32>, y_range: Range<f32>, fun: fn(f32) -> f32) -> Result<(), Box<dyn std::error::Error>> {
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
    root_area.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", filename);
    Ok(())
}

pub fn graph_3d_line_fun(filename: &str, width: u32, height: u32, x_range: Range<f32>, y_range: Range<f32>, z_range: Range<f32>, fun: fn(f32) -> (f32, f32, f32), num_points: u32) -> Result<(), Box<dyn std::error::Error>> {
    let area = SVGBackend::new(filename, (width, height)).into_drawing_area();

    area.fill(&WHITE)?;

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
            (0.0f32..1.0f32).step(1.0 / (num_points as f32)).values().map(fun),
            &BLACK,
        ))?
        .label("Line")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLACK));

    chart.configure_series_labels().border_style(BLACK).draw()?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    area.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", filename);
    Ok(())
}
