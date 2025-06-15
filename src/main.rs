use subdomain_finder::{finder::SubdomainFinder, utils::open_wordlist};
use std::env;
use figlet_rs::FIGfont;
use colored::*;

fn watermark() {
    let font = FIGfont::from_file("fonts/Bloody.flf").unwrap();
    let banner = font.convert("Substaceus").unwrap();
    println!("{}", banner.to_string().cyan().bold());
    println!("Subdomain Finder by Fedekkc\n");
}

    

fn main() {
    watermark();
    
    let finder = SubdomainFinder::new();

    let domain = env::args()
        .nth(1)
        .expect("Usage: subdomain-finder <domain>");
    
    let wordlist_path = env::args()
        .nth(2)
        .unwrap_or_else(|| "subdomains.txt".to_string());



    let domain = domain.trim_end_matches('.');
    let wordlist = open_wordlist(&wordlist_path).expect("Failed to open wordlist file");

    let found = finder.find(wordlist, domain);
    for sub in found {
        println!("{}", sub.green()); // Colorea los resultados
    }
}
