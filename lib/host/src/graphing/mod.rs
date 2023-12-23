use std::ops::Range;

use plotters::prelude::*;

const OUT_FILE_NAME: &str = "plotters-doc-data/sample.png";
pub fn graph2dfun(width: u32, height: u32, x_range: Range<f32>, y_range: Range<f32>, fun: fn(f32) -> f32) -> Result<(), Box<dyn std::error::Error>> {
    let root_area = BitMapBackend::new(OUT_FILE_NAME, (width, height)).into_drawing_area();

    root_area.fill(&WHITE)?;

    //let root_area = root_area.titled("Image Title", ("sans-serif", 60))?;

    //let (upper, lower) = root_area.split_vertically(512);

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

    cc.draw_series(LineSeries::new(
        x_axis.values().map(|x| (x, x.cos())),
        &BLUE,
    ))?
    .label("Cosine")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE));

    cc.configure_series_labels().border_style(BLACK).draw()?;

    /*
    // It's possible to use a existing pointing element
     cc.draw_series(PointSeries::<_, _, Circle<_>>::new(
        (-3.0f32..2.1f32).step(1.0).values().map(|x| (x, x.sin())),
        5,
        Into::<ShapeStyle>::into(&RGBColor(255,0,0)).filled(),
    ))?;*/

    // To avoid the IO failure being ignored silently, we manually call the present function
    root_area.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", OUT_FILE_NAME);
    Ok(())
}
