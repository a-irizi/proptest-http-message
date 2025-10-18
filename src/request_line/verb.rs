//! HTTP request verb strategies.

use proptest::{prelude::Strategy, prop_compose, prop_oneof};
use rand::Rng;

const GET_VERB: &str = "GET";
const HEAD_VERB: &str = "HEAD";
const POST_VERB: &str = "POST";
const PUT_VERB: &str = "PUT";
const DELETE_VERB: &str = "DELETE";
const CONNECT_VERB: &str = "CONNECT";
const OPTIONS_VERB: &str = "OPTIONS";
const TRACE_VERB: &str = "TRACE";
const PATCH_VERB: &str = "PATCH";

/// strategy for generating correct HTTP request verb.
///
/// # Returns
/// one of the following verbs:
/// * `GET`: transfer a current representation of the target resource.
/// * `HEAD`: same as GET, but do not transfer the response content.
/// * `POST`: perform resource-specific processing on the request content.
/// * `PUT`: replace all current representations of the target resource with the
///   request content.
/// * `DELETE`: remove all current representations of the target resource.
/// * `CONNECT`: establish a tunnel to the server identified by the target resource.
/// * `OPTIONS`: describe the communication options for the target resource.
/// * `TRACE`: perform a message loop-back test along the path to the target
///   resource.
/// * `Patch`: apply partial modifications to a resource.
///
/// # Example
/// ```rust,ignore
/// use proptest::prelude::*;
/// use proptest_http_message::request_line::verb::request_verb;
/// proptest!{
///   #[test]
///   fn request_verb_ok(verb in request_verb()) {
///     assert!(
///       matches!(
///         verb.as_str(),
///         "GET" | "HEAD" | "POST" | "PUT" | "DELETE" | "CONNECT" | "OPTIONS" | "TRACE" | "PATCH"
///       )
///     )
///   }
/// }
/// ```
pub fn request_verb() -> impl Strategy<Value = String> {
  prop_oneof![
    GET_VERB,
    HEAD_VERB,
    POST_VERB,
    PUT_VERB,
    DELETE_VERB,
    CONNECT_VERB,
    OPTIONS_VERB,
    TRACE_VERB,
    PATCH_VERB,
  ]
}

/// strategy for generating HTTP request verb but with wrong case.
///
/// HTTP verbs are case sensitive and they should be in upper case.
///
/// # Example
/// ```rust,ignore
/// use proptest::prelude::*;
/// use proptest_http_message::request_line::verb::request_verb;
/// proptest! {
///   #[test]
///   fn request_verb_wrong_case_ok(verb in request_verb_wrong_case()) {
///     assert!(
///       !matches!(
///         verb.as_str(),
///         "GET" | "HEAD" | "POST" | "PUT" | "DELETE" | "CONNECT" | "OPTIONS" | "TRACE" | "PATCH"
///       )
///     );
///
///     // only the case is different.
///     let upper_case_verb = verb.to_uppercase();
///     assert!(
///       matches!(
///         upper_case_verb.as_str(),
///         "GET" | "HEAD" | "POST" | "PUT" | "DELETE" | "CONNECT" | "OPTIONS" | "TRACE" | "PATCH",
///       )
///     );
///   }
/// }
/// ```
pub fn request_verb_wrong_case() -> impl Strategy<Value = String> {
  request_verb().prop_map(|verb| {
    let mut rng = rand::rng();

    let mut wrong_case_verb = String::with_capacity(verb.len());

    loop {
      let mut changed_case = false;
      for c in verb.chars() {
        if rng.random_bool(0.5) {
          wrong_case_verb.push(c.to_ascii_lowercase());
          changed_case = true;
        } else {
          wrong_case_verb.push(c);
        }
      }
      if changed_case {
        break;
      }
      wrong_case_verb.clear();
    }

    wrong_case_verb
  })
}

prop_compose! {
  /// strategy for generating invalid HTTP verbs.
  pub fn request_verb_wrong()
  (input in ".*".prop_filter("valid HTTP verb", |input|
    !matches!(
      input.as_str(),
      "GET" | "HEAD" | "POST" | "PUT" | "DELETE" | "CONNECT" | "OPTIONS" | "TRACE" | "PATCH"
    )
  ))  -> String {
    input
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use proptest::prelude::*;

  proptest! {
    #[test]
    fn request_verb_ok(verb in request_verb()) {
      assert!(
        matches!(
          verb.as_str(),
          "GET" | "HEAD" | "POST" | "PUT" | "DELETE" | "CONNECT" | "OPTIONS" | "TRACE" | "PATCH"
        )
      );
    }
  }

  proptest! {
    #[test]
    fn request_verb_wrong_case_ok(verb in request_verb_wrong_case()) {
      assert!(
        !matches!(
          verb.as_str(),
          "GET" | "HEAD" | "POST" | "PUT" | "DELETE" | "CONNECT" | "OPTIONS" | "TRACE" | "PATCH"
        ),
        "should have the wrong case but got {verb}"
      );
      let upper_case_verb = verb.to_uppercase();
      assert!(
        matches!(
          upper_case_verb.as_str(),
          "GET" | "HEAD" | "POST" | "PUT" | "DELETE" | "CONNECT" | "OPTIONS" | "TRACE" | "PATCH",
        ),
        "upper case version of {verb:?} should be correct but got {upper_case_verb:?}"
      );
    }
  }

  proptest! {
    #[test]
    fn request_verb_wrong_ok(input in request_verb_wrong()) {
      assert!(
        !matches!(
          input.as_str(),
          "GET" | "HEAD" | "POST" | "PUT" | "DELETE" | "CONNECT" | "OPTIONS" | "TRACE" | "PATCH"
        ),
        "expected a non-HTTP verb but got {input}"
      );
    }
  }
}
