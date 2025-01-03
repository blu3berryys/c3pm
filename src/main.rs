#![allow(dead_code)]

use crate::cli::{C3pmArgs, NewSubcmd};
use crate::util::get_current_path;
use clap::Parser;
use std::path::PathBuf;
use std::str::FromStr;
mod cli;
mod config_parser;
mod generator;
mod model;
mod util;

fn main() -> Result<(), String> {
    let args = C3pmArgs::parse();
    
    match args.new_cmd {
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

            //           generate_project(
            //               current_dir.to_str().unwrap().to_string(),
            //               project_name,
            //               generator,
            //               language,
            //           )
            //           .map_err(|e| e.to_string())?;

            util::create_new_project(
                project_name,
                generator,
                language,
                Some(current_dir.to_str().unwrap().to_string()),
            )??;

            Ok(())
        }
        NewSubcmd::Build { jobs, config } => util::build_project(&jobs, &config, None),
    }
}
