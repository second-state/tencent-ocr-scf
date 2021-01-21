use std::io::{self, Read, Write};
use serde::Deserialize;
use ssvm_process_interface::Command;
use std::fs;
use std::fs::File;
use std::str;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).expect("Error reading from STDIN");
    let obj: FaasInput = serde_json::from_str(&buffer).unwrap();
    let img_buf = base64::decode_config(&(obj.body), base64::STANDARD).unwrap();

    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let mut temp_filename = String::from("/tmp/");
    temp_filename.push_str(&now.as_millis().to_string());
    // temp_filename.push_str(".png");
    let copy_of_temp_filename = temp_filename.clone();
    let mut pos = 0;
    let mut file_buf = File::create(temp_filename).unwrap();
    while pos < img_buf.len() {
        let bytes_written = file_buf.write(&img_buf[pos..]).unwrap();
        pos += bytes_written;
    }

    let mut cmd = Command::new("/opt/tesseract");
    cmd.arg(&copy_of_temp_filename)
        .arg("stdout")
        .arg("--dpi")
        .arg("70")
        .arg("-l")
        .arg("chi_sim")
        .env("TESSDATA_PREFIX", "/opt")
        .env("LD_LIBRARY_PATH", "/opt");

    let out = cmd.output();
    fs::remove_file(&copy_of_temp_filename).unwrap();
    println!("{}", str::from_utf8(&out.stdout).unwrap().to_string());
    // println!("{}", str::from_utf8(&out.stderr).unwrap().to_string());
}

#[derive(Deserialize, Debug)]
struct FaasInput {
    body: String
}
