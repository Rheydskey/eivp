use std::process::{Command, Stdio};

pub trait ConstPath {
    const CONST_PATH: &'static str;
}

pub trait Path {
    fn get_path(&self) -> &str;
}

impl<T: ConstPath> Path for T {
    fn get_path(&self) -> &str {
        Self::CONST_PATH
    }
}

pub trait CommandTrait {
    fn spawn_as_root(&self, args: &[&str]) -> Result<String, String>;

    fn spawn(&self, args: &[&str]) -> Result<String, String>;
}

impl<T> CommandTrait for T
where
    T: Path,
{
    fn spawn_as_root(&self, args: &[&str]) -> Result<String, String> {
        let child = Command::new("sudo")
            .args([&[self.get_path()], args].concat())
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to execute process");

        child
            .wait_with_output()
            .map(|f| {
                String::from_utf8(f.stdout)
                    .map_err(|f| f.to_string())
                    .unwrap()
            })
            .map_err(|f| f.to_string())
    }

    fn spawn(&self, args: &[&str]) -> Result<String, String> {
        let child = Command::new(self.get_path())
            .args(args)
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to execute process");

        child
            .wait_with_output()
            .map(|f| {
                String::from_utf8(f.stdout)
                    .map_err(|f| f.to_string())
                    .unwrap()
            })
            .map_err(|f| f.to_string())
    }
}

pub struct XbpsInstall;

impl XbpsInstall {
    pub fn update() -> Result<(), String> {
        Self.spawn(&["-Syu"]).map(|_| ())
    }
}

impl ConstPath for XbpsInstall {
    const CONST_PATH: &'static str = "xbps-install";
}

pub struct XbpsQuery;

impl XbpsQuery {
    pub fn search_from_shortdesc(name: &str) -> Result<String, String> {
        Self.spawn(&["-Rs", name])
    }
}

impl ConstPath for XbpsQuery {
    const CONST_PATH: &'static str = "xbps-query";
}

pub struct XbpsRemove;

impl XbpsRemove {
    pub fn remove(name: &str) -> Result<(), String> {
        Self.spawn_as_root(&["-R", name]).map(|_| ())
    }
}

impl ConstPath for XbpsRemove {
    const CONST_PATH: &'static str = "xbps-remove";
}

pub struct XbpsSrc {
    path: String,
}

impl XbpsSrc {
    pub fn new(path: String) -> Self {
        Self { path }
    }

    pub fn showsysupdate(&self) -> Result<String, String> {
        self.spawn(&["show-sys-updates"])
    }

    pub fn update(&self) -> Result<String, String> {
        self.spawn(&["update-bulk"])
    }

    pub fn pkg(&self, name: &str) -> Result<String, String> {
        self.spawn(&["pkg", name])
    }

    pub fn show(&self, name: &str) -> Result<String, String> {
        self.spawn(&["show", name])
    }

    pub fn binary_bootstrap(&self) -> Result<String, String> {
        self.spawn(&["binary-bootstrap"])
    }
}

impl Path for XbpsSrc {
    fn get_path(&self) -> &str {
        &self.path
    }
}
