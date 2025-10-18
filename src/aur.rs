use reqwest::blocking::Client; // cargo add reqwest --features blocking // cargo add reqwest --features json
use serde::Deserialize;

const URL: &str = "https://aur.archlinux.org/rpc";

#[derive(Deserialize)]
struct AurResponse {
    #[serde(rename = "version")]
    _version: i32, // TODO: check if this matches the requested API version and print a warning if it does not
    #[serde(rename = "type")]
    _response_type: String,
    #[serde(rename = "resultcount")]
    _resultcount: i32,
    results: Vec<AurPackage>,
}

#[derive(Deserialize)]
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
    println!("Searching for: {package}");

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

    println!("Found {} results:", aur_data._resultcount);
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
