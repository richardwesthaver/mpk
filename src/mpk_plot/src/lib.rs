use mpk_db::VecReal;
use plotters::prelude::*;
use std::ops::Range;
const OUT_FILE_NAME: &'static str = "mandelbrot.png";

pub fn plot_histogram(data: VecReal) -> Result<(), Box<dyn std::error::Error>> {
  let root = BitMapBackend::new(OUT_FILE_NAME, (640, 480)).into_drawing_area();

  root.fill(&WHITE)?;

  let mut chart = ChartBuilder::on(&root)
    .x_label_area_size(35u32)
    .y_label_area_size(40u32)
    .margin(5u32)
    .caption("Histogram Test", ("sans-serif", 50.0f32))
    .build_cartesian_2d((0u32..10u32).into_segmented(), 0u32..10u32)?;

  chart
    .configure_mesh()
    .disable_x_mesh()
    .bold_line_style(&WHITE.mix(0.3))
    .y_desc("Count")
    .x_desc("Bucket")
    .axis_desc_style(("sans-serif", 15u32))
    .draw()?;

  chart.draw_series(
    Histogram::vertical(&chart)
      .style(RED.mix(0.5).filled())
      .data(data.0.iter().map(|x| (*x as u32, 1))),
  )?;

  root.present().expect("Unable to write result to file");
  println!("Result has been saved to {}", OUT_FILE_NAME);
  Ok(())
}

pub fn test_draw() -> Result<(), Box<dyn std::error::Error>> {
  let root = BitMapBackend::new(OUT_FILE_NAME, (800, 600)).into_drawing_area();

  root.fill(&WHITE)?;

  let mut chart = ChartBuilder::on(&root)
    .margin(20)
    .x_label_area_size(10)
    .y_label_area_size(10)
    .build_cartesian_2d(-2.1f64..0.6f64, -1.2f64..1.2f64)?;

  chart
    .configure_mesh()
    .disable_x_mesh()
    .disable_y_mesh()
    .draw()?;

  let plotting_area = chart.plotting_area();

  let range = plotting_area.get_pixel_range();

  let (pw, ph) = (range.0.end - range.0.start, range.1.end - range.1.start);
  let (xr, yr) = (chart.x_range(), chart.y_range());

  for (x, y, c) in mandelbrot_set(xr, yr, (pw as usize, ph as usize), 100) {
    if c != 100 {
      plotting_area.draw_pixel((x, y), &HSLColor(c as f64 / 100.0, 1.0, 0.5))?;
    } else {
      plotting_area.draw_pixel((x, y), &BLACK)?;
    }
  }

  // To avoid the IO failure being ignored silently, we manually call the present function
  root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
  println!("Result has been saved to {}", OUT_FILE_NAME);

  Ok(())
}

fn mandelbrot_set(
  real: Range<f64>,
  complex: Range<f64>,
  samples: (usize, usize),
  max_iter: usize,
) -> impl Iterator<Item = (f64, f64, usize)> {
  let step = (
    (real.end - real.start) / samples.0 as f64,
    (complex.end - complex.start) / samples.1 as f64,
  );
  return (0..(samples.0 * samples.1)).map(move |k| {
    let c = (
      real.start + step.0 * (k % samples.0) as f64,
      complex.start + step.1 * (k / samples.0) as f64,
    );
    let mut z = (0.0, 0.0);
    let mut cnt = 0;
    while cnt < max_iter && z.0 * z.0 + z.1 * z.1 <= 1e10 {
      z = (z.0 * z.0 - z.1 * z.1 + c.0, 2.0 * z.0 * z.1 + c.1);
      cnt += 1;
    }
    return (c.0, c.1, cnt);
  });
}
