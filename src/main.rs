use std::net::{IpAddr, Ipv4Addr};
use trust_dns_resolver::Resolver;
use trust_dns_resolver::config::*;

fn main() {
    println!("{}", get_public_ip())
}

fn get_public_ip() -> String {
    let query = "o-o.myaddr.l.google.com.";
    let config = ResolverConfig::from_parts(
        None,
        vec![],
        NameServerConfigGroup::from_ips_clear(
            &[
                IpAddr::V4(Ipv4Addr::new(216, 239, 32, 10)),
                IpAddr::V4(Ipv4Addr::new(216, 239, 34, 10)),
            ],
            53,
            false,
        ),
    );

    let resolver = Resolver::new(config, ResolverOpts::default()).unwrap();
    match resolver.txt_lookup(query) {
        Err(_) => String::from("0.0.0.0"),
        Ok(response) => response.iter().next().unwrap().to_string()
    }
}
