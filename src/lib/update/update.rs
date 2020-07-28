use runas::Command;

pub fn update() {
    Command::new("sudo")
            .arg("xbps-install")
            .arg("-Syu")
            .status()
            .expect("failed to execute process");
    std::process::Command::new("");
}