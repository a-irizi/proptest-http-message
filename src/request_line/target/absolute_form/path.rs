//! URL path strategies.

use std::{fmt::Write, num::NonZero, ops::RangeInclusive, sync::LazyLock};

use array_concat::{concat_arrays, concat_arrays_size};
use proptest::{
  option::of,
  prelude::{Just, Strategy},
  prop_oneof,
};

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

/// rootless path with no query params and no fragment.
pub fn path_rootless(max_segments: NonZero<usize>) -> impl Strategy<Value = String> {
  (segment_nz(50), proptest::collection::vec(segment(0, 50), 0..=max_segments.get())).prop_map(
    |(mut segment_nz, segments)| {
      let segments = segments.join("/");
      let _ = write!(segment_nz, "/{segments}");

      segment_nz
    },
  )
}

/// absolute path with no query params and no fragment.
pub fn path_absolute(max_segments: NonZero<usize>) -> impl Strategy<Value = String> {
  of(path_rootless(max_segments))
    .prop_map(|segments| format!("/{}", segments.as_deref().unwrap_or("")))
}

#[cfg(test)]
mod tests {
  use std::num::NonZeroUsize;

  use proptest::{prelude::ProptestConfig, proptest};

  use super::*;

  proptest! {
    #![proptest_config(ProptestConfig::with_cases(100_000))]
    #[test]
    fn path_absolute_works(path in path_absolute(NonZeroUsize::new(25).unwrap())) {
      assert!(path.starts_with('/'));
    }
  }
}
