//! URL path strategies.

use std::{num::NonZero, ops::RangeInclusive, sync::LazyLock};

use array_concat::{concat_arrays, concat_arrays_size};
use proptest::prelude::Strategy;

use crate::request_line::{
  SUB_DELIMS, UNRESERVED, UrlChar, char_diff_intervals, safe_and_percent_encoded_char,
  url_chars_to_string,
};

static PATH_UNSAFE_CHARS: LazyLock<Vec<RangeInclusive<char>>> =
  LazyLock::new(|| char_diff_intervals(&PATH_SAFE_CHARS));

const PATH_SAFE_CHARS: [char; concat_arrays_size!(UNRESERVED, SUB_DELIMS) + 2] =
  concat_arrays!(UNRESERVED, SUB_DELIMS, [':', '@']);

fn pchar() -> impl Strategy<Value = UrlChar> {
  safe_and_percent_encoded_char(&PATH_SAFE_CHARS, &PATH_UNSAFE_CHARS)
}

fn segment(min_chars: usize, max_chars: usize) -> impl Strategy<Value = String> {
  proptest::collection::vec(pchar(), min_chars..max_chars).prop_map(url_chars_to_string)
}

fn segment_nz(max_chars: usize) -> impl Strategy<Value = String> {
  segment(1, max_chars)
}

/// URL path.
#[derive(Debug)]
pub struct Path {
  /// normalized path.
  /// # Examples
  /// * the path `"/foo/./bar"` will be normalized to `"/foo/bar"`
  /// * the path `"/foo/../bar"` will be normalized to `"/bar"`
  pub normalized: String,
}

/// rootless path with no query params and no fragment.
/// # Returns
/// [`Path`] and its raw representation.
pub fn path_rootless(max_segments: NonZero<usize>) -> impl Strategy<Value = (Path, String)> {
  (segment_nz(50), proptest::collection::vec(segment(0, 50), 0..=max_segments.get())).prop_map(
    |(segment_nz, segments)| {
      let repr = if segments.is_empty() {
        segment_nz.clone()
      } else {
        format!("{segment_nz}/{segments}", segments = segments.join("/"))
      };

      let mut normalized_path_segments = vec![];

      if segment_nz != "." && segment_nz != ".." {
        normalized_path_segments.push(segment_nz);
      }

      let segment_count = segments.len();
      for (idx, segment) in segments.into_iter().enumerate() {
        match segment.as_str() {
          "." => {
            if idx == segment_count - 1 {
              normalized_path_segments.push(String::new());
            }
          }
          ".." => {
            normalized_path_segments.pop();
          }
          _ => normalized_path_segments.push(segment),
        }
      }

      (
        Path {
          normalized: if normalized_path_segments.is_empty() {
            "/".to_string()
          } else {
            normalized_path_segments.join("/")
          },
        },
        repr,
      )
    },
  )
}

/// absolute path with no query params and no fragment.
/// #Returns
/// [`Path`] and its raw representation.
pub fn path_absolute(max_segments: NonZero<usize>) -> impl Strategy<Value = (Path, String)> {
  path_rootless(max_segments).prop_map(|(path, repr)| {
    (Path { normalized: format!("/{}", path.normalized) }, format!("/{repr}"))
  })
}

#[cfg(test)]
mod tests {
  use std::num::NonZeroUsize;

  use proptest::{prelude::ProptestConfig, proptest};

  use super::*;

  proptest! {
    #![proptest_config(ProptestConfig::with_cases(10_000))]
    #[test]
    fn path_absolute_works((_, repr) in path_absolute(NonZeroUsize::new(25).unwrap())) {
      assert!(repr.starts_with('/'));
    }
  }
}
