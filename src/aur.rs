use reqwest::blocking::Client; // cargo add reqwest --features blocking // cargo add reqwest --features json
use serde::Deserialize;

const URL: &str = "https://aur.archlinux.org/rpc";

#[derive(Debug, Deserialize)]
struct AurResponse {
    version: i32,
    #[serde(rename = "type")]
    response_type: String,
    resultcount: i32,
    results: Vec<AurPackage>,
}

#[derive(Debug, Deserialize)]
struct AurPackage {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Version")]
    version: String,
    #[serde(rename = "Description")]
    description: Option<String>,
    #[serde(rename = "Maintainer")]
    maintainer: Option<String>,
}

pub fn search(package: &str) {
    println!("searching for: {package}");

    let params = [
        ("v", "5"), // api version
        ("type", "search"),
        ("arg", package),
    ];

    let client = Client::new();
    let response = client
        .get(URL)
        .query(&params)
        .send()
        .expect("could not get data from AUR");

    let aur_data: AurResponse = response.json().expect("could not parse AUR response");

    println!("Found {} results:", aur_data.resultcount);
    for pkg in aur_data.results.iter().take(5) {
        println!(
            "{} {} - {} (maintainer: {})",
            pkg.name,
            pkg.version,
            pkg.description.as_deref().unwrap_or("No description"),
            pkg.maintainer.as_deref().unwrap_or("None")
        );
    }
}
