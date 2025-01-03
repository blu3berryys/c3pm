#![allow(dead_code, unused_variables)]

use crate::cli::{C3pmArgs, NewSubcmd};
use crate::util::get_current_path;
use clap::Parser;
use std::path::PathBuf;
use std::str::FromStr;

mod cli;
mod generator;
mod impls;
mod model;
mod util;

fn main() -> Result<(), String> {
    let args = C3pmArgs::parse();

    match args.subcommands {
        NewSubcmd::New {
            name,
            generator,
            language,
            folder,
        } => util::create_new_project(name, generator, language, folder)?,
        NewSubcmd::Init {
            name,
            generator,
            language,
        } => {
            let current_dir =
                PathBuf::from_str(get_current_path().map_err(|e| e.to_string())?.as_str()).unwrap();

            let project_name = match name {
                Some(name) => name,
                None => current_dir
                    .clone()
                    .file_name()
                    .and_then(|n| n.to_str())
                    .ok_or_else(|| "Could not determine project name from directory".to_string())?
                    .to_string(),
            };

            util::create_new_project(
                project_name,
                generator,
                language,
                Some(current_dir.to_str().unwrap().to_string()),
            )??;

            Ok(())
        }
        NewSubcmd::Build {
            jobs,
            config,
            generator,
        } => util::build_project(&jobs, &config, generator),
        NewSubcmd::Clean {} => Ok(util::clean_project().expect("fuck")),
    }
}
