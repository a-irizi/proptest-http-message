//! HTTP request target in absolute form strategies.

use std::{num::NonZero, ops::RangeInclusive};

use proptest::{option::of, prelude::Strategy};

use crate::request_line::target::components::{
  authority::{Authority, authority},
  fragment::fragment,
  path::{Path, path_absolute},
  query::{QueryParam, query},
  scheme::http_scheme,
};

/// URL absolute form components
#[derive(Debug)]
pub struct AbsoluteForm {
  pub scheme: String,
  pub authority: Authority,
  pub path: Option<Path>,
  pub query: Option<Vec<QueryParam>>,
  pub fragment: Option<String>,
}

/// strategy for generating target absolute form.
pub fn absolute(
  max_label_count: usize,
  max_segments: NonZero<usize>,
  query_count_range: RangeInclusive<usize>,
) -> impl Strategy<Value = (AbsoluteForm, String)> {
  (
    http_scheme(),
    authority(max_label_count),
    of(path_absolute(max_segments)),
    of(query(*query_count_range.start(), *query_count_range.end())),
    of(fragment()),
  )
    .prop_map(|(scheme, (authority, authority_repr), path, query, fragment)| {
      let repr = format!(
        "{scheme}://{authority}{path}{query}{fragment}",
        scheme = scheme,
        authority = authority_repr,
        path = if let Some(path) = path.as_ref() { path.1.as_str() } else { "" },
        query = if let Some((_, query)) = query.as_ref() { &format!("?{query}") } else { "" },
        fragment =
          if let Some(fragment) = fragment.as_ref() { &format!("#{fragment}") } else { "" }
      );

      (
        AbsoluteForm {
          scheme,
          authority,
          path: path.map(|p| p.0),
          query: query.map(|q| q.0),
          fragment,
        },
        repr,
      )
    })
}

#[cfg(test)]
pub(super) mod tests {
  use claims::{assert_none, assert_ok};
  use proptest::proptest;
  use url::{Host, Url};

  use super::*;

  pub(in super::super) fn absolute_asserts(absolute_form: &AbsoluteForm, repr: &str) {
    let url = assert_ok!(Url::parse(repr), "should be good URL but got {repr}");
    assert_eq!(absolute_form.scheme.to_ascii_lowercase(), url.scheme().to_ascii_lowercase());
    if let Some(user_info) = absolute_form.authority.user_info.as_ref() {
      assert_eq!(
        user_info.username,
        url.username(),
        "expected to get username {:?} but got {:?}",
        user_info.username,
        url.username()
      );
      assert_eq!(
        user_info.password.as_deref(),
        url.password(),
        "expected to get password {:?} but got {:?}",
        user_info.password,
        url.password()
      );
    } else {
      assert!(url.username().is_empty(), "username should be empty if user info is absent");
      assert_none!(url.password(), "password should not exist if user info is absent");
    }

    match (&absolute_form.authority.host, url.host()) {
      (
        crate::request_line::target::components::host::Host::Domain(domain),
        Some(Host::Domain(domain2)),
      ) => assert_eq!(
        domain.to_lowercase(),
        domain2.to_lowercase(),
        "expected domain {:?} but parsed domain {:?}",
        domain.to_lowercase(),
        domain2.to_lowercase()
      ),
      (
        crate::request_line::target::components::host::Host::Ipv6(ipv6_addr, _),
        Some(Host::Ipv6(ipv6_addr2)),
      ) => assert_eq!(
        *ipv6_addr, ipv6_addr2,
        "expected IP v6 {ipv6_addr:?} but parsed IP v6 {ipv6_addr:?}"
      ),
      (
        crate::request_line::target::components::host::Host::Ipv4(ipv4_addr, _),
        Some(Host::Ipv4(ipv4_addr2)),
      ) => assert_eq!(
        *ipv4_addr, ipv4_addr2,
        "expected IP v6 {ipv4_addr:?} but parsed IP v6 {ipv4_addr:?}"
      ),
      _ => panic!("expected host {:?} but parsed {:?}", absolute_form.authority.host, url.host()),
    }

    match (absolute_form.authority.port, url.port()) {
      (None | Some(80), None) => {}
      (Some(port), Some(port2)) => {
        assert_eq!(port, port2, "expected port but {port} parsed {port2}");
      }
      (port, port2) => panic!("expected port but {port:?} parsed {port2:?}"),
    }

    match (&absolute_form.path, url.path()) {
      (Some(path), path2) => {
        assert_eq!(path.normalized, path2, "expected path {path:?} but parsed {path2}");
      }
      (None, path) => {
        assert!(path.is_empty() || path == "/", "expected path to be empty but parsed {path}");
      }
    }

    match (absolute_form.query.as_deref(), url.query()) {
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

    match (absolute_form.fragment.as_deref(), url.fragment()) {
      (None, None) => {}
      (Some(fragment), Some(fragment2)) => {
        assert_eq!(fragment, fragment2, "expected fragment {fragment:?} but parsed {fragment2:?}");
      }
      (fragment, fragment2) => panic!("expected fragment {fragment:?} but parsed {fragment2:?}"),
    }
  }
  proptest! {
    #[test]
    fn absolute_works((absolute_form, repr) in absolute(20, 50.try_into().unwrap(), 0..=20)) {
      absolute_asserts(&absolute_form, &repr);
    }
  }
}
