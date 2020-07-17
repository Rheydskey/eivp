use std::process::Command;

pub fn install(packages: Vec<String>) {
            Command::new("xbps-install")
                .arg(&packages[0])
                .output()
                .expect("failed to execute process");
}