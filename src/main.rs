use std::fs;
use std::path::Path;

const TOS_DISK_PATH: &str = "./assets/templeOS-disk.qcow2";

fn main() {
    let tos_disk = Path::new(TOS_DISK_PATH);

    if !tos_disk.exists() {
        create_tos_disk();
    }

    println!("Hello, world!");
}

fn create_tos_disk() {
    todo!()
}
