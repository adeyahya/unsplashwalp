extern crate reqwest;
extern crate tokio;
extern crate dirs;

use std::process::Command;
use std::io::prelude::*;
use std::fs::File;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let home_dir = dirs::home_dir().unwrap();
    let mut response = reqwest::get("https://source.unsplash.com/random")
    .await?;

    let fname_string = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");

    println!("file to download: '{}'", fname_string);
    let fname = home_dir.join("unsplashwalp").join(fname_string);
    let fname_string = home_dir.join("unsplashwalp").join(fname_string).into_os_string().into_string().unwrap();
    println!("will be located under: '{:?}'", fname);

    let mut buffer = File::create(fname)?;

    while let Some(chunk) = response.chunk().await? {
        buffer.write(&chunk)?;
    }

    let mut child = Command::new("gsettings")
                        .arg("set")
                        .arg("org.gnome.desktop.background")
                        .arg("picture-uri")
                        .arg(fname_string)
                        .spawn()
                        .expect("failed to execute child");

    child.wait().expect("failed to wait on child");

    println!("body = {:?}", response.url());
    Ok(())
}