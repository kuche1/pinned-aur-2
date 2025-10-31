mod arg;
mod aur;

use colored::Colorize;

fn main() {
    let search_package = arg::parse();

    println!("Searching for: {}", search_package.cyan());
    let found_packages = aur::search(&search_package);

    println!();
    for (package_idx, package) in found_packages.iter().enumerate().rev() {
        println!("{}) {}", package_idx.to_string().purple(), package);
    }
}
