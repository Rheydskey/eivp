use runas::Command;

pub fn install(packages: Vec<String>) {
    if packages.len() != 0 {
        Command::new("sudo")
            .arg("xbps-install")
            .arg(&packages[0])
            .status()
            .expect("failed to execute process");
    } else {
        println!("Package needed")
    }
}
