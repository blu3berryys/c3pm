use crate::generator::{configure_cmake_project, generate_project};
use crate::model::Generator;
use crate::model::{Language, ProjectConfig};
use inflector::Inflector;
use lazy_static::lazy_static;
use std::path::PathBuf;
use std::process::Command as ACommand;
use std::str::FromStr;
use std::{
    env::current_dir,
    fs,
    fs::read_to_string,
    io,
    io::{Error, ErrorKind},
    path::Path,
    process::Command,
    thread,
};

pub mod generator;
pub mod impls;
pub mod model;

lazy_static! {
    pub static ref AVAILABLE_THREADS: usize = {
        thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1)
    };
}

pub fn get_current_path() -> io::Result<String> {
    current_dir().and_then(|p| {
        p.into_os_string()
            .into_string()
            .map_err(|_| Error::new(ErrorKind::Other, "Failed to convert OsString to String"))
    })
}

pub fn read_file_to_lines(path: &str) -> io::Result<Vec<String>> {
    let mut lines: Vec<String> = Vec::new();
    let file_contents = read_to_string(path)?;

    for line in file_contents.lines() {
        lines.push(line.to_string());
    }

    Ok(lines)
}

pub fn read_file_to_string(path: &str) -> io::Result<String> {
    read_to_string(path)
}

pub fn create_dir<T1, T2>(parent: T1, name: T2) -> io::Result<String>
where
    T1: AsRef<str>,
    T2: AsRef<str>,
{
    let path = format!("{}/{}", parent.as_ref(), name.as_ref());
    fs::create_dir(&path)?;

    Ok(path)
}

pub fn get_cmake_version() -> Result<String, Error> {
    let output = Command::new("cmake").arg("--version").output()?;
    let output_str = String::from_utf8_lossy(&output.stdout);

    output_str
        .lines()
        .next()
        .and_then(|line| line.strip_prefix("cmake version "))
        .map(|version| version.trim().to_string())
        .ok_or_else(|| Error::new(ErrorKind::Other, "Failed to get CMake version"))
}

pub fn is_c3pm_project(path: &str) -> io::Result<bool> {
    let c3pm_project_base_files: Vec<String> = vec![
        ".cpppm.toml".to_string(),
        "CMakeLists.txt".to_string(),
        "src".to_string(),
        ".git".to_string(),
    ];

    match dir_has_entries(path, c3pm_project_base_files)
        .expect("Directory is a not valid c3pm project")
    {
        true => Ok(true),
        false => Ok(false),
    }
}

pub fn dir_has_entries(directory: &str, entries: Vec<String>) -> Result<bool, Error> {
    let dir_path = Path::new(directory);

    if !dir_path.is_dir() {
        return Err(Error::new(ErrorKind::NotFound, "Directory not found"));
    }

    for entry in entries {
        if !dir_path.join(&entry).exists() {
            return Ok(false);
        }
    }

    Ok(true)
}

pub fn create_new_project(
    name: String,
    generator: Option<Generator>,
    language: Language,
    folder: Option<String>,
) -> Result<Result<(), String>, String> {
    let folder_name = match folder {
        Some(folder) => folder,
        None => name.clone(),
    };

    let current_dir = get_current_path().map_err(|e| e.to_string())?;
    let dir = create_dir(&current_dir, &folder_name).map_err(|e| e.to_string())?;

    match generator {
        Some(generator) => {
            generate_project(dir, name, Some(generator), language).map_err(|e| e.to_string())?;
        }
        None => generate_project(dir, name, None, language).map_err(|e| e.to_string())?,
    }

    Ok(Ok(()))
}

pub fn build_project(
    jobs: &usize,
    config: &String,
    generator: Option<Generator>,
) -> Result<(), String> {
    let current_dir = get_current_path().map_err(|e| e.to_string())?;
    let config_path = Path::new(&current_dir).join(".cpppm.toml");

    configure_cmake_project(&current_dir, generator).expect("Failed to configure cmake project");

    let project_config = load_project_config(&config_path)?;
    let build_dir = project_config
        .get_build_dir()
        .unwrap_or_else(|| "build".to_string());
    let build_dir_str = build_dir.as_str();

    let output = Command::new("cmake")
        .args(&[
            "--build",
            build_dir_str,
            "--parallel",
            &jobs.to_string(),
            "--config",
            &config.to_string(),
        ])
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    println!("{}", String::from_utf8_lossy(&output.stdout));

    move_built_object_files(config, &current_dir, &build_dir_str.to_string())?;

    Ok(())
}

