use crate::model::{BuildConfig, CompilerDetails, Dependency, DependencyData, Generator, Language, Project, ProjectConfig};
use crate::{model, select_compilers};
use clap::builder::ValueParser;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::io::Error;
use std::str::FromStr;

impl DependencyData {
    pub fn default() -> DependencyData {
        DependencyData {
            name: String::new(),
            version: None,
            repository: (String::new(), String::new()),
        }
    }

    pub fn new(
        name: String,
        version: Option<String>,
        repository: (String, String),
    ) -> DependencyData {
        DependencyData {
            name,
            version,
            repository,
        }
    }
}

impl Dependency {
    pub fn default() -> Dependency {
        Dependency {
            dependency: DependencyData::default(),
        }
    }

    pub fn new(name: &str, version: &str, repository: (&str, &str)) -> Dependency {
        Dependency {
            dependency: DependencyData::new(
                name.to_string(),
                Some(version.to_string()),
                (repository.0.to_string(), repository.1.to_string()),
            ),
        }
    }
}

// woah
impl Display for Generator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            Generator::VisualStudio2022 => model::format_display(f, "Visual Studio 17 2022"),
            Generator::VisualStudio2019 => model::format_display(f, "Visual Studio 16 2019"),
            Generator::VisualStudio2017 => model::format_display(f, "Visual Studio 15 2017"),
            Generator::VisualStudio2015 => model::format_display(f, "Visual Studio 14 2015"),
            Generator::BorlandMakefiles => model::format_display(f, "Borland Makefiles"),
            Generator::NMakeMakefiles => model::format_display(f, "NMake Makefiles"),
            Generator::NMakeMakefilesJOM => model::format_display(f, "NMake Makefiles JOM"),
            Generator::MSYSMakefiles => model::format_display(f, "MSYS Makefiles"),
            Generator::MinGWMakefiles => model::format_display(f, "MinGW Makefiles"),
            Generator::GreenHillsMulti => model::format_display(f, "Green Hill MULTI"),
            Generator::UnixMakefiles => model::format_display(f, "Unix Makefiles"),
            Generator::Ninja => model::format_display(f, "Ninja"),
            Generator::NinjaMulticonfig => model::format_display(f, "Ninja Multi-Config"),
            Generator::WatcomWMake => model::format_display(f, "Watcom WMake"),
            Generator::CodeblocksMingw => model::format_display(f, "CodeBlocks - MinGW Makefiles"),
            Generator::CodeblocksNMake => model::format_display(f, "CodeBlocks - MinGW Makefiles"),
            Generator::CodeblocksNMakeJOM => {
                model::format_display(f, "CodeBlocks - NMake Makefiles JOM")
            }
            Generator::CodeblocksNinja => model::format_display(f, "CodeBlocks - Ninja"),
            Generator::CodeblocksUnixMake => {
                model::format_display(f, "CodeBlocks - Unix Makefiles")
            }
            Generator::CodeLiteMingw => model::format_display(f, "CodeLite - MinGW Makefiles"),
            Generator::CodeLiteNMake => model::format_display(f, "CodeLite - NMake Makefiles"),
            Generator::CodeLiteNinja => model::format_display(f, "CodeLite - Ninja"),
            Generator::CodeLiteUnixMake => model::format_display(f, "CodeLite - Unix Makefiles"),
            Generator::EclipseCDT4NMake => {
                model::format_display(f, "Eclipse CDT4 - NMake Makefiles")
            }
            Generator::EclipseCDT4MinGWMake => {
                model::format_display(f, "Eclipse CDT4 - MinGW Makefiles")
            }
            Generator::EclipseCDT4Ninja => model::format_display(f, "Eclipse CDT4 - Ninja"),
            Generator::EclipseCDT4UnixMake => {
                model::format_display(f, "Eclipse CDT4 - Unix Makefiles")
            }
            Generator::KateMinGWMake => model::format_display(f, "Kate - MinGW Makefiles"),
            Generator::KateNMake => model::format_display(f, "Kate - NMake Makefiles"),
            Generator::KateNinja => model::format_display(f, "Kate - Ninja"),
            Generator::KateNinjaMulticonfig => {
                model::format_display(f, "Kate - Ninja Multi-Config")
            }
            Generator::KateUnixMake => model::format_display(f, "Kate - Unix Makefiles"),
            Generator::SublimeMinGW => model::format_display(f, "Sublime Text 2 - MinGW Makefiles"),
            Generator::SublimeNMake => model::format_display(f, "Sublime Text 2 - NMake Makefiles"),
            Generator::SublimeNinja => model::format_display(f, "Sublime Text 2 - Ninja"),
            Generator::SublimeUnixMake => {
                model::format_display(f, "Sublime Text 2 - Unix Makefiles")
            }
        }
    }
}

