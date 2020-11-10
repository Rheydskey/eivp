use crate::lib::query::qstruct::package_struct::*;
use runas::Command;
use std::collections::BTreeMap;
use std::fs::read_dir;
use std::str;

pub fn query(packages: Vec<String>) {
    if packages.len() != 0 {
        Command::new("sudo")
            .arg("xbps-query")
            .arg("-R")
            .arg("-s")
            .arg(&packages[0])
            .status()
            .expect("failed to execute process");
        get_packages_name_repo(packages[0].clone());
        query_for_install(packages[0].clone());
        output_void_package(query_info_void_package(packages[0].clone()))
    } else {
        println!("Package needed")
    }
}

pub fn query_info_void_package(packages: String) -> Packages {
    // Get the home variable
    if let Some(i) = std::env::var_os("HOME") {
        let home = i.to_str().unwrap();
        let mut packages_info: BTreeMap<String, String> = BTreeMap::new();
        // Search packages into script
        match read_dir(format!("{}{}", home, "/.eivp/srcpkgs/")) {
            Ok(o) => {
                for epath in o {
                    match epath {
                        Ok(e) => match e.file_name().into_string() {
                            Ok(e) => {
                                if packages == e {
                                    let command = std::process::Command::new(format!(
                                        "{}{}",
                                        home, "/.eivp/./xbps-src"
                                    ))
                                    .arg("show")
                                    .arg(&e)
                                    .output()
                                    .expect("failed to execute process");

                                    let output = str::from_utf8(command.stdout.as_ref()).unwrap();
                                    let slipted: Vec<&str> = output.split("\n").collect();
                                    for sp in slipted {
                                        let split = sp
                                            .split(":")
                                            .map(|c| c.replace("\t", ""))
                                            .collect::<Vec<String>>();
                                        if split.len() == 2 {
                                            packages_info
                                                .insert(split[0].clone(), split[1].clone());
                                        }
                                    }
                                    let mut packages: Packages = Packages::new();
                                    packages.set_source(Source::VoidPackages);
                                    if let Some(i) = packages_info.get("pkgname") {
                                        packages.set_name(i.to_owned());
                                    }
                                    if let Some(i) = packages_info.get("maintainer") {
                                        packages.set_maintainer(i.to_owned());
                                    }
                                    if let (Some(i), Some(e)) = (
                                        packages_info.get("version"),
                                        packages_info.get("revision"),
                                    ) {
                                        packages.set_version(format!("{}_{}", i, e));
                                    }
                                    if let Some(i) = packages_info.get("archs") {
                                        packages.set_arch(i.to_owned());
                                    }
                                    if let Some(i) = packages_info.get("short_desc") {
                                        packages.set_short_desc(i.to_owned());
                                    }
                                    return packages;
                                }
                            }

                            Err(_e) => {}
                        },
                        Err(_e) => {}
                    }
                }
            }
            Err(_e) => {}
        };
    }
    Packages::new()
}

pub fn get_packages_name_repo(packages_name: String) -> Vec<String> {
    let mut packages_info: Vec<String> = Vec::new();
    let command = std::process::Command::new("xbps-query")
        .arg("-R")
        .arg("-s")
        .arg(packages_name)
        .output()
        .expect("failed to execute process");
    let output = str::from_utf8(command.stdout.as_ref()).unwrap();
    for sp in output.split("\n").collect::<Vec<&str>>() {
        let s: Vec<&str> = sp.split_whitespace().collect();
        if s.len() >= 1 {
            let split: Vec<&str> = s[1].split("-").collect();
            let mut name: String = "".to_string();
            for s in 0..split.len() {
                if !split[s].contains(".") && !split[s].contains("_") {
                        name = format!("{}-{}", name, split[s]);
                } else {
                    packages_info.push(name);
                    break;
                }
            }
        }
    }
    packages_info
}

fn output_void_package(packages_info: Packages) {
    let mut show = format!("{}-{}", packages_info.name, packages_info.version);
    if packages_info.name.trim().is_empty() && packages_info.source == Source::None {
    } else {
        let lenght = 30 - show.len();
        for _i in lenght + 1..29 {
            show.push(' ');
        }
        println!("[-] {}{} (Void-Packages)", show, packages_info.short_desc);
    }
}

pub fn query_for_install(packages_name: String) -> Vec<Packages> {
    let mut vec: Vec<Packages> = Vec::new();
    let mut index: usize = 0 as usize;
    let repo_packages = get_packages_name_repo(packages_name.clone());
    let void_packages = query_info_void_package(packages_name.to_owned());
    if void_packages.source != Source::None {
        println!(
            "{} {} from {} {}",
            index, void_packages.name, void_packages.source, void_packages.version
        );
        index += 1;
        vec.push(void_packages);
    }
    for package in repo_packages.to_owned() {
        let package_info = get_info_repo_packages(package);
        if package_info.name.is_empty() {
            continue;
        };
        println!(
            "{} {} from {} {}",
            index, package_info.name, package_info.source, package_info.version
        );
        index += 1;
        vec.push(package_info);
    }
    vec
}

pub fn get_info_repo_packages(packages_name: String) -> Packages {
    let mut packages_info: BTreeMap<String, String> = BTreeMap::new();
    // Search packages into script
    let command = std::process::Command::new("xbps-query")
        .arg("-R")
        .arg("-S")
        .arg(&packages_name)
        .output()
        .expect("failed to execute process");

    let output = String::from(str::from_utf8(command.stdout.as_ref()).unwrap());
    let slipted: Vec<&str> = output.split("\n").collect();
    for sp in slipted {
        let sps = sp.to_string();
        let split: Vec<&str> = sps.split(":").collect();
        let lenght: usize =  split.len();
        if lenght == 2 {
            packages_info.insert(split[0].to_string(), split[1].to_string());
        }
    }
    let mut packages: Packages = Packages::new();
    packages.set_source(Source::Repo);
    if let Some(i) = packages_info.get("pkgname") {
        packages.set_name(i.to_owned().replace(" ", ""));
    }
    if let Some(i) = packages_info.get("maintainer") {
        packages.set_maintainer(i.trim_start().to_string());
    }
    if let Some(i) = packages_info.get("pkgver") {
        let split: Vec<&str> = i.split("-").collect();
        for s in 0..split.len() {
            if split[s].contains(".") && split[s].contains("_") {
                packages.set_version(split[s].to_string());
            }
        }
    }
    if let Some(i) = packages_info.get("architecture") {
        packages.set_arch(i.trim_start().to_string());
    }
    if let Some(i) = packages_info.get("short_desc") {
        packages.set_short_desc(i.trim_start().to_string());
    }
    packages
}
