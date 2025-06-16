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

    pub async fn find(&self, reader: BufReader<File>, domain: &str, export_type: &str) -> Vec<String> {
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
            
        if export_type == "json" {
            let json_results = serde_json::to_string(&results).expect("Failed to serialize results to JSON");
            std::fs::write("results.json", json_results).expect("Failed to write results to file");
            println!("Results exported to results.json");
        } else if export_type == "txt" {
            let txt_results = results.join("\n");
            std::fs::write("results.txt", txt_results).expect("Failed to write results to file");
            println!("Results exported to results.txt");
        } else if export_type == "none"
        {
            println!("Results not exported.");
        }
        else {
            println!("Unknown export type. Results not exported.");
        }
        results
        
    }
}
