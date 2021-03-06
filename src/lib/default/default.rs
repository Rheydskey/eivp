use std::fs::read_dir;
use std::io::Write;
use std::process::Command;

pub fn default() {
    if let Some(i) = std::env::var_os("HOME") {
        let home = i.to_str().unwrap();
        let mut content: Vec<String> = Vec::new();
        match read_dir(home) {
            Ok(o) => {
                for epath in o {
                    match epath {
                        Ok(e) => match e.file_name().into_string() {
                            Ok(e) => {
                                content.push(e);
                            }
                            _ => {}
                        },
                        Err(_e) => {}
                    }
                }
            }
            Err(_e) => {}
        };
        if !content.contains(&String::from(".eivp")) {
            Command::new("git")
                .arg("clone")
                .arg("https://github.com/void-linux/void-packages.git")
                .arg(format!("{}{}", &home, "/.eivp/"))
                .status()
                .expect("failed to execute process");
            std::process::Command::new(format!("{}{}{}", &home, "/.eivp/", "./xbps-src"))
                .arg("binary-bootstrap")
                .status()
                .expect("failed to execute process");
        }
        std::fs::File::create(format!("{}{}", home, "/.eivp/etc/conf"))
            .expect("Error")
            .write(String::from("XBPS_ALLOW_RESTRICTED=yes").as_bytes())
            .expect("Error");
    } else {
        println!("Where is the HOME variable")
    }
}
