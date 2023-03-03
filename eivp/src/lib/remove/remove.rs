use libxbps::XbpsRemove;

pub fn remove(packages: Vec<String>) {
    if packages.len() != 0 {
        XbpsRemove::remove(&packages[0]).unwrap();
    } else {
        println!("Package needed")
    }
}
