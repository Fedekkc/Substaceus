use std::fs::File;
use std::io::{BufRead, BufReader};
use trust_dns_resolver::TokioAsyncResolver;
use trust_dns_resolver::config::*;
use colored::*;
use futures::stream::{self, StreamExt};

pub struct SubdomainFinder {
    resolver: TokioAsyncResolver,
}

impl SubdomainFinder {
    pub async fn new() -> Self {
        let resolver = TokioAsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default());
        SubdomainFinder { resolver }
    }

    pub async fn find(&self, reader: BufReader<File>, domain: &str) -> Vec<String> {
        let lines: Vec<_> = reader.lines().filter_map(Result::ok).collect();

        let results = stream::iter(lines)
            .map(|sub| {
                let domain = domain.to_string();
                let resolver = self.resolver.clone(); // esto se hace para que cada tarea tenga su propio resolver
                async move { //esto es un tipo de lambda asíncrona
                    let fqdn = format!("{}.{}", sub, domain);
                    if resolver.lookup_ip(&fqdn).await.is_ok() {
                        println!("{}.{}", sub.bold().green(), domain.bold().blue());
                        Some(fqdn)
                    } else {
                        None
                    }
                }
            })
            .buffer_unordered(100) // este numero se puede modificar pero no tiene q estar muy alto para no saturar el DNS
            .filter_map(async move |res| res) // aca filtro los resultados que son None
            .collect::<Vec<_>>()
            .await;

        results
    }
}
