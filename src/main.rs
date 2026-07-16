mod memory;
mod installation_utils;

use std::fs;
use std::path::{self, Path};
use std::process::{Command, Stdio};
use std::time::Duration;
use std::{time, thread};
use std::os::unix::net::UnixStream;
use std::io::{Write, Read, BufRead, BufReader};
use jzon::{object, JsonValue};

//TODO support a custom assets dir via a flag
pub const TOS_DISK_PATH: &str = "./assets/templeOS-disk.qcow2";
pub const TOS_IMAGE_LOCAL_PATH: &str = "./assets/TempleOS.ISO";
pub const TOS_IMAGE_REMOTE_PATH: &str = "https://templeos.org/Downloads/TempleOS.ISO";
pub const QMP_SOCKET_PATH: &str = "./assets/qmp_socket.sock"; 

fn main() {
    let tos_disk = Path::new(TOS_DISK_PATH);
    let tos_disk = path::absolute(tos_disk).unwrap();
    let qmp_sock = Path::new(QMP_SOCKET_PATH);
    let qmp_sock = path::absolute(qmp_sock).unwrap();

    check_dependencies();

    if !tos_disk.exists() {
        installation_utils::create_tos_disk(&tos_disk);
    }

    let is_tos_installed = true; //TODO check this on runtime. maybe a assets/.tos_installed file?
    if !is_tos_installed {
        start_tos_installation(&tos_disk);
    
        thread::sleep(time::Duration::from_secs(20));
    }
    
    let _ = fs::remove_file(&qmp_sock);
    let mut tos = start_tos(&tos_disk, &qmp_sock);

    thread::sleep(time::Duration::from_secs(10));

    connect_qmp(&qmp_sock).unwrap();
    
    tos.kill();
    println!("Hello, world!");
    std::process::exit(0);
}

fn connect_qmp(sockPath: &Path) -> std::io::Result<()> {
   let mut stream = UnixStream::connect(sockPath)?; 
   println!("INFO: Connected with the QEMU stream");

   let mut reader = BufReader::new(stream.try_clone()?);

   stream.write_all(br#"{ "execute": "qmp_capabilities" }"#)?;
   stream.write_all(b"\n");

   //I think I need to do this because mutable reference (?)
   let mut msg_response = String::new();
   let mut send_msg = |cmd: &str, arguments: Option<JsonValue>| -> Option<JsonValue> {
        let msg = match arguments {
            Some(arguments) => object!{
                "execute": cmd,
                "arguments": arguments 
                },
            None => object!{ "execute": cmd }
        };
        let msg = jzon::stringify(msg);

        println!("QMP -> {}", &msg);
        stream.write_all(msg.as_bytes()).ok()?;
        stream.write_all(b"\n").ok()?;

        msg_response.clear();
        reader.read_line(&mut msg_response).ok()?;
        println!("QMP <- {}", &msg_response);
        return jzon::parse(&msg_response).ok();
   };

   send_msg("qmp_capabilities", Some(object! {}));
   println!("INFO: Sent qmp_capabilities message");

   loop {
        println!("INFO: ");
       send_msg("query_version", None);
       thread::sleep(Duration::from_secs(10));
   }
   
}

fn create_qmp_event_listener(cloned_stream: UnixStream) {
    todo!("I probably should stop worrying about async events and just work on the main functionality");
    thread::spawn(move || -> std::io::Result<()>{
        let mut reader = BufReader::new(cloned_stream.try_clone()?);
        let mut res = String::new();
        loop {
            res.clear();
            reader.read_line(&mut res)?;
            println!("QMP (e) <- {}", &res);
            let msg = jzon::parse(res.trim()).unwrap_or_else(|_| {
                println!("ERROR: malformed event received, aborting");
                std::process::exit(1)
            });
            if msg["event"] == "POWERDOWN" {
                println!("INFO: POWERDOWN event received. Shutting down");
                return Ok(());           
            }
        }

        Ok(())
    }); 

}

fn start_tos(disk_path: &Path, qmp_sock_path: &Path) -> std::process::Child {
    let disk_path = disk_path.to_str().unwrap();
    let qmp_sock_path = qmp_sock_path.to_str().unwrap();

    let tos = Command::new("qemu-system-x86_64")
                    .args(["-m", "512"])
                    .args(["-hda", disk_path])
                    .args(["-display","sdl"])
                    .arg("-qmp").arg(format!("unix:{},server=on,wait=off", qmp_sock_path))
                    .spawn()
                    .unwrap();

    thread::sleep(time::Duration::from_secs(1));
    println!("INFO: successfully started the TempleOS VM");
    tos
}

fn start_tos_installation(disk_path: &Path) -> std::process::Child {
    let tos_image_local = Path::new(TOS_IMAGE_LOCAL_PATH);
    let tos_image_local = path::absolute(tos_image_local).unwrap();
    if !tos_image_local.exists() {
        installation_utils::download_tos_image(&tos_image_local);
    }

    let tos_image_local = tos_image_local.to_str().unwrap();
    let disk_path = disk_path.to_str().unwrap();
 
    let mut tos = Command::new("qemu-system-x86_64")
                    .args(["-m","512"])
                    .args(["-hda",disk_path])
                    .args(["-cdrom",tos_image_local])
                    .args(["-boot","d"])
                    //.args(["-display", "sdl"])
                    .spawn()
                    .unwrap();
    todo!("installation flow not implemented")// I probably should focus on the runtime itself and see
             // how to handle installation later
    //return tos
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
