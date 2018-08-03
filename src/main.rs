//use glob::glob;
//use std::result::Result;

struct Repo {
    path: String
}

fn main() {
    let repos = listar_directorios_git(".".to_string());

    while let Some(repo) = repos.pop() {
        println!("{}", repo.path);
    }

    println!("{}", repos.len());
}

fn listar_directorios_git(path: String) -> Vec<Repo> {
    let mut repos = Vec::new();

    repos.push(Repo{path: "mgit-rs".to_string()});

    /*
    println!("{}", repos.len());

    for path in glob("/.git").unwrap().filter_map(Result::ok) {
        println!("{}", path.display());
    }
    */

    return repos;
}

#[test]
fn test_listar_directorios_git() {
    //assert_eq!(listar_directorios_git("."), 2);
}
