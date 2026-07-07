use std::fs;
use std::path::Path;
use std::process::Command;

const TOS_DISK_PATH: &str = "./assets/templeOS-disk.qcow2";
const TOS_IMAGE_LOCAL_PATH: &str = "./assets/TempleOS.ISO";
const TOS_IMAGE_REMOTE_PATH: &str = "https://templeos.org/Downloads/TempleOS.ISO";

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

fn start_tos_installation() {
    let tos_image_local = Path::new(TOS_IMAGE_LOCAL_PATH);
    let tos_image_local = tos_image_local.absolute();
    if !tos_image_local.exists() {
        download_tos_image(tos_image_local);
    }
}

fn download_tos_image(path: Path) {
    let res = Command::new("wget")
        .arg(TOS_IMAGE_REMOTE_PATH)
        .args(["-o", ])
}
