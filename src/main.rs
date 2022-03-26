use std::net::{IpAddr, Ipv4Addr};
use trust_dns_resolver::Resolver;
use trust_dns_resolver::error::ResolveResult;
use trust_dns_resolver::{config::*, lookup::TxtLookup};

fn main() {
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
            false),
    );

    let resolver = Resolver::new(config, ResolverOpts::default()).unwrap();
    let response = resolver.txt_lookup(query);
    display_txt(&response);
}

fn display_txt(response: &ResolveResult<TxtLookup>) {
    match response {
        Err(_) => println!("No Records."),
        Ok(resp) => {
            for record in resp.iter() {
                println!("{}", record.to_string());
            }
        }
    }
}