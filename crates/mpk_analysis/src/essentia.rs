//! MPK_ANALYSIS -- ESSENTIA
/// A Vec<f32>
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct VecReal(pub Vec<f32>);

impl VecReal {
  pub fn len(&self) -> usize {
    self.0.len()
  }
}

impl fmt::Display for VecReal {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut iter = self.0.iter();
    let mut riter = self.0.iter().rev();
    write!(
      f,
      "vec([{0}, {1}, {2}, {3} ... {7}, {6}, {5}, {4}], len={8})",
      iter.next().unwrap(),
      iter.next().unwrap(),
      iter.next().unwrap(),
      iter.next().unwrap(),
      riter.next().unwrap(),
      riter.next().unwrap(),
      riter.next().unwrap(),
      riter.next().unwrap(),
      self.0.len()
    )
  }
}

impl From<Vec<f32>> for VecReal {
  fn from(v: Vec<f32>) -> Self {
    VecReal(v)
  }
}

impl Iterator for VecReal {
  type Item = f32;
  fn next(&mut self) -> Option<Self::Item> {
    self.into_iter().next()
  }
}

impl Index<usize> for VecReal {
  type Output = f32;
  fn index(&self, idx: usize) -> &Self::Output {
    &self.0[idx]
  }
}

impl Index<Range<usize>> for VecReal {
  type Output = [f32];
  fn index(&self, idx: Range<usize>) -> &Self::Output {
    &self.0[idx]
  }
}

impl From<MatrixReal> for VecReal {
  fn from(m: MatrixReal) -> Self {
    m.vec
  }
}

/// A Matrix of f32s for SQLite. Implemented as a flat Vec with a frame_size
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct MatrixReal {
  pub vec: VecReal,
  pub frame_size: usize,
}

impl MatrixReal {
  pub fn new(vec: VecReal, frame_size: usize) -> Self {
    MatrixReal { vec, frame_size }
  }
  pub fn to_vec(&self) -> VecReal {
    self.clone().into()
  }
  pub fn frames(&self) -> usize {
    self.vec.len() / self.frame_size
  }
}

impl Index<usize> for MatrixReal {
  type Output = [f32];
  fn index(&self, idx: usize) -> &Self::Output {
    &self.vec[idx * self.frame_size..idx + 1 * self.frame_size]
  }
}

impl fmt::Display for MatrixReal {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "matrix([{}, ...], frame_size={}, frames={})",
      VecReal::from(self[0].to_vec()),
      //      VecReal::from(self[1].to_vec()),
      self.frame_size,
      self.frames()
    )
  }
}

//TODO
#[derive(Debug, Serialize, Deserialize)]
pub enum Note {
  C = 0,
  Db = 1,
  D = 2,
  Eb = 3,
  E = 4,
  F = 5,
  Gb = 6,
  G = 7,
  Ab = 8,
  A = 9,
  Bb = 10,
  B = 11,
}

impl Note {
  pub fn val(&self) -> u8 {
    match self {
      Note::C => 0,
      Note::Db => 1,
      Note::D => 2,
      Note::Eb => 3,
      Note::E => 4,
      Note::F => 5,
      Note::Gb => 6,
      Note::G => 7,
      Note::Ab => 8,
      Note::A => 9,
      Note::Bb => 10,
      Note::B => 11,
    }
  }
}

impl FromStr for Note {
  type Err = Error;
  fn from_str(input: &str) -> Result<Note> {
    match input {
      "C" => Ok(Note::C),
      "Db" => Ok(Note::Db),
      "D" => Ok(Note::D),
      "Eb" => Ok(Note::Eb),
      "E" => Ok(Note::E),
      "F" => Ok(Note::F),
      "Gb" => Ok(Note::Gb),
      "G" => Ok(Note::G),
      "Ab" => Ok(Note::Ab),
      "A" => Ok(Note::A),
      "Bb" => Ok(Note::Bb),
      "B" => Ok(Note::B),
      e => Err(Error::BadNote(e.to_string())),
    }
  }
}

impl From<&u8> for Note {
  fn from(n: &u8) -> Self {
    match n {
      0 => Note::C,
      1 => Note::Db,
      2 => Note::D,
      3 => Note::Eb,
      4 => Note::E,
      5 => Note::F,
      6 => Note::Gb,
      7 => Note::G,
      8 => Note::Ab,
      9 => Note::A,
      10 => Note::Bb,
      11 => Note::B,
      _ => panic!("invalid note value"),
    }
  }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct VecNote(pub Vec<Note>);

impl VecNote {
  fn as_bytes(&self) -> Vec<u8> {
    self.0.iter().map(|n| n.val()).collect()
  }
}

impl From<Vec<u8>> for VecNote {
  fn from(v: Vec<u8>) -> Self {
    VecNote(v.iter().map(|n| Note::from(n)).collect())
  }
}

impl From<&[u8]> for VecNote {
  fn from(v: &[u8]) -> Self {
    VecNote(v.iter().map(|n| Note::from(n)).collect())
  }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct VecText(pub Vec<String>);

