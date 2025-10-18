//! HTTP request target in authority form strategies.

use proptest::prelude::{Strategy, any};

use crate::request_line::target::host::{Host, host};

/// strategy for generating target authority form.
///
/// target authority is composed of a host and a port separated by colon.
///
/// # Arguments
/// * `max_label_count`: maximum label count to use for domain hosts.
///
/// # Returns
/// `UriHost`, port and their authority representation.
pub fn authority(max_label_count: usize) -> impl Strategy<Value = (Host, u16, String)> {
  (host(max_label_count), any::<u16>()).prop_map(move |(host, port)| {
    let host_repr = match &host {
      Host::Domain(repr) | Host::Ipv6(_, repr) | Host::Ipv4(_, repr) => repr,
    };
    let repr = format!("{host_repr}:{port}");
    (host, port, repr)
  })
}
