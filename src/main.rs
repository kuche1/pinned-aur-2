mod arg;

fn main() {
    let package = arg::parse();
    println!("package: {package}");
}
