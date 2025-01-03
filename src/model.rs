use clap::builder::ValueParser;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

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

fn format_display(f: &mut Formatter<'_>, string: &str) -> std::fmt::Result {
    f.write_str(string)
}

// woah
impl Display for Generator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            Generator::VisualStudio2022 => format_display(f, "Visual Studio 17 2022"),
            Generator::VisualStudio2019 => format_display(f, "Visual Studio 16 2019"),
            Generator::VisualStudio2017 => format_display(f, "Visual Studio 15 2017"),
            Generator::VisualStudio2015 => format_display(f, "Visual Studio 14 2015"),
            Generator::BorlandMakefiles => format_display(f, "Borland Makefiles"),
            Generator::NMakeMakefiles => format_display(f, "NMake Makefiles"),
            Generator::NMakeMakefilesJOM => format_display(f, "NMake Makefiles JOM"),
            Generator::MSYSMakefiles => format_display(f, "MSYS Makefiles"),
            Generator::MinGWMakefiles => format_display(f, "MinGW Makefiles"),
            Generator::GreenHillsMulti => format_display(f, "Green Hill MULTI"),
            Generator::UnixMakefiles => format_display(f, "Unix Makefiles"),
            Generator::Ninja => format_display(f, "Ninja"),
            Generator::NinjaMulticonfig => format_display(f, "Ninja Multi-Config"),
            Generator::WatcomWMake => format_display(f, "Watcom WMake"),
            Generator::CodeblocksMingw => format_display(f, "CodeBlocks - MinGW Makefiles"),
            Generator::CodeblocksNMake => format_display(f, "CodeBlocks - MinGW Makefiles"),
            Generator::CodeblocksNMakeJOM => format_display(f, "CodeBlocks - NMake Makefiles JOM"),
            Generator::CodeblocksNinja => format_display(f, "CodeBlocks - Ninja"),
            Generator::CodeblocksUnixMake => format_display(f, "CodeBlocks - Unix Makefiles"),
            Generator::CodeLiteMingw => format_display(f, "CodeLite - MinGW Makefiles"),
            Generator::CodeLiteNMake => format_display(f, "CodeLite - NMake Makefiles"),
            Generator::CodeLiteNinja => format_display(f, "CodeLite - Ninja"),
            Generator::CodeLiteUnixMake => format_display(f, "CodeLite - Unix Makefiles"),
            Generator::EclipseCDT4NMake => format_display(f, "Eclipse CDT4 - NMake Makefiles"),
            Generator::EclipseCDT4MinGWMake => format_display(f, "Eclipse CDT4 - MinGW Makefiles"),
            Generator::EclipseCDT4Ninja => format_display(f, "Eclipse CDT4 - Ninja"),
            Generator::EclipseCDT4UnixMake => format_display(f, "Eclipse CDT4 - Unix Makefiles"),
            Generator::KateMinGWMake => format_display(f, "Kate - MinGW Makefiles"),
            Generator::KateNMake => format_display(f, "Kate - NMake Makefiles"),
            Generator::KateNinja => format_display(f, "Kate - Ninja"),
            Generator::KateNinjaMulticonfig => format_display(f, "Kate - Ninja Multi-Config"),
            Generator::KateUnixMake => format_display(f, "Kate - Unix Makefiles"),
            Generator::SublimeMinGW => format_display(f, "Sublime Text 2 - MinGW Makefiles"),
            Generator::SublimeNMake => format_display(f, "Sublime Text 2 - NMake Makefiles"),
            Generator::SublimeNinja => format_display(f, "Sublime Text 2 - Ninja"),
            Generator::SublimeUnixMake => format_display(f, "Sublime Text 2 - Unix Makefiles"),
        }
    }
}

impl TryFrom<&str> for Generator {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        parse_generator::<Generator>(
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
        parse_standard(
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
        parse_standard::<CppStandard>(
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

fn parse_standard<T: Copy>(value: &str, pairs: &[(T, &'static str)]) -> Result<T, String> {
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

fn parse_generator<T: Copy>(value: &str, pairs: &[(T, &'static str)]) -> Result<T, String> {
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