use crate::model::Generator;
use crate::{model::CppStandard::Cpp23, model::Language};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::Error;
use std::path::Path;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ProjectDetails {
    pub name: String,
    pub generator: Option<Generator>,
    pub language: Language
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ProjectConfig {
    pub project_details: ProjectDetails,
    #[serde(default)]
    pub dirs: HashMap<String, String>,
}

impl Default for ProjectDetails {
    fn default() -> ProjectDetails {
        Self {
            name: String::new(),
            generator: None,
            language: Language::CPP(Cpp23)
        }
    }
}

impl Default for ProjectConfig {
    fn default() -> Self {
        let mut dirs = HashMap::new();
        dirs.insert("sources".to_string(), "src/".to_string());
        dirs.insert("headers".to_string(), "include/".to_string());
        dirs.insert("build".to_string(), "build/".to_string());

        Self {
            project_details: ProjectDetails::default(),
            dirs,
        }
    }
}

impl ProjectConfig {
    pub fn get_name(&self) -> &str {
        &self.project_details.name
    }

    pub fn get_language(&self) -> &str {
        match self.project_details.language {
            Language::CPP(_) => "cpp",
            Language::C(_) => "c",
        }
    }

    pub fn get_dir(&self, key: &str) -> Option<String> {
        self.dirs.get(key).cloned()
    }

    pub fn get_sources_dir(&self) -> Option<String> {
        self.dirs.get("sources").cloned()
    }

    pub fn get_headers_dir(&self) -> Option<String> {
        self.dirs.get("headers").cloned()
    }

    pub fn get_build_dir(&self) -> Option<String> {
        self.dirs.get("build").cloned()
    }
    
    pub fn get_generator(&self) -> Option<String> {
        Some(self.project_details.generator?.to_string())
    }

    pub fn create_new_config(
        name: &str,
        generator: Option<Generator>,
        language: Language,
        sources_dir: &str,
        headers_dir: &str,
        build_dir: &str,
    ) -> ProjectConfig {
        let mut dirs = HashMap::new();
        dirs.insert("sources".to_string(), sources_dir.to_string());
        dirs.insert("headers".to_string(), headers_dir.to_string());
        dirs.insert("build".to_string(), build_dir.to_string());
        
        ProjectConfig {
            project_details: ProjectDetails { name: name.to_string(), generator, language },
            dirs,
        }
    }

    pub fn serialize_config(config: &ProjectConfig) -> Result<String, Error> {
        toml::to_string(config).map_err(|e| Error::new(std::io::ErrorKind::Other, e))
    }

    pub fn deserialize_config(config_str: &str) -> Result<ProjectConfig, Error> {
        toml::from_str(config_str).map_err(|e| Error::new(std::io::ErrorKind::Other, e))
    }
}

pub fn load_project_config(config_path: &Path) -> Result<ProjectConfig, String> {
    if !config_path.exists() {
        return Err(format!(
            "Config file not found at {}",
            config_path.display()
        ));
    }

    let config_contents =
        fs::read_to_string(config_path).map_err(|e| format!("Error reading config file: {}", e))?;

    toml::de::from_str(&config_contents).map_err(|e| format!("Error parsing config file: {}", e))
}
