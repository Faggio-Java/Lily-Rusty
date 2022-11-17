use curl::easy::Easy;
use std::env;
use std::fs::{File, remove_file};
use std::io::{Write, Read};
use std::process::Command;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    sudo::escalate_if_needed()?;
    let mut dst = Vec::new();
    let mut easy = Easy::new();
    let args: Vec<String> = env::args().collect();
    if args[1] == "list" {
        let response = reqwest::blocking::get(
            "https://sourceforge.net/projects/bin-lily/files/",
        )
        .unwrap()
        .text()
        .unwrap();
        
        let html = scraper::Html::parse_document(&response);
        let finder = scraper::Selector::parse("tr.file>th>a>span.name").unwrap();
        let names = html.select(&finder).map(|x| x.inner_html());
        names
        .zip(1..101)
        .for_each(|name| println!("{:?}", name));
    } else if args[1] == "install" {
        let urlz = format!("https://sourceforge.net/projects/bin-lily/files/{}.tar.xz/download", args[2]);
        let path = format!("{}.tar.xz", args[2]);
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
            .arg(&path)
            .output()
            .expect("Command failed to start");

        Command::new("sh")
            .arg(format!("{}/install.sh", args[2]))
            .output()
            .expect("Command failed to start");

        remove_file(&path)?;
        
    } else if args[1] == "installed" {
        let mut path = File::open("/usr/lily/ports/installed.sh").expect("File doesn't exist?");
        let mut text = String::new();
        println!("{:?}", path.read_to_string(&mut text));
    }
    Ok(())
}
