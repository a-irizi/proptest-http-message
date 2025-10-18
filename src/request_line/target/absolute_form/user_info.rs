//! url authority's user info strategies.

use std::fmt::Write;

use proptest::{
  prelude::{Strategy, any},
  prop_oneof,
  sample::select,
};

#[derive(Debug)]
enum UrlChar {
  Normal(char),
  PercentEncoded(String),
}

const USER_INFO_SAFE_CHARS: [char; 77] = [
  'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
  't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
  'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4',
  '5', '6', '7', '8', '9', '-', '.', '_', '~', '!', '$', '&', '\'', '(', ')', '*', '+', ',', ';',
  '=',
];

fn url_char() -> impl Strategy<Value = UrlChar> {
  let safe_chars_strategy = select(&USER_INFO_SAFE_CHARS).prop_map(UrlChar::Normal);

  let unsafe_chars_strategy =
    any::<char>().prop_filter_map("unreserved, sub-delims or colon", |c: char| {
      if USER_INFO_SAFE_CHARS.contains(&c) {
        None
      } else {
        let mut pct_encoded = String::with_capacity(c.len_utf8() * 3);
        for byte in c.encode_utf8(&mut [0u8; 4]).bytes() {
          let _ = write!(pct_encoded, "%{byte:02x}");
        }

        Some(UrlChar::PercentEncoded(pct_encoded))
      }
    });

  prop_oneof![98 => safe_chars_strategy, 2 => unsafe_chars_strategy]
}

fn user_info_subcomponent() -> impl Strategy<Value = String> {
  proptest::collection::vec(url_char(), 0..=50).prop_map(|chars| {
    let mut result = String::new();
    for c in chars {
      match c {
        UrlChar::Normal(c) => result.push(c),
        UrlChar::PercentEncoded(pct_encoded) => result.push_str(&pct_encoded),
      }
    }

    result
  })
}

/// URI authority's user information.
///
/// user info does not have a standard format, buf for HTTP, it usually takes the form:
/// > `<username>[:[<password>]]`.
///
/// where the password is optional, and can be an empty string.
pub fn user_info() -> impl Strategy<Value = String> {
  (
    user_info_subcomponent(),
    prop_oneof!["", ":", user_info_subcomponent().prop_map(|password| format!(":{password}"))],
  )
    .prop_map(|(username, password)| format!("{username}{password}"))
}

#[cfg(test)]
mod tests {
  use proptest::proptest;

  use super::*;
  proptest! {
    #[test]
    fn userinfo_works(userinfo in user_info()) {
      assert!(userinfo.chars().all(|c| c == '%' || c == ':' || USER_INFO_SAFE_CHARS.contains(&c)));
    }
  }
}
