use git2::Repository;
use std::io;

pub fn get_conflicted_files() -> io::Result<Vec<String>> {
    let repo = Repository::open(".").expect("Failed to open repository");
    let statuses = repo.statuses(None).expect("Failed to get repository status");

    let conflicts: Vec<String> = statuses.iter()
        .filter(|entry| entry.status().contains(git2::Status::CONFLICTED))
        .filter_map(|entry| entry.path().map(String::from))
        .collect();

    Ok(conflicts)
}

