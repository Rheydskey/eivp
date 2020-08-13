use crate::lib::query::qstruct::package_struct::{Packages, Source};
use crate::query::*;
use crate::install::*;
use runas::Command;

pub fn update() {
    Command::new("sudo")
        .arg("xbps-install")
        .arg("-Syu")
        .status()
        .expect("failed to execute process");
    std::process::Command::new("");
    if let Some(i) = std::env::var_os("HOME") {
        std::env::set_current_dir(&format!("{}/.eivp/", &i.to_str().unwrap())).unwrap();
        std::process::Command::new("git")
            .arg("pull")
            .status()
            .expect("failed to execute process");
    }
    check_update();
}

fn check_update() {
    if let Some(i) = std::env::var_os("HOME") {
        let mut packages_to_update : Vec<String> = Vec::new();
        let command = std::process::Command::new(format!("{}{}", i.to_str().unwrap(), "/.eivp/./xbps-src"))
            .arg("show-sys-updates")
            .output()
            .expect("failed to execute process");
        let output = std::str::from_utf8(command.stdout.as_ref()).unwrap();
        for sp in output.split("\n").collect::<Vec<&str>>() {
            if !sp.is_empty() {
                let mut package= Packages::new();
                package.set_name(sp.to_string());
                package.set_source(Source::VoidPackages);
                packages_to_update.push(sp.to_string());
                let toinstall = query_info_void_package(sp.to_string());
                let installed = get_info_repo_packages(sp.to_string());
                println!("{} {} --> {}",sp,installed.version,toinstall.version );
                install_package(&package);
            }
        }

    }
}