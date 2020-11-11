mod lib;
use crate::lib::default::default;
use crate::lib::install::install;
use crate::lib::query::query;
use crate::lib::remove::remove;
use crate::lib::update::update;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "A easily installer for void packages", version = "0.0.1")]
enum Opt {
    /// Install a package
    Install {
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
        #[structopt()]
        package_name: Vec<String>,
    },
    /// Update package
    Update {},
}
fn main() {
    default::default();
    match Opt::from_args() {
        Opt::Install { package_name } => {
            install::install(package_name);
        }
        Opt::Query { package_name } => query::query(package_name),
        Opt::Remove { package_name } => remove::remove(package_name),
        Opt::Update {} => update::update(),
    }
}
