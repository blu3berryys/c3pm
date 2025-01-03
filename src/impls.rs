use crate::model;
use crate::model::{
    BuildConfig, CStandard, CppStandard, Generator, Language, ProjectConfig, ProjectDetails,
};
use clap::builder::ValueParser;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::io::Error;
use std::str::FromStr;

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

impl TryFrom<&str> for CStandard {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        model::parse_standard(
            value,
            &[
                (Self::C89, "89"),
                (Self::C89, "90"),
                (Self::C99, "99"),
                (Self::C11, "11"),
                (Self::C17, "17"),
                (Self::C23, "23"),
            ],
        )
    }
}

impl Display for CStandard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            CStandard::C89 => "90",
            CStandard::C99 => "99",
            CStandard::C11 => "11",
            CStandard::C17 => "17",
            CStandard::C23 => "23",
        };
        write!(f, "{str}")
    }
}

impl TryFrom<&str> for CppStandard {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        model::parse_standard::<CppStandard>(
            value,
            &[
                (Self::Cpp98, "98"),
                (Self::Cpp11, "11"),
                (Self::Cpp14, "14"),
                (Self::Cpp17, "17"),
                (Self::Cpp20, "20"),
                (Self::Cpp23, "23"),
            ],
        )
    }
}

impl Display for CppStandard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            CppStandard::Cpp98 => "98",
            CppStandard::Cpp11 => "11",
            CppStandard::Cpp14 => "14",
            CppStandard::Cpp17 => "17",
            CppStandard::Cpp20 => "20",
            CppStandard::Cpp23 => "23",
        };
        write!(f, "{str}")
    }
}

impl Display for Language {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match &self {
            Language::C(_) => "c".to_string(),
            Language::CPP(_) => "c++".to_string(),
        };
        write!(f, "{}", str)
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

impl Default for ProjectDetails {
    fn default() -> ProjectDetails {
        Self {
            name: String::new(),
            generator: None,
            language: Language::CPP(CppStandard::Cpp23),
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
            project_details: ProjectDetails {
                name: name.to_string(),
                generator,
                language,
            },
            dirs,
        }
    }

    pub fn serialize_config(config: &ProjectConfig) -> Result<String, Error> {
        toml::to_string_pretty(config).map_err(|e| Error::new(std::io::ErrorKind::Other, e))
    }

    pub fn deserialize_config(config_str: &str) -> Result<ProjectConfig, Error> {
        toml::from_str(config_str).map_err(|e| Error::new(std::io::ErrorKind::Other, e))
    }
}