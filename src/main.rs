mod arg;
mod aur;

fn main() {
    let package = arg::parse();
    aur::search(&package);
}
