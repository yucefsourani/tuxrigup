use std::fs;
use tempfile;
use tempfile::NamedTempFile;
use std::io::Write;
use std::env;
use std::path::PathBuf;


pub fn get_exe_dir() -> Option<PathBuf> {
    let mut exe_path = env::current_exe().ok()?;
    exe_path.pop();
    Some(exe_path)
}

pub fn is_file(path: &str,ignore_symlink:bool) -> bool {
    if let Ok(metadata) = fs::metadata(path) {
        let is_file = metadata.file_type().is_file() ;
        if ignore_symlink == true {
            return is_file;
        } else {
            if metadata.file_type().is_symlink() {
                return false;
            }else {
                return is_file;
            }
            
        }
    }
    false
}

pub fn is_dir(path: &str,ignore_symlink:bool) -> bool {
    if let Ok(metadata) = fs::metadata(path) {
        let is_dir = metadata.file_type().is_dir() ;
        if ignore_symlink == true {
            return is_dir;
        } else {
            if metadata.file_type().is_symlink() {
                return false;
            }else {
                return is_dir;
            }
            
        }
    }
    false
}

pub fn is_exists(path: &str) -> bool {
    if let Ok(is_exists) = fs::exists(path) {
        is_exists
    }else {
        false
            
    }
}

pub fn join_paths(dir: &str, file: &str) -> String {
    let mut path = PathBuf::from(dir);
    path.push(file);
    path.to_string_lossy().into_owned()
}

pub fn get_icon_path(icon_name:&str) -> Option<String> {
    if let Some(location)  = get_icons_location(){
        let icon_name_location = join_paths(&location,icon_name);
        if is_file(&icon_name_location,true){
            return Some(icon_name_location);
        }
        
    }
    None
}
    
    
pub fn get_icons_location() -> Option<String>  {
    if let Some(mut dir) = get_exe_dir() {
        let mut clone_dir = dir.clone();
        
        dir.push("../../images");
        let dir = dir.to_string_lossy().into_owned();
        if is_dir(&dir,true) {
            return Some(dir);
        }

        clone_dir.push("../share/tuxrigup/images");
        let clone_dir = clone_dir.to_string_lossy().into_owned();
        if is_dir(&clone_dir,true) {
            return Some(clone_dir);
        }
        return None;
    }
    None
}



enum CommandToTempFileResult {
    Admin(Vec<String>),
    NonAdmin(Vec<String>),
    }



fn parser_commands(commands:  Vec<String>) -> CommandToTempFileResult {
    let  need_admin: bool = commands.iter().any(|c| c.trim().starts_with("pkexec"));
    let mut vec_commands: Vec<String> = Vec::new();
    let mut vec_target_location: Vec<String> = Vec::new();
    for command in commands {
        let trimmed = command.trim();
        if trimmed.is_empty() {
            continue;
        }
        if let Some(new_command) = trimmed.strip_prefix("pkexec") {
            vec_commands.push(new_command.trim().to_string());
        }else {
            if command.starts_with("export TARGET_FILE") || 
               command.starts_with("export TARGET_DIR") || 
               command.starts_with("TARGET_FILE") ||
               command.starts_with("TARGET_DIR")  ||
               command.starts_with("export USER_HOME_DIR") ||
               command.starts_with("export USER_CONFIG_DIR") ||
               command.starts_with("USER_HOME_DIR") ||
               command.starts_with("USER_CONFIG_DIR")
               {
                vec_commands.push(command.trim().to_string());
                vec_target_location.push(command.trim().to_string());
                continue;
            }
            if need_admin {
                vec_commands.push(format!("su - \"$REAL_USER\" << 'EOF'\n"));
                for target_env in vec_target_location.iter() {
                    vec_commands.push(format!("{}\n",target_env));
                }
                vec_commands.push(format!("{}\nEOF",command));
            }else{
                vec_commands.push(command.to_string());
            }
        }
    }
    if need_admin {
        vec_commands.insert(0,"REAL_USER=$(id -nu \"$PKEXEC_UID\")".to_string());
        CommandToTempFileResult::Admin(vec_commands)
    }else{
        CommandToTempFileResult::NonAdmin(vec_commands)
    }
}





#[derive(Debug)]
pub enum TempFileResult {
    Admin(String),
    NonAdmin(String),
    Error,
    }
    
#[allow(dead_code)]
pub fn get_temp_file(commands: Vec<String>) ->  TempFileResult{
    let  temp_file = NamedTempFile::new();
    match temp_file {
        Ok(temp_f) => {
            if let Ok((mut _file, path)) = temp_f.keep() {
                let new_command: CommandToTempFileResult = parser_commands(commands);
                match new_command {
                    CommandToTempFileResult::Admin(vec_command) => {
                        for line in vec_command.into_iter() {
                            let _ = write!(_file,"{}\n",line);
                        }
                        TempFileResult::Admin(path.to_string_lossy().into_owned())
                    },
                    CommandToTempFileResult::NonAdmin(vec_command) => {
                        for line in vec_command.into_iter() {
                            let _ = write!(_file,"{}\n",line);
                        }
                        TempFileResult::NonAdmin(path.to_string_lossy().into_owned())
                    }
                }
            }else{
                TempFileResult::Error
            }
            
            },
        _ => TempFileResult::Error ,
        
    }
    
}

