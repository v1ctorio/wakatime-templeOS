use std::fs;
use std::path::{self, Path};
use std::process::{Command, Stdio};


const TOS_DISK_PATH: &str = "./assets/templeOS-disk.qcow2";
const TOS_IMAGE_LOCAL_PATH: &str = "./assets/TempleOS.ISO";
const TOS_IMAGE_REMOTE_PATH: &str = "https://templeos.org/Downloads/TempleOS.ISO";

fn main() {
    let tos_disk = Path::new(TOS_DISK_PATH);
    let tos_disk = path::absolute(tos_disk).unwrap();

    check_dependencies();

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
    let tos_image_local = path::absolute(tos_image_local).unwrap();
    if !tos_image_local.exists() {
        download_tos_image(&tos_image_local);
    }
}

fn download_tos_image(path: &Path) {
    let res = Command::new("wget")
        .arg(TOS_IMAGE_REMOTE_PATH)
        .args(["-o", path.to_str().unwrap()]);
}

const REQUIRED_CMDS: &[&str] = &["qemu-system-x86_64", "qemu-img", "wget"];
fn check_dependencies() {
    let mut not_ok_cmds: Vec<&str> = Vec::new();
    for cmd in REQUIRED_CMDS {
        let c = Command::new(cmd)
                    .arg("--help")
                    .stdout(Stdio::null())
                    .status()
                    .ok();
        let alr = match c {
            Some(status) => status.success(),
            None => false
        };

        if !alr {
            not_ok_cmds.push(cmd);
        }

    }

    if not_ok_cmds.len() > 0 {
        println!("ERROR: the following commands failed to be executed or could not be found in path");
        println!("       {}", not_ok_cmds.join(", "));
        std::process::exit(1);
    }
}
