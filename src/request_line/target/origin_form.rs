//! HTTP request target in origin form strategies.

use std::{num::NonZero, ops::RangeInclusive};

use proptest::prelude::Strategy;

use super::components::{
  path::{Path, path_absolute},
  query::{QueryParam, query},
};

/// URL origin form components
#[derive(Debug)]
pub struct OriginForm {
  pub path: Path,
  pub query: Option<Vec<QueryParam>>,
}

/// strategy for generating target origin form.
/// # Returns
/// [`OriginForm`] and its representation.
pub fn origin(
  max_segments: NonZero<usize>,
  query_count_range: RangeInclusive<usize>,
) -> impl Strategy<Value = (OriginForm, String)> {
  (path_absolute(max_segments), query(*query_count_range.start(), *query_count_range.end()))
    .prop_map(|((path, path_repr), (query, query_repr))| {
      if query.is_empty() {
        (OriginForm { path, query: None }, path_repr)
      } else {
        let repr = format!("{path_repr}?{query_repr}");
        (OriginForm { path, query: Some(query) }, repr)
      }
    })
}

#[cfg(test)]
pub(super) mod tests {
  use std::sync::LazyLock;

  use claims::assert_ok;
  use proptest::{prelude::ProptestConfig, proptest};
  use url::Url;

  use super::*;

  const DUMMY_BASE_URL: &str = "https://example.com";
  static BASE_URL: LazyLock<Url> = LazyLock::new(|| {
    Url::parse(DUMMY_BASE_URL).unwrap_or_else(|_| panic!("{DUMMY_BASE_URL:?} is a valid base URL"))
  });

  pub(in super::super) fn origin_asserts(origin: &OriginForm, repr: &str) {
    let url = assert_ok!(BASE_URL.join(repr));

    assert_eq!(
      origin.path.normalized,
      url.path(),
      "expected path {:?} but parsed {}",
      origin.path.normalized,
      url.path()
    );

    match (origin.query.as_deref(), url.query()) {
      (None, None) => {}
      (Some(query), Some(query2)) => {
        let query = query
          .iter()
          .map(|query| format!("{}={}", query.key, query.value.as_deref().unwrap_or_default()))
          .collect::<Vec<_>>()
          .join("&");
        assert_eq!(query, query2, "expected query {query:?} but parsed {query2:?}");
      }
      (query, query2) => panic!("expected query {query:?} but got {query2:?}"),
    }
  }

  proptest! {
    #![proptest_config(ProptestConfig::with_cases(10_000))]
    #[test]
    fn origin_works((origin, repr) in origin(50.try_into().unwrap(), 0..=20)) {
      origin_asserts(&origin, &repr);
    }
  }
}
