use crate::lib::query::qstruct::package_struct::{Packages, Source};
use crate::query::*;
use libxbps::{CommandTrait, XbpsInstall, XbpsSrc};

pub fn update() {
    XbpsInstall::update().unwrap();

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
        let mut packages_to_update: Vec<String> = Vec::new();
        let xbpssrc = XbpsSrc::new(format!("{}{}", i.to_str().unwrap(), "/.eivp/./xbps-src"));
        let output = xbpssrc.showsysupdate().unwrap();
        xbpssrc.update().unwrap();

        for sp in output.split("\n").collect::<Vec<&str>>() {
            if !sp.is_empty() {
                let mut package = Packages::new();
                package.set_name(sp.to_string());
                package.set_source(Source::VoidPackages);
                packages_to_update.push(sp.to_string());
                let toinstall = query_info_void_package(sp.to_string());
                let installed = get_info_repo_packages(sp.to_string());
                println!("{} {} --> {}", sp, installed.version, toinstall.version);
                XbpsInstall.spawn(&[&sp]).unwrap();
            }
        }
    }
}

fn voidupdate() {
    if let Some(i) = std::env::var_os("HOME") {
        let command =
            std::process::Command::new(format!("{}{}", i.to_str().unwrap(), "/.eivp/./xbps-src"))
                .arg("")
                .output()
                .expect("failed to execute process");
    }
}
