use regex::{CaptureLocations, Regex};

use super::lending_iterator::LendingIterator;

pub struct LendingCaptures<'a, 'h> {
  locs: &'a CaptureLocations,
  haystack: &'h str,
}

impl<'a, 'h> LendingCaptures<'a, 'h> {
  pub fn get(&self, i: usize) -> Option<&'h str> {
    self
      .locs
      .get(i)
      .map(|(start, end)| &self.haystack[start..end])
  }
}

pub struct LendingCaptureMatches<'r, 'h> {
  regex: &'r Regex,
  locs: CaptureLocations,
  start: usize,
  haystack: &'h str,
}

impl<'r, 'h> LendingIterator for LendingCaptureMatches<'r, 'h> {
  type Item<'a> = LendingCaptures<'a, 'h> where Self: 'a;

  fn next(&mut self) -> Option<Self::Item<'_>> {
    self
      .regex
      .captures_read_at(&mut self.locs, self.haystack, self.start)
      .map(|m| {
        self.start = m.end();
        LendingCaptures {
          locs: &self.locs,
          haystack: self.haystack,
        }
      })
  }
}

pub trait RegexExt {
  fn lending_captures_iter<'r, 'h>(
    &'r self,
    haystack: &'h str,
  ) -> LendingCaptureMatches<'r, 'h>;
}

impl RegexExt for Regex {
  fn lending_captures_iter<'a, 'h>(
    &'a self,
    haystack: &'h str,
  ) -> LendingCaptureMatches<'a, 'h> {
    LendingCaptureMatches {
      regex: self,
      locs: self.capture_locations(),
      start: 0,
      haystack,
    }
  }
}
