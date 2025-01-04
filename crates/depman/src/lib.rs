use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::process::{Command, Output};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Host {
    Github,
    Gitlab,
    Codeberg,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DependencyRepo {
    pub host: Option<Host>,
    pub owner: String,
    pub name: String,
    pub version: String,
}

impl Display for Host {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match &self {
            Host::Github => "https://github.com/".to_string(),
            Host::Gitlab => "https://gitlab.com/".to_string(),
            Host::Codeberg => "https://codeberg.com/".to_string(),
        };

        write!(f, "{}", str)
    }
}

impl DependencyRepo {
    pub fn new(host: Option<Host>, owner: String, name: String, version: String) -> DependencyRepo {
        DependencyRepo {
            host,
            owner,
            name,
            version,
        }
    }

    pub fn default() -> DependencyRepo {
        DependencyRepo {
            host: Some(Host::Github),
            ..Default::default()
        }
    }
}

fn cmd_exists(exe: &str) -> bool {
    !Command::new(exe).get_program().is_empty()
}

pub fn run_if_which<T, U>(exec: &str, code: T, alt_code: U)
where
    T: FnOnce(),
    U: FnOnce(),
{
    match cmd_exists(exec) {
        true => code(),
        false => alt_code(),
    }
}

pub fn git_switch(branch: &str, path: &str) -> std::io::Result<()> {
    run_if_which(
        "git-switch",
        move || {
            Command::new("git")
                .current_dir(path)
                .arg("switch")
                .arg(branch)
                .output()
                .expect("fuck");
        },
        move || println!("fuck"),
    );
    
    Ok(())
}
