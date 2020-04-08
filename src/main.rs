use std::process::Command;

extern crate regex;
extern crate wallpaper;

use regex::Regex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("xdpyinfo").output()?;

    let output = String::from_utf8(output.stdout)?;

    let re = Regex::new(r"dimensions:+\s*(\S+)").unwrap();
    let caps = re.captures(&output).unwrap();
    let mut url = String::new();
    match caps.get(1) {
        Some(resolution) => {
            url = format!("https://source.unsplash.com/{}/?woman", resolution.as_str());
        }
        None => {}
    }

    wallpaper::set_from_url(&url).unwrap();

    Ok(())
}