impl TryFrom<&str> for Generator {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        model::parse_generator::<Generator>(
            value,
            &[
                (Self::VisualStudio2022, "Visual Studio 17 2022"),
                (Self::VisualStudio2019, "Visual Studio 16 2019"),
                (Self::VisualStudio2017, "Visual Studio 15 2017"),
                (Self::VisualStudio2015, "Visual Studio 14 2015"),
                (Self::BorlandMakefiles, "Borland Makefiles"),
                (Self::NMakeMakefiles, "NMake Makefiles"),
                (Self::NMakeMakefilesJOM, "NMake Makefiles JOM"),
                (Self::MSYSMakefiles, "MSYS Makefiles"),
                (Self::MinGWMakefiles, "MinGW Makefiles"),
                (Self::GreenHillsMulti, "Green Hill MULTI"),
                (Self::UnixMakefiles, "Unix Makefiles"),
                (Self::Ninja, "Ninja"),
                (Self::NinjaMulticonfig, "Ninja Multi-Config"),
                (Self::WatcomWMake, "Watcom WMake"),
                (Self::CodeblocksMingw, "CodeBlocks - MinGW Makefiles"),
                (Self::CodeblocksNMake, "CodeBlocks - NMake Makefiles"),
                (Self::CodeblocksNMakeJOM, "CodeBlocks - NMake Makefiles JOM"),
                (Self::CodeblocksNinja, "CodeBlocks - Ninja"),
                (Self::CodeblocksUnixMake, "CodeBlocks - Unix Makefiles"),
                (Self::CodeLiteMingw, "CodeLite - MinGW Makefiles"),
                (Self::CodeLiteNMake, "CodeLite - NMake Makefiles"),
                (Self::CodeLiteNinja, "CodeLite - Ninja"),
                (Self::CodeLiteUnixMake, "CodeLite - Unix Makefiles"),
                (Self::EclipseCDT4NMake, "Eclipse CDT4 - NMake Makefiles"),
                (Self::EclipseCDT4MinGWMake, "Eclipse CDT4 - MinGW Makefiles"),
                (Self::EclipseCDT4Ninja, "Eclipse CDT4 - Ninja"),
                (Self::EclipseCDT4UnixMake, "Eclipse CDT4 - Unix Makefiles"),
                (Self::KateMinGWMake, "Kate - MinGW Makefiles"),
                (Self::KateNMake, "Kate - NMake Makefiles"),
                (Self::KateNinja, "Kate - Ninja"),
                (Self::KateNinjaMulticonfig, "Kate - Ninja Multi-Config"),
                (Self::KateUnixMake, "Kate - Unix Makefiles"),
                (Self::SublimeMinGW, "Sublime Text 2 - MinGW Makefiles"),
                (Self::SublimeNMake, "Sublime Text 2 - NMake Makefiles"),
                (Self::SublimeNinja, "Sublime Text 2 - Ninja"),
                (Self::SublimeUnixMake, "Sublime Text 2 - Unix Makefiles"),
            ],
        )
    }
}

impl Display for BuildConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            BuildConfig::Debug => f.write_str("Debug"),
            BuildConfig::RelWithDebInfo => f.write_str("RelWithDebInfo"),
            BuildConfig::Release => f.write_str("Release"),
            BuildConfig::MinSizeRel => f.write_str("MinSizeRel"),
        }
    }
}

impl Display for Language {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match &self {
            //Language::C89 => "c89".to_string(),
            Language::C99 => "c99".to_string(),
            Language::C11 => "c11".to_string(),
            Language::C17 => "c17".to_string(),
            Language::C23 => "c23".to_string(),
            Language::Cpp98 => "cpp98".to_string(),
            Language::Cpp11 => "cpp11".to_string(),
            Language::Cpp14 => "cpp14".to_string(),
            Language::Cpp17 => "cpp17".to_string(),
            Language::Cpp20 => "cpp20".to_string(),
            Language::Cpp23 => "cpp23".to_string(),
        };

        write!(f, "{}", str)
    }
}

