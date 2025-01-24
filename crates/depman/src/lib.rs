use git2::ErrorCode::Exists;
use git2::Repository;
use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::path::Path;
use util::model::Dependency;

#[derive(Debug, Clone, Deserialize)]
pub struct Depman {
    #[serde(rename = "deps")]
    dependencies: Vec<Dependency>,
}

impl Depman {
    pub fn from_repo(repo_path: &Path) -> Result<Self, Box<dyn Error>> {
        let toml_path = repo_path.join(".c3pm.toml");
        let toml_contents = fs::read_to_string(toml_path)?;
        let depman: Depman = toml::from_str(&toml_contents)?;

        Ok(depman)
    }

    pub fn retrieve_dependencies(&self, base_path: &Path) -> Result<(), Box<dyn Error>> {
        let deps_dir = base_path.join("deps");
        fs::create_dir_all(&deps_dir)?;

        for dependency in &self.dependencies {
            let repo_url = format!(
                "https://{}/{}/{}.git",
                dependency
                    .clone()
                    .dependency
                    .host
                    .unwrap_or("github.com".to_string()),
                dependency.clone().dependency.repository.0,
                dependency.clone().dependency.repository.1
            );
            let dependency_name = &dependency.clone().dependency.name;
            let dependency_path = deps_dir.join(dependency_name);

            if let Err(e) = Repository::clone_recurse(repo_url.as_str(), &dependency_path) {
                if e.code() == Exists {
                    eprintln!("Repository {} already exists, skipping...", dependency_name);
                    continue;
                } else {
                    return Err(Box::new(e));
                }
            }

            let dep_repo = Repository::open(&dependency_path)?;
            let object = dep_repo.revparse_single(
                dependency
                    .clone()
                    .dependency
                    .revision
                    .unwrap_or("".to_string())
                    .as_str(),
            );
            dep_repo.checkout_tree(&object.unwrap(), None)?;

            let dep_deps = Depman::from_repo(&dependency_path)?;
            dep_deps.retrieve_dependencies(&dependency_path)?
        }

        Ok(())
    }
}

pub fn fetch_repository(url: &str, base_path: &Path) -> Result<Depman, Box<dyn Error>> {
    let repo_name = url
        .split('/')
        .last()
        .unwrap_or("repo")
        .trim_end_matches(".git");
    let repo_path = base_path.join("deps").join(repo_name);

    Repository::clone_recurse(url, &repo_path)?;

    let depman = Depman::from_repo(&repo_path)?;
    depman.retrieve_dependencies(&repo_path)?;

    Ok(depman)
}
