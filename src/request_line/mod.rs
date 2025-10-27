//! HTTP request line strategies.

use std::{num::NonZero, ops::RangeInclusive};

use proptest::prelude::Strategy;

use crate::request_line::{target::RequestTarget, version::HttpVersion};

pub mod target;
pub mod verb;
pub mod version;

/// Http request line components.
#[derive(Debug)]
pub struct HttpRequestLine {
  pub verb: String,
  pub target: RequestTarget,
  pub version: HttpVersion,
}

/// strategy for generating HTTP request line.
///
/// # Arguments
/// * `max_label_count`: maximum label count to use for domain hosts in case of authority form,
///   origin form and absolute form.
/// * `max_segments`: maximum number of segments that compose the path in case of absolute form
///   and origin form.
/// * `query_count_range`: range of the number of queries to include in case of  absolute form
///   and origin form.
///
/// # Returns
/// [`HttpRequestLine`] and it representation.
pub fn request_line(
  max_label_count: usize,
  max_segments: NonZero<usize>,
  query_count_range: RangeInclusive<usize>,
) -> impl Strategy<Value = (HttpRequestLine, String)> {
  (
    verb::request_verb(),
    target::target(max_label_count, max_segments, query_count_range),
    version::version(),
  )
    .prop_map(|(verb, (target, target_repr), (version, version_repr))| {
      let repr = format!("{verb} {target_repr} {version_repr}");
      (HttpRequestLine { verb, target, version }, repr)
    })
}

#[cfg(test)]
mod tests {
  use proptest::proptest;

  use super::*;

  proptest! {
    #[test]
    fn request_line_works((request_line, repr) in request_line(20, 50.try_into().unwrap(), 0..=20)) {
      let mut request_line_components = repr.split_ascii_whitespace();

      let verb = request_line_components.next().unwrap();
      assert_eq!(verb, request_line.verb, "expected to get verb {:?} but parsed {:?}", request_line.verb, verb);
      verb::tests::request_verb_asserts(verb);

      let target = request_line_components.next().unwrap();
      target::tests::target_asserts(&request_line.target, target);

      let version = request_line_components.next().unwrap();
      version::tests::version_asserts(&request_line.version, version);
    }
  }
}
