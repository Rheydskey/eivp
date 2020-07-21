use crate::lib::query::qstruct::package_struct::Source;
use crate::lib::query::query::query_for_install;
use runas::Command;
use std::{io::{self}};
pub fn install(packages: Vec<String>) {
    /*if packages.len() != 0 {

    */
    let vec = query_for_install(packages[0].clone());
    let mut index: usize = 0 as usize;
    for a in &vec {
        println!("{} {} from {} {}", index, a.name, a.source, a.version);
        index += 1;
    }
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    std::process::Command::new("clear").status().unwrap();
    let usize = buffer.trim().parse::<i64>().unwrap();
    println!("I'll install {}", vec[usize as usize].name);
    if vec[usize as usize].source == Source::Repo {
        Command::new("sudo")
            .arg("xbps-install")
            .arg(&vec[usize as usize].name)
            .status()
            .expect("failed to execute process");
    } else if vec[usize as usize].source == Source::VoidPackages {
        //xbps-install xtools
        Command::new("sudo")
            .arg("xbps-install")
            .arg("xtools")
            .status()
            .expect("failed to execute process");
        if let Some(i) = std::env::var_os("HOME") {
            std::process::Command::new(format!("{}/.eivp/./xbps-src", &i.to_str().unwrap()))
                .arg("pkg")
                .arg(&vec[usize as usize].name)
                .status()
                .expect("failed to execute process");
            std::env::set_current_dir(&format!("{}/.eivp/masterdir/", &i.to_str().unwrap())).unwrap();
            Command::new("sudo")
            .arg("xi")
            .arg(&vec[usize as usize].name)
            .status()
            .expect("failed to execute process");
        }
    }
}
