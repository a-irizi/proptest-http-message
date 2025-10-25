//! URL authority's user info strategies.

use std::{fmt::Write, ops::RangeInclusive, sync::LazyLock};

use array_concat::{concat_arrays, concat_arrays_size};
use proptest::{option::of, prelude::Strategy};

use crate::request_line::{
  SUB_DELIMS, UNRESERVED, char_diff_intervals, safe_and_percent_encoded_char, url_chars_to_string,
};

static USER_INFO_UNSAFE_CHARS: LazyLock<Vec<RangeInclusive<char>>> =
  LazyLock::new(|| char_diff_intervals(&USER_INFO_SAFE_CHARS));

const USER_INFO_SAFE_CHARS: [char; concat_arrays_size!(UNRESERVED, SUB_DELIMS)] =
  concat_arrays!(UNRESERVED, SUB_DELIMS);

fn user_info_subcomponent() -> impl Strategy<Value = String> {
  proptest::collection::vec(
    safe_and_percent_encoded_char(&USER_INFO_SAFE_CHARS, &USER_INFO_UNSAFE_CHARS),
    0..=50,
  )
  .prop_map(url_chars_to_string)
}

#[derive(Debug)]
pub struct UserInfo {
  pub username: String,
  pub password: Option<String>,
}

/// URI authority's user information.
///
/// user info does not have a standard format, buf for HTTP, it usually takes the form:
/// > `<username>[:[<password>]]`.
///
/// where the password is optional, and can be an empty string.
/// # Returns
/// `UserInfo` along with it's representation.
pub fn user_info() -> impl Strategy<Value = (UserInfo, String)> {
  (user_info_subcomponent(), of(user_info_subcomponent())).prop_map(|(username, password)| {
    let mut repr = username.clone();
    if let Some(password) = password.as_ref() {
      let _ = write!(repr, ":{password}");
    }
    (UserInfo { username, password }, repr)
  })
}

#[cfg(test)]
mod tests {
  use proptest::proptest;

  use super::*;
  proptest! {
    #[test]
    fn userinfo_works((_, repr) in user_info()) {
      println!("{repr}");
      assert!(repr.chars().all(|c| c == '%' || c == ':' || USER_INFO_SAFE_CHARS.contains(&c)));
    }
  }
}