pub fn clean_project() -> io::Result<()> {
    let current_dir = get_current_path()?;
    let config_path = Path::new(&current_dir).join(".cpppm.toml");
    let project_cfg = load_project_config(&config_path).unwrap();
    let build_dir = project_cfg.get_build_dir().expect("Fuck");
    let target_path = Path::new(&"target");
    let build_path = Path::new(&build_dir);

    if build_path.exists() {
        fs::remove_dir_all(build_path)?;
    }

    if target_path.exists() {
        fs::remove_dir_all(target_path)?;
    }

    Ok(())
}

pub fn move_built_object_files(
    config: &String,
    current_dir: &String,
    build_dir: &String,
) -> Result<(), String> {
    let valid_extensions = vec!["exe", "so", "dll", "pdb", "a", "o", "lib", "dylib"];
    let config_snake_case = config.to_string().to_pascal_case();
    let _build_config_dir = Path::new(&build_dir);
    let target_dir = Path::new(&current_dir)
        .join("target")
        .join(config_snake_case);

    fs::create_dir_all(&target_dir).map_err(|e| e.to_string())?;

    let mut moved_files: Vec<PathBuf> = Vec::new();
    let entries = walkdir::WalkDir::new(build_dir.clone());

    for entry in entries.into_iter().filter_map(|e| e.ok()) {
        let file_name = entry.file_name();

        // if file_name.to_str().unwrap().contains(current_dir.as_str()) {
        if let Some(extension) = entry.clone().path().extension() {
            if valid_extensions.contains(&extension.to_str().unwrap()) {
                let source_path = entry.path();
                let dest_path = target_dir.join(&file_name);

                fs::rename(&source_path, &dest_path)
                    .map_err(|e| format!("Failed to move file: {}", e))?;

                moved_files.push(dest_path);
            }
        }
        // }
    }

    if moved_files.is_empty() {
        return Err("No valid output files found to move.".to_string());
    }

    for moved_file in moved_files {
        println!("Moved file: {}", moved_file.display());
    }

    Ok(())
}

pub fn load_project_config(cfg_path: &Path) -> Result<ProjectConfig, String> {
    if !cfg_path.exists() {
        return Err(format!("Config not found at {}", cfg_path.display()));
    }

    let cfg_str = read_to_string(cfg_path).map_err(|e| format!("Error reading config: {}", e))?;
    toml::de::from_str(&cfg_str).map_err(|e| format!("Error parsing config file: {}", e))
}

pub fn init_project_subcommand(
    name: Option<String>,
    generator: Option<Generator>,
    language: Language,
) -> Result<(), String> {
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

    create_new_project(
        project_name,
        generator,
        language,
        Some(current_dir.to_str().unwrap().to_string()),
    )??;
    Ok(())
}

pub fn reconfigure_project_subcommand(generator: Option<Generator>) {
    clean_project().expect("fuck");

    match generator {
        Some(gen) => configure_cmake_project(&get_current_path().expect("fuck"), Some(gen)),
        None => configure_cmake_project(&get_current_path().unwrap(), None),
    }
    .expect("owo");
}

pub fn parse_language(lang: &str) -> Result<Language, String> {
    let input: Vec<&str> = lang.split(':').collect();
    let lang = input[0];
    let language = Language::from_str(lang)?;
    //let standard = input.get(1);
    let supported_langs = if language.is_c() {
        vec![/* "c89", */ "c99", "c11", "c17", "c23"]
    } else {
        vec!["cpp98", "cpp11", "cpp14", "cpp17", "cpp20", "cpp23"]
    };

    if supported_langs[0] == lang {
        // let _standard = standard.map(|t| *t).unwrap_or("23");
        let standard = Language::from_str(lang);
        return Ok(standard?);
    }

    if supported_langs[1..].contains(&lang) {
        //let standard = standard.map(|t| *t).unwrap_or("23");
        let standard = Language::from_str(lang);
        return Ok(standard?);
    }

    let formatted_possible_values = supported_langs.join(", ");
    Err(format!(
        "Possible values are {:?}",
        formatted_possible_values
    ))
}

fn is_cmd_success(cmd: &str) -> bool {
    ACommand::new(cmd).output().is_ok()
}

pub fn select_compilers() -> (String, String) {
    if is_cmd_success("clang") && is_cmd_success("clang++") {
        ("clang".to_string(), "clang++".to_string())
    } else if is_cmd_success("gcc") && is_cmd_success("g++") {
        ("gcc".to_string(), "g++".to_string())
    } else {
        (String::new(), String::new())
    }
}
