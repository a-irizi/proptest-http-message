//! URL fragment strategies.

use std::{ops::RangeInclusive, sync::LazyLock};

use array_concat::{concat_arrays, concat_arrays_size};
use proptest::prelude::Strategy;

use crate::request_line::{
  SUB_DELIMS, UNRESERVED, UrlChar, char_diff_intervals, safe_and_percent_encoded_char,
};

static FRAGMENT_UNSAFE_CHARS: LazyLock<Vec<RangeInclusive<char>>> =
  LazyLock::new(|| char_diff_intervals(&FRAGMENT_SAFE_CHARS));

const FRAGMENT_SAFE_CHARS: [char; concat_arrays_size!(UNRESERVED, SUB_DELIMS) + 4] =
  concat_arrays!(UNRESERVED, SUB_DELIMS, [':', '@', '/', '?']);

fn chars() -> impl Strategy<Value = UrlChar> {
  safe_and_percent_encoded_char(&FRAGMENT_SAFE_CHARS, &FRAGMENT_UNSAFE_CHARS)
}

/// URL fragment.
pub fn fragment() -> impl Strategy<Value = String> {
  proptest::collection::vec(chars(), 0..=125).prop_map(|chars| {
    let mut fragment = String::new();
    for c in chars {
      match c {
        UrlChar::Normal(c) => fragment.push(c),
        UrlChar::PercentEncoded(s) => fragment.push_str(&s),
      }
    }

    fragment
  })
}
