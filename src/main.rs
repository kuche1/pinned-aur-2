mod arg;
mod aur_search;

fn main() {
    let package = arg::parse();
    aur_search::search(&package);
}
