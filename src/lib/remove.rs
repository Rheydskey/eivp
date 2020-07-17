use runas::Command;

pub fn remove(packages: Vec<String>) {
    if packages.len() != 0 {
        Command::new("sudo")
            .arg("xbps-remove")
            .arg("-R")
            .arg(&packages[0])
            .status()
            .expect("failed to execute process");
    } else {
        println!("Package needed")
    }
}
