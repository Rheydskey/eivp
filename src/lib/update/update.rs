use runas::Command;

pub fn update() {
    Command::new("sudo")
            .arg("xbps-install")
            .arg("-Syu")
            .status()
            .expect("failed to execute process");
    std::process::Command::new("");
    if let Some(i) = std::env::var_os("HOME") {
        std::env::set_current_dir(&format!("{}/.eivp/", &i.to_str().unwrap()))
            .unwrap();
        std::process::Command::new("git")
            .arg("pull")
            .status()
            .expect("failed to execute process");
    }

}