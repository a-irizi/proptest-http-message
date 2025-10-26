//! HTTP request target in authority form strategies.

use proptest::prelude::{Strategy, any};

use crate::request_line::target::components::host::{Host, host};

/// URL authority form components
#[derive(Debug)]
pub struct AuthorityForm {
  pub host: Host,
  pub port: u16,
}

/// strategy for generating target authority form.
///
/// target authority is composed of a host and a port separated by colon.
///
/// # Arguments
/// * `max_label_count`: maximum label count to use for domain hosts.
///
/// # Returns
/// `UriHost`, port and their authority representation.
pub fn authority(max_label_count: usize) -> impl Strategy<Value = (AuthorityForm, String)> {
  (host(max_label_count), any::<u16>()).prop_map(move |(host, port)| {
    let host_repr = match &host {
      Host::Domain(repr) | Host::Ipv6(_, repr) | Host::Ipv4(_, repr) => repr,
    };
    let repr = format!("{host_repr}:{port}");
    (AuthorityForm { host, port }, repr)
  })
}

#[cfg(test)]
pub(super) mod tests {
  use claims::assert_ok;
  use proptest::proptest;
  use url::{Host, Url};

  use super::*;

  pub(in super::super) fn authority_asserts(authority_form: &AuthorityForm, repr: &str) {
    let url = assert_ok!(Url::parse(&format!("http://{repr}")));

    match (&authority_form.host, url.host()) {
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
      _ => panic!("expected host {:?} but parsed {:?}", authority_form.host, url.host()),
    }

    if let Some(port) = url.port() {
      assert_eq!(
        authority_form.port, port,
        "expected port {} but parsed port {}",
        authority_form.port, port
      );
    } else {
      assert_eq!(
        80, authority_form.port,
        "expected default HTTP port 80 but got {}",
        authority_form.port
      );
    }
  }

  proptest! {
    #[test]
    fn authority_works((authority_form, repr) in authority(20)) {
      authority_asserts(&authority_form, &repr);
    }
  }
}