impl FromStr for Language {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().trim() {
            //"c89" => Ok(Language::C89),
            "c99" => Ok(Language::C99),
            "c11" => Ok(Language::C11),
            "c17" => Ok(Language::C17),
            "c23" => Ok(Language::C23),
            "cpp98" => Ok(Language::Cpp98),
            "cpp11" => Ok(Language::Cpp11),
            "cpp14" => Ok(Language::Cpp14),
            "cpp17" => Ok(Language::Cpp17),
            "cpp20" => Ok(Language::Cpp20),
            "cpp23" => Ok(Language::Cpp23),
            _ => Err(format!("Unknown language: {}", s)),
        }
    }
}

impl Language {
    pub fn get_lang_and_standard(&self) -> (String, String) {
        match self {
            //Language::C89 => ("C".to_string(), "89".to_string()),
            Language::C99 => ("C".to_string(), "99".to_string()),
            Language::C11 => ("C".to_string(), "11".to_string()),
            Language::C17 => ("C".to_string(), "17".to_string()),
            Language::C23 => ("C".to_string(), "23".to_string()),
            Language::Cpp98 => ("C++".to_string(), "98".to_string()),
            Language::Cpp11 => ("C++".to_string(), "11".to_string()),
            Language::Cpp14 => ("C++".to_string(), "14".to_string()),
            Language::Cpp17 => ("C++".to_string(), "17".to_string()),
            Language::Cpp20 => ("C++".to_string(), "20".to_string()),
            Language::Cpp23 => ("C++".to_string(), "23".to_string()),
        }
    }

    pub fn is_c(&self) -> bool {
        match self {
            //Language::C89 => true,
            Language::C99 => true,
            Language::C11 => true,
            Language::C17 => true,
            Language::C23 => true,
            Language::Cpp98 => false,
            Language::Cpp11 => false,
            Language::Cpp14 => false,
            Language::Cpp17 => false,
            Language::Cpp20 => false,
            Language::Cpp23 => false,
        }
    }
}

impl FromStr for BuildConfig {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_lowercase().as_str() {
            "Debug" => Ok(BuildConfig::Debug),
            "RelWithDebInfo" => Ok(BuildConfig::RelWithDebInfo),
            "Release" => Ok(BuildConfig::Release),
            "MinSizeRel" => Ok(BuildConfig::MinSizeRel),
            _ => Err("Invalid BuildConfig. Possible values are: Debug, RelWithDebInfo, Release, MinSizeRel".to_string()),
        }
    }
}

impl Into<ValueParser> for BuildConfig {
    fn into(self) -> ValueParser {
        ValueParser::new(move |arg: &str| BuildConfig::from_str(arg))
    }
}

impl Into<ValueParser> for Generator {
    fn into(self) -> ValueParser {
        ValueParser::new(move |arg: &str| Generator::try_from(arg))
    }
}

impl Default for Project {
    fn default() -> Project {
        let compilers = select_compilers();

        Self {
            name: String::new(),
            generator: None,
            language: Language::Cpp23,
            compiler: CompilerDetails {
                c_compiler: Some(compilers.0),
                cxx_compiler: Some(compilers.1),
            }
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
            project_details: Project::default(),
            dirs,
            dependencies: Some(vec![Dependency::default()]),
        }
    }
}

impl ProjectConfig {
    pub fn get_name(&self) -> &str {
        &self.project_details.name
    }

    pub fn get_language(&self) -> &str {
        match self.project_details.language {
            Language::C99 => "c",
            Language::C11 => "c",
            Language::C17 => "c",
            Language::C23 => "c",
            Language::Cpp98 => "cpp",
            Language::Cpp11 => "cpp",
            Language::Cpp14 => "cpp",
            Language::Cpp17 => "cpp",
            Language::Cpp20 => "cpp",
            Language::Cpp23 => "cpp",
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
        let compilers = select_compilers();
        dirs.insert("sources".to_string(), sources_dir.to_string());
        dirs.insert("headers".to_string(), headers_dir.to_string());
        dirs.insert("build".to_string(), build_dir.to_string());

        ProjectConfig {
            project_details: Project {
                name: name.to_string(),
                generator,
                language,
                compiler: CompilerDetails {
                    c_compiler: Some(compilers.0),
                    cxx_compiler: Some(compilers.1),
                }
            },
            dirs,
            dependencies: None,
        }
    }

    pub fn serialize_config(config: &ProjectConfig) -> Result<String, Error> {
        toml::to_string_pretty(config).map_err(|e| Error::new(std::io::ErrorKind::Other, e))
    }

    pub fn deserialize_config(config_str: &str) -> Result<ProjectConfig, Error> {
        toml::from_str(config_str).map_err(|e| Error::new(std::io::ErrorKind::Other, e))
    }
}
