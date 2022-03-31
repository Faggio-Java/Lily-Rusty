
use curl::easy::Easy;
use std::env;
use std::fs::File;
use std::io::Write;
use std::process::Command;
use std::{thread, time};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    sudo::escalate_if_needed()?;
    let mut dst = Vec::new();
    let mut easy = Easy::new();
    let args: Vec<String> = env::args().collect();
    let urlz = format!("https://sourceforge.net/projects/bin-lily/files/{}.tar.xz/download", args[1]);
    let path = format!("{}.tar.xz", args[1]);

    easy.url(&urlz).unwrap();
    let _redirect = easy.follow_location(true);
    {
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {
            dst.extend_from_slice(data);
            Ok(data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }
    {
        let mut file = File::create(&path)?;
        file.write_all(dst.as_slice())?;
    }

    Command::new("tar")
        .arg("-xf")
        .arg(format!("{}.tar.xz", args[1]))
        .spawn()
        .expect("Command failed to start");

        thread::sleep(time::Duration::from_secs(1));

    Command::new("sh")
        .arg(format!("{}/install.sh", args[1]))
        .spawn()
        .expect("Command failed to start");
    Ok(())
}