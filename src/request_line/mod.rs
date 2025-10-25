//! HTTP request line strategies.

use std::{fmt::Write, ops::RangeInclusive};

use array_concat::{concat_arrays, concat_arrays_size};
use proptest::{char::ranges, prelude::Strategy, prop_oneof, sample::select};
pub mod target;
pub mod verb;

const UNRESERVED: [char; 66] = [
  'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
  't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
  'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4',
  '5', '6', '7', '8', '9', '-', '.', '_', '~',
];

const SUB_DELIMS: [char; 11] = ['!', '$', '&', '’', '(', ')', '*', '+', ',', ';', '='];
const GEN_DELIMS: [char; 7] = [':', '/', '?', '#', '[', ']', '@'];
const RESERVED: [char; concat_arrays_size!(SUB_DELIMS, GEN_DELIMS)] =
  concat_arrays!(SUB_DELIMS, GEN_DELIMS);

/// URL character.
#[derive(Debug, Clone)]
enum UrlChar {
  /// normal character.
  Normal(char),
  /// percent encoded character.
  PercentEncoded(String),
}

fn percent_encoded_char(chars: impl Strategy<Value = char>) -> impl Strategy<Value = String> {
  chars.prop_map(|c: char| {
    let mut pct_encoded = String::with_capacity(c.len_utf8() * 3);
    for byte in c.encode_utf8(&mut [0u8; 4]).bytes() {
      let _ = write!(pct_encoded, "%{byte:02x}");
    }

    pct_encoded
  })
}

fn char_diff_intervals(chars: &[char]) -> Vec<RangeInclusive<char>> {
  let mut chars = chars.to_vec();
  chars.sort_unstable();
  chars.dedup();

  if chars.is_empty() {
    return vec!['\0'..=char::MAX];
  }

  let mut unsafe_chars = Vec::new();
  if let Some(&first_safe) = chars.first()
    && first_safe > '\0'
  {
    unsafe_chars.push('\0'..=char::from_u32(first_safe as u32 - 1).unwrap());
  }

  for window in chars.windows(2) {
    let current = window[0] as u32;
    let next = window[1] as u32;

    if next - current > 1 {
      unsafe_chars.push(char::from_u32(current + 1).unwrap()..=char::from_u32(next - 1).unwrap());
    }
  }

  if let Some(&last) = chars.last()
    && last < char::MAX
  {
    unsafe_chars.push(char::from_u32(last as u32 + 1).unwrap()..=char::MAX);
  }

  unsafe_chars
}

fn safe_and_percent_encoded_char(
  safe_chars: &'static [char],
  unsafe_chars_ranges: &[RangeInclusive<char>],
) -> impl Strategy<Value = UrlChar> {
  let safe_chars_strategy = select(safe_chars).prop_map(UrlChar::Normal);

  let unsafe_chars_strategy =
    percent_encoded_char(ranges(std::borrow::Cow::Borrowed(unsafe_chars_ranges)))
      .prop_map(UrlChar::PercentEncoded);

  prop_oneof![98 => safe_chars_strategy, 2 => unsafe_chars_strategy]
}

fn url_chars_to_string(chars: Vec<UrlChar>) -> String {
  let mut result = String::new();
  for c in chars {
    match c {
      UrlChar::Normal(c) => result.push(c),
      UrlChar::PercentEncoded(pct_encoded) => result.push_str(&pct_encoded),
    }
  }

  result
}
