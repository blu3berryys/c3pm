use std::env::current_dir;
use std::fs;
use std::io::{Error, ErrorKind};
use std::process::Command;

pub fn get_current_path() -> std::io::Result<String> {
    current_dir().and_then(|p| {
        p.into_os_string()
            .into_string()
            .map_err(|_| Error::new(ErrorKind::Other, "Failed to convert OsString to String"))
    })
}

pub fn create_dir<T1, T2>(parent: T1, name: T2) -> std::io::Result<String>
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
