use std::{fmt::Display, str::FromStr};

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Grid<T> {
  data: Vec<T>,
  rows: usize,
}

impl<T> Grid<T> {
  pub const fn rows(&self) -> isize {
    self.rows as isize
  }

  pub fn cols(&self) -> isize {
    (self.data.len() as isize) / self.rows()
  }

  pub fn get(&self, [x, y]: [isize; 2]) -> Option<&T> {
    let rows = self.rows();
    let cols = self.cols();

    if (0..rows).contains(&y) && (0..cols).contains(&x) {
      Some(&self.data[(rows * y + x) as usize])
    } else {
      None
    }
  }

  pub fn get_mut(&mut self, [x, y]: [isize; 2]) -> Option<&mut T> {
    let rows = self.rows();
    let cols = self.cols();

    if (0..rows).contains(&y) && (0..cols).contains(&x) {
      Some(&mut self.data[(rows * y + x) as usize])
    } else {
      None
    }
  }

  pub fn keys(&self) -> impl Iterator<Item = [isize; 2]> + '_ {
    let rows = self.rows();
    let cols = self.cols();

    (0..rows).flat_map(move |y| (0..cols).map(move |x| [x, y]))
  }

  pub fn values(&self) -> impl Iterator<Item = &T> {
    self.data.iter()
  }

  pub fn iter(&self) -> impl Iterator<Item = ([isize; 2], &T)> + '_ {
    let rows = self.rows();
    let cols = self.cols();

    (0..rows).flat_map(move |y| (0..cols).map(move |x| ([x, y], &self[[x, y]])))
  }
}

impl<T> std::ops::Index<[isize; 2]> for Grid<T> {
  type Output = T;

  fn index(&self, idx: [isize; 2]) -> &Self::Output {
    self.get(idx).unwrap()
  }
}

impl<T> std::ops::IndexMut<[isize; 2]> for Grid<T> {
  fn index_mut(&mut self, idx: [isize; 2]) -> &mut Self::Output {
    self.get_mut(idx).unwrap()
  }
}

#[derive(Debug)]
pub enum ParseGridError<ItemError> {
  InvalidItem(ItemError),
}

impl<T> FromStr for Grid<T>
where
  T: TryFrom<char>,
{
  type Err = ParseGridError<T::Error>;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut rows = 0;
    let mut data = Vec::new();

    for line in s.lines() {
      rows += 1;
      for c in line.chars() {
        data.push(c.try_into().map_err(ParseGridError::InvalidItem)?);
      }
    }

    Ok(Self { rows, data })
  }
}

impl<T> Display for Grid<T>
where
  T: Display,
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let rows = self.rows();
    let cols = self.cols();

    for y in 0..rows {
      for x in 0..cols {
        write!(f, "{}", self[[x, y]])?;
      }
      writeln!(f)?;
    }

    Ok(())
  }
}
