use crate::cli::{C3pmArgs, NewSubcmd};
use crate::generator::generate_project;
use crate::util::{create_dir, get_current_path};
use clap::Parser;
use std::path::PathBuf;
use std::str::FromStr;

mod cli;
mod generator;
mod model;
mod util;

fn main() -> Result<(), String> {
    let args = C3pmArgs::parse();

    match args.new_cmd {
        NewSubcmd::New {
            name,
            language,
            folder,
        } => {
            let folder_name = match folder {
                Some(folder) => folder,
                None => name.clone(),
            };

            let current_dir = get_current_path().map_err(|e| e.to_string())?;
            let dir = create_dir(&current_dir, &folder_name).map_err(|e| e.to_string())?;

            generate_project(dir, name, language).map_err(|e| e.to_string())?;

            Ok(())
        }
        NewSubcmd::Init { name, language } => {
            let current_dir =
                PathBuf::from_str(get_current_path().map_err(|e| e.to_string())?.as_str());

            let project_name = match name {
                Some(name) => name,
                None => current_dir
                    .clone()
                    .unwrap()
                    .file_name()
                    .and_then(|n| n.to_str())
                    .ok_or_else(|| "Could not determine project name from directory".to_string())?
                    .to_string(),
            };

            generate_project(
                current_dir.expect("Fuck").to_str().unwrap().to_string(),
                project_name,
                language,
            )
            .map_err(|e| e.to_string())?;

            Ok(())
        }
    }
}
