use clap::builder::ValueParser;
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
