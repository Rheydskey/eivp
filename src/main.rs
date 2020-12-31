mod lib;
use crate::lib::default::default;
use crate::lib::install::install;
use crate::lib::query::query;
use crate::lib::remove::remove;
use crate::lib::update::update;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = r"
___ _____   _____ 
| __|_ _\ \ / / _ \
| _| | | \ V /|  _/
|___|___| \_/ |_|  
                  
",
    about = "A easily installer for void packages",
    version = "\nv2.0.0"
    
)]
enum Opt {
    
    /// Install a package
    #[structopt()]
    Install {
        #[structopt(long = "void-package-only")]
        void_package_only: bool,
        #[structopt()]
        package_name: Vec<String>,
    },
    /// Remove a package
    Remove {
        #[structopt()]
        package_name: Vec<String>,
    },
    /// Search a package
    Query {
        #[structopt(long = "void-package-only")]
        void_package_only: bool,
        #[structopt()]
        package_name: Vec<String>,
    },
    /// Update package
    Update {},
}
fn main() {
    default::default();
    match Opt::from_args() {
        Opt::Install {void_package_only, package_name } => {
            install::install(package_name, void_package_only);
        }
        Opt::Query {void_package_only, package_name } => query::query(package_name, void_package_only),
        Opt::Remove { package_name } => remove::remove(package_name),
        Opt::Update {} => update::update(),
    }
}
