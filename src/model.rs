use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Language {
    C(CStandard),
    CPP(CppStandard),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CStandard {
    C89,
    C99,
    C11,
    C17,
    C23,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CppStandard {
    Cpp98,
    Cpp11,
    Cpp14,
    Cpp17,
    Cpp20,
    Cpp23,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum BuildConfig {
    Debug,
    RelWithDebInfo,
    Release,
    MinSizeRel,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, ValueEnum)]
pub enum Generator {
    VisualStudio2022,
    VisualStudio2019,
    VisualStudio2017,
    VisualStudio2015,
    BorlandMakefiles,
    NMakeMakefiles,
    NMakeMakefilesJOM,
    MSYSMakefiles,
    MinGWMakefiles,
    GreenHillsMulti,
    UnixMakefiles,
    Ninja,
    NinjaMulticonfig,
    WatcomWMake,
    CodeblocksMingw,
    CodeblocksNMake,
    CodeblocksNMakeJOM,
    CodeblocksNinja,
    CodeblocksUnixMake,
    CodeLiteMingw,
    CodeLiteNMake,
    CodeLiteNinja,
    CodeLiteUnixMake,
    EclipseCDT4NMake,
    EclipseCDT4MinGWMake,
    EclipseCDT4Ninja,
    EclipseCDT4UnixMake,
    KateMinGWMake,
    KateNMake,
    KateNinja,
    KateNinjaMulticonfig,
    KateUnixMake,
    SublimeMinGW,
    SublimeNMake,
    SublimeNinja,
    SublimeUnixMake,
}

pub fn format_display(f: &mut Formatter<'_>, string: &str) -> std::fmt::Result {
    f.write_str(string)
}

pub fn parse_standard<T: Copy>(value: &str, pairs: &[(T, &'static str)]) -> Result<T, String> {
    let formatted_possible_values = pairs.iter().map(|p| p.1).collect::<Vec<&str>>().join(", ");

    if value.is_empty() {
        return Err(format!(
            "Standard was not provided! Possible values are: {}",
            formatted_possible_values
        ));
    }

    let value = value.to_lowercase();

    for (k, v) in pairs.iter() {
        if v.eq(&value) {
            return Ok(*k);
        }
    }

    Err(format!(
        "Invalid standard version! Possible values are: {}",
        formatted_possible_values
    ))
}

pub fn parse_generator<T: Copy>(value: &str, pairs: &[(T, &'static str)]) -> Result<T, String> {
    let formatted_possible_values = pairs.iter().map(|p| p.1).collect::<Vec<&str>>().join(", ");

    if value.is_empty() {
        return Err(format!(
            "Generator was not provided! Possible values are: {}",
            formatted_possible_values
        ));
    }

    // let value = value.to_lowercase();

    for (k, v) in pairs.iter() {
        if v.eq(&value) {
            return Ok(*k);
        }
    }

    Err(format!(
        "Invalid generator name! Possible values are: {}",
        formatted_possible_values
    ))
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ProjectDetails {
    pub name: String,
    pub generator: Option<Generator>,
    pub language: Language,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ProjectConfig {
    pub project_details: ProjectDetails,
    #[serde(default)]
    pub dirs: HashMap<String, String>,
}