use std::io::{self, Read};

use ql_wiki::{RegistryDisclosureProvider, WikiRefractionEngine, WikiRefractionRequest};

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let request: WikiRefractionRequest = serde_json::from_str(&input)?;
    let provider = RegistryDisclosureProvider::new();
    let response = WikiRefractionEngine::new(Some(&provider)).refract(&request)?;
    serde_json::to_writer_pretty(io::stdout().lock(), &response)?;
    println!();
    Ok(())
}
