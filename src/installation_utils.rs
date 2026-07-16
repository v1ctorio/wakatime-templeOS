use std::process::{Command, Stdio};
use std::path::{self, Path};
use crate::{TOS_DISK_PATH, TOS_IMAGE_REMOTE_PATH, TOS_IMAGE_LOCAL_PATH};

pub fn create_tos_disk(path: &Path) {
    let disk_size = "2G";

    let path = path.to_str().unwrap();
    println!("INFO: Storage disk not found in {TOS_DISK_PATH}, attempting to create a 2G one via `qemu-img`\n");
    let status = Command::new("qemu-img")
                    .arg("create")
                    .args(["-f", "qcow2"])
                    .arg(path)
                    .arg(disk_size)
                    .status()
                    .unwrap();

    println!();
    match status.success() {
        true => println!("INFO: Successfully created the disk"),
        false => {
                println!("ERROR: failed to create disk. aborting");
                std::process::exit(1);
        }
    }
}

pub fn download_tos_image(path: &Path) {
    let path = path.to_str().unwrap();
    println!("INFO: TempleOS ISO not found in {TOS_IMAGE_LOCAL_PATH}, attempting to download from {TOS_IMAGE_REMOTE_PATH}");
    let res = Command::new("wget")
        .arg(TOS_IMAGE_REMOTE_PATH)
        .args(["-o", path])
        .status()
        .unwrap();
    if !res.success() {
        println!("ERROR: failed to download the TempleOS image");
        std::process::exit(1);
    } else {
        println!("INFO: successfully downloaded the TempleOS image")
    }
}

