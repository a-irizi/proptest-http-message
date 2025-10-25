//! URL query strategies.

use std::{ops::RangeInclusive, sync::LazyLock};

use proptest::prelude::Strategy;

use crate::request_line::{
  SUB_DELIMS, UNRESERVED, UrlChar, char_diff_intervals, safe_and_percent_encoded_char,
  url_chars_to_string,
};

static QUERY_UNSAFE_CHARS: LazyLock<Vec<RangeInclusive<char>>> =
  LazyLock::new(|| char_diff_intervals(&QUERY_SAFE_CHARS));

static QUERY_SAFE_CHARS: LazyLock<Vec<char>> = LazyLock::new(|| {
  // space character is included in safe chars, because it should be replaced with
  // '+' and not percent-encoded.
  let mut safe_chars = vec![':', '@', '/', '?', ' '];
  safe_chars.extend_from_slice(&UNRESERVED);
  safe_chars.extend(SUB_DELIMS.into_iter().filter(|c| !matches!(c, '+' | '&' | '=')));

  safe_chars
});

fn chars() -> impl Strategy<Value = UrlChar> {
  safe_and_percent_encoded_char(&QUERY_SAFE_CHARS, &QUERY_UNSAFE_CHARS).prop_map(|c| {
    if let UrlChar::Normal(c) = c
      && c == ' '
    {
      // url-encoding requires space to be encoded as '+' instead of percent encoding
      UrlChar::Normal('+')
    } else {
      c
    }
  })
}

fn query_subcomponent(min_chars: usize, max_chars: usize) -> impl Strategy<Value = String> {
  proptest::collection::vec(chars(), min_chars..=max_chars).prop_map(url_chars_to_string)
}

/// URL Query parameter.
#[derive(Debug)]
pub struct QueryParam {
  /// param key.
  pub key: String,
  /// param value.
  pub value: Option<String>,
}

/// single URL query param
/// # Returns
/// [`QueryParam`] with it representation in the form `<key>=<value>`.
pub fn query_param() -> impl Strategy<Value = (QueryParam, String)> {
  (query_subcomponent(0, 50), query_subcomponent(0, 50)).prop_map(|(key, value)| {
    let repr = format!("{key}={value}");
    (QueryParam { key, value: if value.is_empty() { None } else { Some(value) } }, repr)
  })
}

/// URL query.
///
/// # Returns
/// Vec of [`QueryParam`] and it representation. individual params are separated by `'&'`.
pub fn query(
  min_queries: usize,
  max_queries: usize,
) -> impl Strategy<Value = (Vec<QueryParam>, String)> {
  proptest::collection::vec(query_param(), min_queries..=max_queries).prop_map(|params| {
    let (params, reprs): (Vec<_>, Vec<_>) = params.into_iter().unzip();
    (params, reprs.join("&"))
  })
}

#[cfg(test)]
mod tests {
  use proptest::proptest;

  use super::*;

  proptest! {
    #[test]
    fn query_param_works((param, repr) in query_param()) {
      println!("{repr:?}");
      assert!(repr.starts_with(param.key.as_str()), "param should start with key but got {param:?} {repr:?}");
      assert!(repr.ends_with(param.value.as_deref().unwrap_or_default()), "param should end with value but got {param:?} {repr:?}");
    }
  }
}
