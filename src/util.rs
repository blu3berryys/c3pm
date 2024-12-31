use std::env::current_dir;
use std::fs;
use std::io::{Error, ErrorKind};

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
