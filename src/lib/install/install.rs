use crate::lib::query::qstruct::package_struct::{Packages, Source};
use crate::lib::query::query::query_for_install;
use runas::Command;
use std::io::{self};

pub fn install(packages: Vec<String>) {
    if packages.is_empty() {
        println!("No package name provied")
    } else {
        let vec = query_for_install(packages[0].clone());
        println!("Choose a number of package (1 2 3 , 1-3)");
        loop {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).unwrap();
            if buffer.contains("-") {
                let split: Vec<&str> = buffer.split("-").collect();
                println!("{}", split.len());
                if split.len() == 2 {
                    for d in (split[0].clone().trim().parse::<i64>().unwrap() as usize)
                        ..(split[1].clone().trim().parse::<i64>().unwrap() as usize)
                    {
                        install_package(&vec[(vec.len()-1) - d as usize]);
                        break;
                    }
                } else {
                    println!("too many or too few arguments")
                }
            } else if buffer.contains(" ") {
                let split: Vec<&str> = buffer.split(" ").collect();
                for f in 0..split.len() {
                    if !split[f].trim().is_empty() {
                        let n = &vec[(vec.len()-1) - split[f].clone().trim().parse::<i64>().unwrap() as usize];
                        install_package(n);
                        break;
                    }
                }
            } else {
                let index = buffer.clone().trim().parse::<i64>().unwrap() as usize;
                if  vec.len() >= index {
                    std::process::Command::new("clear").status().unwrap();
                    install_package(&vec[(vec.len()-1) - index]);
                    break;
                } else {
                    println!("the number entered is incorrect, retry !")
                }
            };
        }
    }
}

pub fn install_package(package: &Packages) {
    println!("I'll install {}", package.name);
    let name = package.name.clone();
    if package.source == Source::Repo {
        Command::new("sudo")
            .arg("xbps-install")
            .arg(&name)
            .status()
            .expect("failed to execute process");
    } else if package.source == Source::VoidPackages {
        //xbps-install xtools
        Command::new("sudo")
            .arg("xbps-install")
            .arg("xtools")
            .status()
            .expect("failed to execute process");
        if let Some(i) = std::env::var_os("HOME") {
            let home = format!("{}/.eivp/masterdir/", &i.to_str().unwrap());
            std::process::Command::new(format!("{}/.eivp/./xbps-src", &i.to_str().unwrap()))
                .arg("pkg")
                .arg(&name)
                .status()
                .expect("failed to execute process");
            std::env::set_current_dir(&home).unwrap();
            println!("{}", &home);
            std::process::Command::new("xi")
                .arg(&name)
                .status()
                .expect("failed to execute process");
        }
    }
}
