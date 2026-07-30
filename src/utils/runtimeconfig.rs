use std::io::{BufRead,Result,BufReader};
use std::fs::File;

pub fn get_distro_info(like:bool) -> Result<(String,String)>{
    let file   = File::open("/etc/os-release")?;
    let buffer = BufReader::new(file);
    let mut distro_name    = String::new();
    let mut distro_version = String::new();
    for line in buffer.lines() {
        let line = line?;
        let mut newline = line.trim().to_string();
        newline.retain(|c| c != '"');
        newline.retain(|c| c != '\'');
        newline.retain(|c| c != ' ');
        if let Some(value) = newline.strip_prefix("ID=") {
            if like {
                if distro_name.len() == 0 {
                    distro_name.replace_range(..,value);
                }
            }else {
                distro_name.replace_range(..,value);
            }
            continue;
        }
        if let Some(value) = newline.strip_prefix("VERSION_ID=") {
            if like {
                if distro_version.len() == 0 {
                    distro_version.replace_range(..,value);
                }
            }else {
                distro_version.replace_range(..,value);
            }
            continue;
        }
        if let Some(value) = newline.strip_prefix("ID_LIKE=") {
            if !like {
                if distro_name.len() == 0 {
                    distro_name.replace_range(..,value);
                }
            }else {
                distro_name.replace_range(..,value);
            }
            continue;
        }
        if let Some(value) = newline.strip_prefix("VERSION_ID_LIKE=") {
            if !like {
                if distro_version.len() == 0 {
                    distro_version.replace_range(..,value);
                }
            }else {
                distro_version.replace_range(..,value);
            }
            continue;
        }
    }
    Ok((distro_name,distro_version))
}
