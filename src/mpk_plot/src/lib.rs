use mpk_db::{MatrixReal, VecReal, VecText};
use plotters::prelude::*;
const OUT_FILE_NAME: &'static str = "mandelbrot.png";

pub enum PlotType {
  Spectrogram(MatrixReal),
  Line(VecReal),
  Histogram(VecReal),
  WordCloud(VecText),
}

pub fn plot_spec(data: MatrixReal) -> Result<(), Box<dyn std::error::Error>> {
  let root = BitMapBackend::new(OUT_FILE_NAME, (640, 480)).into_drawing_area();

  root.fill(&WHITE)?;

  let mut chart = ChartBuilder::on(&root)
    .x_label_area_size(35)
    .y_label_area_size(40)
    .margin(5)
    .caption("Spectrogram", ("sans-serif", 50.0))
    .build_cartesian_2d(0..data.frames(), -1.0f32..1.0f32)?;

  chart
    .configure_mesh()
    .disable_x_mesh()
    .bold_line_style(&WHITE.mix(0.3))
    .y_desc("Count")
    .x_desc("Bucket")
    .axis_desc_style(("sans-serif", 15u32))
    .draw()?;

  root.present().expect("Unable to write result to file");
  println!("Result has been saved to {}", OUT_FILE_NAME);
  Ok(())
}
