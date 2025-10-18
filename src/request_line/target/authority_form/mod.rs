//! HTTP request target in authority form strategies.

use std::net::{Ipv4Addr, Ipv6Addr};

use proptest::{
  prelude::{Strategy, any},
  prop_oneof,
};

pub mod domain;
pub mod ip_v4;
pub mod ip_v6;

/// strategy for generating IP v6 hosts.
///
/// IP v6 host is just an IP v6 wrapped in square brackets.
///
/// # Returns
/// `Ipv6Addr` and its host representation.
pub fn ip_v6_host() -> impl Strategy<Value = (Ipv6Addr, String)> {
  ip_v6::ip_v6().prop_map(move |(ip, repr)| (ip, format!("[{repr}]")))
}

/// URI host variants.
#[derive(Debug)]
pub enum UriHost {
  Domain(String),
  Ipv6(Ipv6Addr, String),
  Ipv4(Ipv4Addr, String),
}

/// strategy for generating URI hosts.
///
/// # Arguments
/// * `max_label_count`: maximum label count to use for domain hosts.
pub fn uri_host(max_label_count: usize) -> impl Strategy<Value = UriHost> {
  prop_oneof![
    domain::domain(max_label_count).prop_map(UriHost::Domain),
    ip_v4::ip_v4().prop_map(|(ip, repr)| UriHost::Ipv4(ip, repr)),
    ip_v6_host().prop_map(|(ip, repr)| UriHost::Ipv6(ip, repr))
  ]
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
pub fn authority(max_label_count: usize) -> impl Strategy<Value = (UriHost, u16, String)> {
  (uri_host(max_label_count), any::<u16>()).prop_map(move |(host, port)| {
    let host_repr = match &host {
      UriHost::Domain(repr) | UriHost::Ipv6(_, repr) | UriHost::Ipv4(_, repr) => repr,
    };
    let repr = format!("{host_repr}:{port}");
    (host, port, repr)
  })
}
