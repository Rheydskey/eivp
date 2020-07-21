pub mod package_struct {
    use std::fmt;
    #[derive(Debug, PartialEq)]
    pub enum Source {
        Repo,
        VoidPackages,
        None,
    }
    impl fmt::Display for Source {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                Source::Repo => write!(f, "Repository"),
                Source::VoidPackages => write!(f, "Void Packages"),
                Source::None => write!(f, "anywhere")
            }
        }
    }
    #[derive(Debug)]
    pub struct Packages {
        pub name: String,       // PkgName
        pub version: String,    // X.X.X_Y X = Version, Y = Revision
        pub arch: String,       //Archicture or Archs
        pub maintainer: String,  // maintainer
        pub short_desc: String, // short_desc
        pub source: Source,     // Void_Packages or Repo
    }
    impl Packages {
        pub fn new() -> Packages {
            Packages {
                name: String::new(),
                version: String::new(),
                arch: String::new(),
                maintainer: String::new(),
                short_desc: String::new(),
                source: Source::None,
            }
        }
        pub fn set_name(&mut self, name: String) {
            self.name = name
        }
        pub fn set_version(&mut self, version: String) {
            self.version = version
        }
        pub fn set_arch(&mut self, arch: String) {
            self.arch = arch
        }
        pub fn set_maintainer(&mut self, maintainer: String) {
            self.maintainer = maintainer
        }
        pub fn set_short_desc(&mut self, short_desc: String) {
            self.short_desc = short_desc
        }
        pub fn set_source(&mut self, source: Source) {
            self.source = source
        }
    }
}
