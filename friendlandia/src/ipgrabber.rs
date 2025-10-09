use std::process::{Command, Stdio};
use std::io;
pub fn get_ip() -> Vec<char>{
    let mut arr = vec![];
    if cfg!(target_os = "windows"){
         let ipconfigcommand = Command::new("cmd")
            .args(["/C", "ipconfig"])
            .stdout(Stdio::piped())
            .output()
            .expect("Failure");
        let resultthing = String::from_utf8_lossy(&ipconfigcommand.stdout);
        if resultthing.contains("IPv4 Address") {
            match resultthing.find("IPv4 Address") {
            Some(index) => {
                println!("Found 'program' at position: {}", index);
                match resultthing.find("Subnet Mask"){
                    Some(index2) =>{
                        println!("{}", index);
                        println!("{}", index2);
                        let hello = &resultthing[index..index2];
                        println!("{}", hello);
                        let mut n = 0;
                        let mut isready = false;
                        for i in hello.chars(){
                            println!("{}", i);
                            println!("{}", n);
                            if isready{
                                arr.push(i);
                            }
                            if i == ':'{
                                isready = true;
                            }
                            n = n + 1;
                        }
                        println!("{:?}", arr);
                        for i in 1..=5{
                            arr.pop();
                        }
                        arr.remove(0);
                        println!("{:?}", arr);
                    },
                    None => println!("Hi"), 
                } 

        },
            None => println!("Substring not found!"),
        }
        }
//TO DO: Create parser for the other supported OSes
    } else if cfg!(target_os = "linux") {
         let ipconfigcommand = Command::new("sh")
            .arg("-c")
            .arg("ip a")
            .stdout(Stdio::piped())
            .output()
            .expect("Failure");
        println!("{}", String::from_utf8_lossy(&ipconfigcommand.stdout));
    } else if cfg!(target_os = "macos"){
         let ipconfigcommand = Command::new("sh")
            .arg("-c")
            .arg("ifconfig")
            .stdout(Stdio::piped())
            .output()
            .expect("Failure");
        println!("{}", String::from_utf8_lossy(&ipconfigcommand.stdout));
    };
    arr
}