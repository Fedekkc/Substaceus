use std::io::BufRead;
use trust_dns_resolver::{Resolver, config::*};
use colored::*;

pub struct SubdomainFinder {
    resolver: Resolver,
}

impl SubdomainFinder {
    pub fn new() -> Self {
        let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default())
            .expect("Could not create DNS resolver");
        SubdomainFinder { resolver }
    }

    pub fn find<B: BufRead>(&self, reader: B, domain: &str) -> Vec<String> {
        let mut results = Vec::new();

        for line in reader.lines() {
            if let Ok(sub) = line {
                let fqdn = format!("{}.{}", sub, domain);
                if self.resolver.lookup_ip(&fqdn).is_ok() {
                    print!("{}.{} \n", sub.bold().green(), domain.bold().blue());
                    results.push(fqdn);
                }
            }
        }

        results
    }
}
