use std::net::IpAddr;
use std::str::FromStr;

use error::SdpParserInternalError;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SdpAddrType {
    IP4 = 4,
    IP6 = 6,
}

impl SdpAddrType {
    pub fn same_protocol(self, addr: &IpAddr) -> bool {
        (addr.is_ipv6() && self == SdpAddrType::IP6) || (addr.is_ipv4() && self == SdpAddrType::IP4)
    }
}

pub fn addr_to_string(addr: IpAddr) -> String {
    match addr {
        IpAddr::V4(ipv4) => format!("IN IP4 {}", ipv4.to_string()),
        IpAddr::V6(ipv6) => format!("IN IP6 {}", ipv6.to_string()),
    }
}

pub fn parse_nettype(value: &str) -> Result<(), SdpParserInternalError> {
    if value.to_uppercase() != "IN" {
        return Err(SdpParserInternalError::Generic(
            "nettype needs to be IN".to_string(),
        ));
    };
    Ok(())
}

#[test]
fn test_parse_nettype() {
    let internet = parse_nettype("iN");
    assert!(internet.is_ok());

    assert!(parse_nettype("").is_err());
    assert!(parse_nettype("FOO").is_err());
}

pub fn parse_addrtype(value: &str) -> Result<SdpAddrType, SdpParserInternalError> {
    Ok(match value.to_uppercase().as_ref() {
        "IP4" => SdpAddrType::IP4,
        "IP6" => SdpAddrType::IP6,
        _ => {
            return Err(SdpParserInternalError::Generic(
                "address type needs to be IP4 or IP6".to_string(),
            ));
        }
    })
}

#[test]
fn test_parse_addrtype() {
    let ip4 = parse_addrtype("iP4");
    assert!(ip4.is_ok());
    assert_eq!(ip4.unwrap(), SdpAddrType::IP4);
    let ip6 = parse_addrtype("Ip6");
    assert!(ip6.is_ok());
    assert_eq!(ip6.unwrap(), SdpAddrType::IP6);

    assert!(parse_addrtype("").is_err());
    assert!(parse_addrtype("IP5").is_err());
}

pub fn parse_unicast_addr(value: &str) -> Result<IpAddr, SdpParserInternalError> {
    Ok(IpAddr::from_str(value)?)
}

#[test]
fn test_parse_unicast_addr() {
    let ip4 = parse_unicast_addr("127.0.0.1");
    assert!(ip4.is_ok());
    let ip6 = parse_unicast_addr("::1");
    assert!(ip6.is_ok());
}
