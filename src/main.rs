mod arg;
mod aur;

fn main() {
    let search_package = arg::parse();

    let found_packages = aur::search(&search_package);

    for package in found_packages {
        println!("{}", package);
    }
}
