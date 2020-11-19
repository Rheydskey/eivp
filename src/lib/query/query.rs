use crate::lib::query::qstruct::package_struct::*;
use runas::Command;
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
        output_void_package(get_list_void_package(packages[0].clone()))
    } else {
        println!("Package needed")
    }
}
pub fn query_info_void_package(packages: String) -> Packages {
    // Get the home variable
    if let Some(i) = std::env::var_os("HOME") {
        let home = i.to_str().unwrap();
        // Search packages into script
        match read_dir(format!("{}{}", home, "/.eivp/srcpkgs/")) {
            Ok(o) => {
                for epath in o {
                    match epath {
                        Ok(e) => match e.file_name().into_string() {
                            Ok(e) => {
                                if e == packages {
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
                                    let mut packages: Packages = Packages::new();
                                    for sp in slipted {
                                        let split = sp
                                            .split(":")
                                            .map(|c| c.replace("\t", ""))
                                            .collect::<Vec<String>>();
                                        if split.len() == 2 {
                                            match split[0].as_str() {
                                                "pkgname" => {
                                                    packages.set_name(split[1].clone());
                                                }
                                                "maintainer" => {
                                                    packages.set_maintainer(split[1].clone());
                                                }
                                                "version" => {
                                                    packages.set_version(split[1].clone());
                                                }
                                                "revision" => {
                                                    packages.set_subversion(split[1].clone());
                                                }
                                                "archs" => {
                                                    packages.set_arch(split[1].clone());
                                                }
                                                "short_desc" => {
                                                    packages.set_short_desc(split[1].clone());
                                                }
                                                _ => {}
                                            }
                                        }
                                    }
                                    packages.set_source(Source::VoidPackages);
                                    return packages;
                                };
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
pub fn get_list_void_package(packages: String) -> Vec<Packages> {
    let mut vec_package: Vec<Packages> = Vec::new();
    // Get the home variable
    if let Some(i) = std::env::var_os("HOME") {
        let home = i.to_str().unwrap();
        // Search packages into script
        match read_dir(format!("{}{}", home, "/.eivp/srcpkgs/")) {
            Ok(o) => {
                for epath in o {
                    match epath {
                        Ok(e) => match e.file_name().into_string() {
                            Ok(e) => {
                                if e.contains(&packages.clone()) {
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
                                    let mut packages: Packages = Packages::new();
                                    for sp in slipted {
                                        let split = sp
                                            .split(":")
                                            .map(|c| c.replace("\t", ""))
                                            .collect::<Vec<String>>();
                                        if split.len() == 2 {
                                            match split[0].as_str() {
                                                "pkgname" => {
                                                    packages.set_name(split[1].clone());
                                                }
                                                "maintainer" => {
                                                    packages.set_maintainer(split[1].clone());
                                                }
                                                "version" => {
                                                    packages.set_version(split[1].clone());
                                                }
                                                "revision" => {
                                                    packages.set_subversion(split[1].clone());
                                                }
                                                "archs" => {
                                                    packages.set_arch(split[1].clone());
                                                }
                                                "short_desc" => {
                                                    packages.set_short_desc(split[1].clone());
                                                }
                                                _ => {}
                                            }
                                        }
                                    }
                                    packages.set_source(Source::VoidPackages);
                                    vec_package.push(packages);
                                };
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
    vec_package
}

pub fn get_packages_name_repo(packages_name: String) -> Vec<Packages> {
    let command = std::process::Command::new("xbps-query")
        .arg("-R")
        .arg("-s")
        .arg(packages_name)
        .output()
        .expect("failed to execute process");

    let output = str::from_utf8(command.stdout.as_ref()).unwrap();

    let mut packages: Vec<Packages> = Vec::new();

    for sp in output.split("\n").collect::<Vec<&str>>() {
        let mut pkg = Packages::new();

        let s: Vec<&str> = sp.split_whitespace().collect();
        if s.len() >= 1 {
            let split: Vec<&str> = s[1].split("-").collect();
            let mut name: String = "".to_string();
            for s in 0..split.len() {
                if split[s].contains(".") && split[s].contains("_") {
                    let s = split[s].split("_").collect::<Vec<&str>>();
                    pkg.set_subversion(s[1].to_string());
                    pkg.set_version(s[0].to_string());
                } else {
                    if !split[s].is_empty() {
                        if !name.is_empty() {
                            name.push_str(format!("-{}", split[s].clone()).as_str());
                        } else {
                            name.push_str(split[s]);
                        }
                    }
                }
            }
            pkg.set_source(Source::Repo);
            pkg.set_name(name);
        }
        if !pkg.name.is_empty() {
            packages.push(pkg);
        }
    }
    packages
}

fn output_void_package(packages: Vec<Packages>) {
    for packages_info in packages {
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
}

pub fn query_for_install(packages_name: String) -> Vec<Packages> {
    let packages = remove_void_package_if_repo(
        get_list_void_package(packages_name.to_owned()),
        get_packages_name_repo(packages_name.clone()),
    );
    let mut lenght = packages.len() - 1;
    for vpkg in &packages {
        if vpkg.source == Source::VoidPackages {
            println!(
                "{} {} from {} {}",
                lenght, vpkg.name, vpkg.source, vpkg.version
            );
        } else if vpkg.source == Source::Repo {
            println!(
                "{} {} from {} {}",
                lenght, vpkg.name, vpkg.source, vpkg.version
            );
        }
        lenght -= 1;
    }

    packages
}
pub fn remove_void_package_if_repo(
    void_package: Vec<Packages>,
    mut repo_package: Vec<Packages>,
) -> Vec<Packages> {
    let mut vec: Vec<Packages> = Vec::new();
    for vpkg in void_package.clone() {
        let mut lenght = 0;
        for i in 0..repo_package.len() {
            if vpkg.name == repo_package[i].name {
                break;
            } else {
                if lenght == repo_package.len() - 1 {
                    vec.push(vpkg.clone());
                    break;
                }
            }
            lenght += 1;
        }
    }
    vec.append(&mut repo_package);
    vec
}

pub fn get_info_repo_packages(packages_name: String) -> Packages {
    // Search packages into script
    let command = std::process::Command::new("xbps-query")
        .arg("-R")
        .arg("-S")
        .arg(&packages_name)
        .output()
        .expect("failed to execute process");

    let mut packages: Packages = Packages::new();
    packages.set_source(Source::Repo);
    let output = String::from(str::from_utf8(command.stdout.as_ref()).unwrap());
    let slipted: Vec<&str> = output.split("\n").collect();
    for sp in slipted {
        let sps = sp.to_string();
        let split: Vec<&str> = sps.split(":").collect();
        if split.len() == 2 {
            match split[0] {
                "pkgname" => {
                    packages.set_name(split[1].to_string().to_owned().replace(" ", ""));
                }
                "maintainer" => {
                    packages.set_maintainer(split[1].to_string().trim_start().to_string());
                }
                "pkgver" => {
                    let split_version: Vec<&str> = split[1].split("-").collect();
                    for s in 0..split_version.len() {
                        if split_version[s].contains(".") && split_version[s].contains("_") {
                            packages.set_version(split_version[s].to_string());
                        }
                    }
                }
                "architecture" => {
                    packages.set_arch(split[1].to_string().trim_start().to_string());
                }
                "short_desc" => {
                    packages.set_short_desc(split[1].to_string().trim_start().to_string());
                }
                _ => {}
            }
        }
    }
    packages
}
