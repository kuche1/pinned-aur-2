use colored::Colorize;
use reqwest::blocking::Client; // cargo add reqwest --features blocking // cargo add reqwest --features json
use serde::Deserialize;
use std::fmt::Display;

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

// for example:
// {'Description': 'Zoom VDI VMWare plugin', 'FirstSubmitted': 1706807860, 'ID': 1528188, 'LastModified': 1724630068, 'Maintainer': 'vachicorne', 'Name': 'zoom-vmware-plugin', 'NumVotes': 0, 'OutOfDate': None, 'PackageBase': 'zoom-vmware-plugin', 'PackageBaseID': 202104, 'Popularity': 0, 'URL': 'https://support.zoom.us/hc/en-us/articles/4415057249549-VDI-releases-and-downloads', 'URLPath': '/cgit/aur.git/snapshot/zoom-vmware-plugin.tar.gz', 'Version': '6.0.10-1'}
#[derive(Deserialize)]
pub struct AurPackage {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Description")]
    description: Option<String>,
    #[serde(rename = "Version")]
    version: String,
    #[serde(rename = "NumVotes")]
    num_votes: u32,
    #[serde(rename = "Popularity")]
    popularity: f32,
    // #[serde(rename = "Maintainer")]
    // maintainer: Option<String>,
}

impl Display for AurPackage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ", self.name.cyan())?;
        write!(
            f,
            "[{}/{}] ",
            format!("{}", self.num_votes).green(),
            format!("{:.2}", self.popularity).green(),
        )?;
        write!(f, "{{{}}}", self.version.yellow())?;
        writeln!(f)?;

        match &self.description {
            Some(desc) => writeln!(f, "{}", desc)?,
            None => writeln!(f, "No Description")?,
        }

        Ok(())
    }
}

pub fn search(package: &str) -> Vec<AurPackage> {
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

    // println!("Found {} results:", aur_data._resultcount);
    // for pkg in aur_data.results.iter().take(5) {
    //     println!(
    //         "{} {} - {}", // "{} {} - {} (maintainer: {})",
    //         pkg.name,
    //         pkg.version,
    //         pkg.description.as_deref().unwrap_or("No description"),
    //         // pkg.maintainer.as_deref().unwrap_or("None")
    //     );
    // }

    let mut packages = aur_data.results;

    packages.sort_by(|a, b| a.num_votes.cmp(&b.num_votes));

    packages.reverse();

    return packages;
}
