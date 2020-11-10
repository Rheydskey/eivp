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
                Source::None => write!(f, "anywhere"),
            }
        }
    }
    #[derive(Debug)]
    pub struct Packages {
        pub name: String,       // PkgName
        pub version: String,    // X.X.X
        pub subversion: String, // X.X.X.Y
        pub arch: String,       //Archicture or Archs
        pub maintainer: String, // maintainer
        pub short_desc: String, // short_desc
        pub source: Source,     // Void_Packages or Repo
    }
    impl Packages {
        pub fn new() -> Packages {
            Packages {
                name: String::new(),
                version: String::new(),
                subversion: String::new(),
                arch: String::new(),
                maintainer: String::new(),
                short_desc: String::new(),
                source: Source::None,
            }
        }
        pub fn set_name(&mut self, name: String) -> &mut Packages {
            self.name = name;
            self
        }
        pub fn set_version(&mut self, version: String) -> &mut Packages{
            self.version = version;
            self
        }
        pub fn set_subversion(&mut self, subversion: String) -> &mut Packages {
            self.subversion = subversion;
            self
        }
        pub fn set_arch(&mut self, arch: String) -> &mut Packages {
            self.arch = arch;
            self
        }
        pub fn set_maintainer(&mut self, maintainer: String) -> &mut Packages {
            self.maintainer = maintainer;
            self
        }
        pub fn set_short_desc(&mut self, short_desc: String) -> &mut Packages {
            self.short_desc = short_desc;
            self
        }
        pub fn set_source(&mut self, source: Source) -> &mut Packages {
            self.source = source;
            self
        }
    }
}
