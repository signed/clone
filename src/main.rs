use std::env;
use dirs;
use url::{Url};
use std::path::{PathBuf};
use substring::Substring;
use git2::Repository;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let repository_url = args.get(1).ok_or("clone https://github.com/octocat/Hello-World.git")?;
    let url = Url::parse(repository_url)?;
    let hoster = match url.host_str().ok_or("no host string")? {
        "github.com" => "github",
        _ => return Ok(()) //todo return an error instead
    };

    let segments = url.path_segments().map(|c| c.collect::<Vec<_>>()).ok_or("url has no path")?;
    let user_name_or_organisation = *segments.get(0).ok_or("url has no user name or organisation")?;
    let mut repository = *segments.get(1).ok_or("url has no repository name")?;
    repository = match repository.ends_with(".git") {
        false => repository,
        true => repository.substring(0, repository.len()-4),
    };

    let home_dir = dirs::home_dir().ok_or("home directory not found")?;
    let home: &str = home_dir.to_str().ok_or("home dir contains none unicode characters")?;
    let local_destination: PathBuf = [home, "dev", hoster, user_name_or_organisation, repository].iter().collect();

    if local_destination.is_dir() {
        println!("local directory '{}' already exists", local_destination.display());
        return Ok(())
    }

    println!("{}", local_destination.display());

    Repository::clone(repository_url, local_destination)?;
    Ok(())
}
