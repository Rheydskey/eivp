use runas::Command;

pub fn query(packages: Vec<String>) {
    if packages.len() != 0 {
        Command::new("sudo")
            .arg("xbps-query")
            .arg("-R")
            .arg("-s")
            .arg(&packages[0])
            .status()
            .expect("failed to execute process");
    } else {
        println!("Package needed")
    }
}
