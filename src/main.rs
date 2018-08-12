extern crate glob;

use glob::glob;
use std::path::Path;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("mgit v0.1.0\n");

    let path: &str = match args.get(1) {
        None => "./",
        Some(s) => s,
    };

    repos_git(path);
}

fn repos_git(path: &str) -> () {
    println!("Buscando en {}", path);

    let path = Path::new(path).join("*/.git");
    let path: &str = match path.to_str() {
        None => panic!("La ruta no es utf8"),
        Some(s) => s,
    };

    for entry in glob(path).unwrap() {
        match entry {
            Ok(path) => println!("{:?}", path.parent().unwrap()),
            Err(e) => println!("{:?}", e),
        }
    }
}
