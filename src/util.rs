use crate::generator::{configure_cmake_project, generate_project};
use crate::model::{BuildConfig, Generator, Language, ProjectConfig};
use inflector::Inflector;
use lazy_static::lazy_static;
use std::env::current_dir;
use std::io::{Error, ErrorKind};
use std::path::Path;
use std::process::Command;
use std::{fs, io, thread};
use std::fs::read_to_string;

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
    let file_contents = fs::read_to_string(path)?;

    for line in file_contents.lines() {
        lines.push(line.to_string());
    }

    Ok(lines)
}

pub fn read_file_to_string(path: &str) -> io::Result<String> {
    fs::read_to_string(path)
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
        "c3pm.toml".to_string(),
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

pub fn build_project(jobs: &usize, config: &BuildConfig, generator: Option<Generator>) -> Result<(), String> {
    let current_dir = get_current_path().map_err(|e| e.to_string())?;
    let config_path = Path::new(&current_dir).join(".c3pm.toml");

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

    move_built_object_files(config, &current_dir, &build_dir)?;

    Ok(())
}

pub fn move_built_object_files(
    config: &BuildConfig,
    current_dir: &String,
    build_dir: &String,
) -> Result<(), String> {
    let config_snake_case = config.to_string().to_snake_case();
    let build_config_dir = Path::new(&build_dir).join(config.to_string());
    let target_dir = Path::new(&current_dir)
        .join("target")
        .join(config_snake_case);

    fs::create_dir_all(&target_dir).map_err(|e| e.to_string())?;

    let mut moved_files = Vec::new();

    if let Ok(entries) = fs::read_dir(build_config_dir) {
        for entry in entries.filter_map(Result::ok) {
            let file_name = entry.file_name();
            if let Some(extension) = entry.path().extension() {
                let valid_extensions = vec!["exe", "so", "dll", "pdb", "a", "o", "lib", "dylib"];

                if valid_extensions.contains(&extension.to_str().unwrap()) {
                    let source_path = entry.path();
                    let dest_path = target_dir.join(file_name);

                    fs::rename(&source_path, &dest_path)
                        .map_err(|e| format!("Failed to move file: {}", e))?;

                    moved_files.push(dest_path);
                }
            }
        }
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