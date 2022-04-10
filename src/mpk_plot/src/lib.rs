use mpk_db::{MatrixReal, VecReal, VecText};
//use plotters::prelude::*;

pub enum PlotType {
  Spectrogram(MatrixReal),
  Line(VecReal),
  Histogram(VecReal),
  WordCloud(VecText),
}
