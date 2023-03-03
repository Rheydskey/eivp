use std::{fmt::Display, process::Command};

use libxbps::XbpsSrc;

#[derive(thiserror::Error, Debug)]
enum EivpError {
    #[error("Cannot get home")]
    HomeInexistant,
    #[error("Cannot get home")]
    OsStringConversion,
}

pub fn is_eivp_folder_exists() -> anyhow::Result<bool> {
    let home = get_home()?;

    return Ok(std::fs::read_dir(format!("{}/.eivp/", home)).is_ok());
}

pub fn get_home() -> anyhow::Result<String> {
    let home_osstr = std::env::var_os("HOME").ok_or_else(|| EivpError::HomeInexistant)?;

    Ok(String::from(
        home_osstr
            .to_str()
            .ok_or_else(|| EivpError::OsStringConversion)?,
    ))
}

pub fn default() -> anyhow::Result<()> {
    if is_eivp_folder_exists()? {
        return Ok(());
    }

    let home = get_home()?;

    Command::new("git")
        .args(&[
            "clone",
            "https://github.com/void-linux/void-packages.git",
            "--depth",
            "1",
            &format!("{}{}", &home, "/.eivp/"),
        ])
        .status()
        .expect("failed to execute process");

    XbpsSrc::new(format!("{}{}{}", &home, "/.eivp/", "./xbps-src"))
        .binary_bootstrap()
        .unwrap();

    std::process::Command::new("sh")
        .arg("-c")
        .arg("echo")
        .arg("XBPS_ALLOW_RESTRICTED=yes")
        .arg(">>")
        .arg(format!("{}{}", home, "/.eivp/etc/conf"))
        .status()
        .expect("Error");

    Ok(())
}
