use futures::channel::mpsc::UnboundedSender;
use crate::utils;
use crate::DISTRO_VERSION;
use crate::HOMEDIR;
use crate::CONFIGDIR;
use crate::DOWNLOADSDIR;
use gio;
use gtk::glib;
use tempfile::Builder;
use std::sync::{Arc,Mutex};

pub trait DownloadTaskTrait: Send + Sync {
    fn generate_download_location(&mut self) -> bool;
}

#[derive(Debug)]
pub struct LauncherFileInfo {
    pub laucher_file_name: &'static str,
    pub type_: PluginType,
    pub arch: &'static [&'static str],
    pub distro_name: &'static [&'static str],
    pub distro_version: &'static [&'static str],
    pub category: Category,
    pub desktop_env:  &'static [&'static str],
    pub display_type: &'static [&'static str],
    pub icon_name: &'static str,
    pub custom_button_label: Option<&'static str>,
}

#[derive(Debug)]
pub struct WebsiteInfo {
    pub link: &'static str,
    pub title: &'static str,
    pub subtitle: &'static str,
    pub type_: PluginType,
    pub arch: &'static [&'static str],
    pub distro_name: &'static [&'static str],
    pub distro_version: &'static [&'static str],
    pub category: Category,
    pub desktop_env:  &'static [&'static str],
    pub display_type: &'static [&'static str],
    pub icon_name: &'static str,
    pub custom_button_label: Option<&'static str>,
}


#[derive(Debug)]
pub struct DownloadTask {
    pub link: &'static str,
    pub dir_download_location: Option<String>,
    pub file_name: &'static str,
}



impl DownloadTaskTrait for Box<dyn DownloadTaskTrait> {
    fn generate_download_location(&mut self) -> bool { (**self).generate_download_location() }
}

impl DownloadTaskTrait for DownloadTask {
    fn generate_download_location(&mut self) ->bool {
        if let Some(_dir_download) =  &self.dir_download_location {
            return true;
        }else{
            let mut _to_return:bool;
            let temp_dir = Builder::new().tempdir().unwrap();
            let path = temp_dir.keep();
            let path_string: Option<String> = match path.into_os_string().into_string() {
                Ok(string) => {
                    _to_return = true;
                    Some(string)
                    },
                Err(_original_os_string) => {
                    _to_return = false;
                    None
                    },
            };
            
            self.dir_download_location = path_string;
            _to_return
        }
    }
}

#[derive(Eq, Hash, PartialEq,Debug,Copy,Clone)]
pub enum Category {
    Website,
    Developertools,
    Multimedia,
    Graphics,
    Other,
    System,
    Gnome,
    Internet,
    Launcher,
    Education,
    Utility,
    }

impl Category {
    pub fn get_catagory_label(category:Self) -> &'static str {
        match category {
            Self::Website => "WebSite",
            Self::Developertools => "Developer",
            Self::Multimedia => "Multimedia",
            Self::Graphics => "Graphics",
            Self::Other => "Other",
            Self::System => "System",
            Self::Gnome => "Gnome",
            Self::Internet => "Internet",
            Self::Launcher => "Launcher",
            Self::Education => "Education",
            Self::Utility => "Utility",
            }
        }

        
    pub fn get_str_list_catagory() -> &'static [&'static str] {
        &["Launcher","WebSite","Internet","Multimedia","Graphics","Education","Developer","Utility","System","Gnome","Other"]
        }
    /*pub fn get_catagory_icon_name(category: &'static str) -> &'static str {
        match category {
            "WebSite" => "insert-link-symbolic",
            "Developer" => "utilities-terminal-symbolic",
            "Multimedia" => "applications-multimedia-symbolic",
            "Graphics" => "applications-graphics-symbolic",
            "Other" => "preferences-other-symbolic",
            "System" => "applications-system-symbolic",
            "Gnome" => "preferences-desktop-appearance-symbolic",
            "Internet" => "web-browser-symbolic",
            "Launcher" => "application-x-executable-symbolic",
            "Education" => "accessories-dictionary-symbolic",
            "Utility" => "applications-utilities-symbolic",
            _           => "action-unavailable-symbolic",
            }
        }*/
    }


#[derive(Debug,Copy,Clone,PartialEq)]
pub enum PluginType {
    Installer,
    EnableDisable,
    Oneshot,
    Website,
    Launcher,
    }

impl PluginType {
    pub fn get_type_label(category:Self) -> &'static str {
        match category {
            Self::Installer => "Installer",
            Self::Oneshot => "Oneshot",
            Self::Website => "Website",
            Self::EnableDisable => "Enable|Disable",
            Self::Launcher => "Launcher",
            }
        }
}

pub struct DownloadFractionInfo {
    pub fraction: f64,
    pub filenumber: u32, 
    pub countfiles: u32, 
}

impl DownloadFractionInfo {
    pub fn new(fraction: f64,filenumber: u32,countfiles: u32) -> Self {
        Self {
            fraction,
            filenumber, 
            countfiles, 
        }
    }
}

#[derive(Debug,Clone)]
pub struct TempFileDirPath {
    pub file_path: String,
    pub dir_path: String,
}
pub enum OutMesseageType {
    Message(String),
    State(bool),
    Error,
    Cancelled,
    DownloadError,
    DownloadCancelled,
    Progress(DownloadFractionInfo),
    DownloadState(Option<Vec<TempFileDirPath>>),
    }

pub trait PluginTools: Send + Sync {
    fn need_install(&self) -> bool; 
    fn download_files(&self,sender:UnboundedSender<OutMesseageType>,cancellable:gio::Cancellable) ; 
    fn install(&self,sender:UnboundedSender<OutMesseageType>,cancellable:gio::Cancellable ,downloads_files_info: Option<Vec<TempFileDirPath>>);
    fn remove(&self,sender:UnboundedSender<OutMesseageType>,cancellable:gio::Cancellable);
    fn metadata(&self) -> &PluginMetaData;
    fn get_need_install(&self) -> bool ;
    fn set_need_install(&mut self,_:bool);
    fn set_install_is_running(&mut self,_:bool);
    fn get_install_is_running(&self) -> bool;
    fn get_downlods_files_length(&self) -> usize;
}


#[derive(Debug)]
pub struct PluginMetaData {
    pub install_in_queue :bool,
    pub yes_or_no: bool,
    pub if_true_skip: bool,
    pub type_: PluginType,
    pub arch: &'static [&'static str],
    pub distro_name: &'static [&'static str],
    pub distro_version: &'static [&'static str],
    pub category: Category,
    pub desktop_env:  &'static [&'static str],
    pub display_type: &'static [&'static str],
    pub title: &'static str,
    pub icon_name: &'static str,
    pub subtitle: &'static str,
    pub button_install_label: &'static str,
    pub button_remove_label: &'static str,
    pub button_install_running_label: &'static str,
    pub button_remove_running_label: &'static str,
    pub button_waiting_label: &'static str,
    pub install_yes_or_no_header: &'static str,
    pub install_yes_or_no_label: &'static str,
    pub remove_yes_or_no_header: &'static str,
    pub remove_yes_or_no_label: &'static str,
    pub custom_cancel_warning_message: Option<&'static [&'static str]>,
    pub after_success_install_message: Option<&'static str>,
    pub after_success_remove_message: Option<&'static str>,
    pub licenses: &'static [&'static [&'static str]],
    pub website:  &'static [&'static str],
}



impl PluginTools for Box<dyn PluginTools> {
    fn download_files(&self,sender:UnboundedSender<OutMesseageType>,cancellable:gio::Cancellable) { (**self).download_files(sender,cancellable) }
    fn install(&self,sender:UnboundedSender<OutMesseageType>,cancellable:gio::Cancellable,downloads_files_info: Option<Vec<TempFileDirPath>>) { (**self).install(sender,cancellable,downloads_files_info) }
    fn remove(&self,sender:UnboundedSender<OutMesseageType>,cancellable:gio::Cancellable)  { (**self).remove(sender,cancellable) }
    fn metadata(&self) -> &PluginMetaData { (**self).metadata() }
    fn need_install(&self) -> bool { (**self).need_install() }
    fn set_need_install(&mut self, val: bool) { (**self).set_need_install(val) }
    fn get_need_install(&self) -> bool { (**self).get_need_install() }
    fn set_install_is_running(&mut self, val: bool) { (**self).set_install_is_running(val) }
    fn get_install_is_running(&self) -> bool { (**self).get_install_is_running() }
    fn get_downlods_files_length(&self) -> usize { (**self).get_downlods_files_length() }
}



#[derive(Debug)]
pub struct DnfInstaller {
    pub metadata: PluginMetaData,
    pub packages_name: &'static [&'static str],
    pub need_install: bool,
    pub install_is_running: bool,
    pub need_rpmfusion_repo: bool,
    pub run_commands_before: &'static [&'static str],
    pub run_commands_after: &'static [&'static str],
    pub files_to_dowmload: Box<[Arc<Mutex<Option<DownloadTask>>>]>,
}


impl PluginTools for DnfInstaller {
    fn need_install(&self) -> bool {
        for package_name in self.packages_name {
            let package_name = package_name.trim();
            if package_name.starts_with("/") {
                if package_name.ends_with("/") {
                    if  !utils::fs::is_dir(package_name,true) {
                         return true;
                    }
                }else{
                    if  !utils::fs::is_file(package_name,true) {
                         return true;
                    }
                }
            }else{
                if let Some(new_package_name) = package_name.strip_prefix("KEEP_") {
                    if utils::command::run_command(&format!("rpm -q {}",new_package_name)) == false {
                        return true;
                    }
                }else {
                    if utils::command::run_command(&format!("rpm -q {}",package_name)) == false {
                        return true;
                    }
                }
            }
        }
        false
    }
    fn get_downlods_files_length(&self) -> usize {
        self.files_to_dowmload.len()
    }
    fn download_files(&self,sender:UnboundedSender<OutMesseageType>,cancellable:gio::Cancellable) {
        if self.files_to_dowmload.len() > 0{
            let sender_clone = sender.clone();
            let files_to_dowmload_clone = self.files_to_dowmload.clone();
            let cancellable_clone       = cancellable.clone();
            glib::spawn_future_local(async move {
                utils::command::download_and_save_with_progress_async(
                    files_to_dowmload_clone,
                    cancellable_clone,
                    move |msg: OutMesseageType| {
                        // الـ Callback ينفذ هذا الكود مع كل رسالة جديدة
                        let _ = sender_clone.unbounded_send(msg);
                    }
                ).await;
            });
        }else {
            self.install(sender.clone(),cancellable,None);
        }
    }
    fn install(&self,sender:UnboundedSender<OutMesseageType>,cancellable:gio::Cancellable,downloads_files_info:Option<Vec<TempFileDirPath>>) {
        let mut vec_command: Vec<String> = Vec::new();
        //vec_command.push(format!("export DNF5_FORCE_INTERACTIVE=1"));
        if let Some(downlodfiles_info) = downloads_files_info {
            for (index, temp_file_dir_path) in downlodfiles_info.iter().enumerate() {
                vec_command.push(format!("export TARGET_FILE{}='{}'",index +1, temp_file_dir_path.file_path));
                vec_command.push(format!("export TARGET_DIR{}='{}'", index +1,temp_file_dir_path.dir_path));
                vec_command.push(format!("export USER_HOME_DIR='{}'",HOMEDIR.get().unwrap()));
                vec_command.push(format!("export USER_CONFIG_DIR='{}'",CONFIGDIR.get().unwrap()));
                vec_command.push(format!("export USER_DOWNLOADS_DIR='{}'",DOWNLOADSDIR.get().unwrap()));
            }
        }
        if self.need_rpmfusion_repo {
            let distro_version: &str = DISTRO_VERSION.get().unwrap();
            if utils::command::run_command("rpm -q rpmfusion-free-release rpmfusion-nonfree-release") == false {
                vec_command.push(format!("pkexec stdbuf -o1 dnf install  --best -y --nogpgcheck  --color=never \
                    http://download1.rpmfusion.org/free/fedora/rpmfusion-free-release-{}.noarch.rpm \
                    http://download1.rpmfusion.org/nonfree/fedora/rpmfusion-nonfree-release-{}.noarch.rpm",distro_version,distro_version));
            }
            vec_command.push(format!("pkexec dnf config-manager enable rpmfusion-free"));
            vec_command.push(format!("pkexec dnf config-manager enable rpmfusion-free-updates"));
            vec_command.push(format!("pkexec dnf config-manager enable rpmfusion-nonfree"));
            vec_command.push(format!("pkexec dnf config-manager enable rpmfusion-nonfree-updates"));
            vec_command.push(format!("pkexec dnf config-manager enable rpmfusion-nonfree-nvidia-driver"));
            vec_command.push(format!("pkexec dnf config-manager enable rpmfusion-nonfree-steam"));
        }
        
        
        for co in self.run_commands_before {
            vec_command.push(format!("{}",co));
        }
        for c in self.packages_name {
            let c = c.trim();
            if c.starts_with("/") || c.ends_with("/") {
                continue;
            }
            if let Some(new_package_name) = c.strip_prefix("KEEP_") {
                if utils::command::run_command(&format!("rpm -q {}",new_package_name)) == false {
                    vec_command.push(format!("pkexec stdbuf -o1  dnf install {} -y   --best --color=never",new_package_name));
                }
            }else {
                if utils::command::run_command(&format!("rpm -q {}",c)) == false {
                    vec_command.push(format!("pkexec stdbuf -o1  dnf install {} -y   --best --color=never",c));
                }
            }
        }

        for co in self.run_commands_after {
            vec_command.push(format!("{}",co));
        }
        if vec_command.len() == 1 {
            utils::command::run_command_async_with_output(&vec_command[0],sender.clone(),cancellable);
        }else {
            let temp_result: utils::fs::TempFileResult = utils::fs::get_temp_file(vec_command);
            match temp_result {
                utils::fs::TempFileResult::Admin(commands) => {
                    utils::command::run_command_async_with_output(&format!("pkexec bash {}",commands),sender.clone(),cancellable);
                },
                utils::fs::TempFileResult::NonAdmin(commands) => {
                    utils::command::run_command_async_with_output(&format!("bash {}",commands),sender.clone(),cancellable);
                        
                },
                _ =>  {let _ = sender.unbounded_send(OutMesseageType::Error);},
            }
        }

    }
    
    fn remove(&self,sender:UnboundedSender<OutMesseageType>,cancellable:gio::Cancellable) {
        let mut vec_command: Vec<String> = Vec::new();
        for c in self.packages_name {
            let c = c.trim();
            if c.starts_with("/") || c.ends_with("/") {
                continue;
            }
            if let Some(_new_package_name) = c.strip_prefix("KEEP_") {
                continue;
            }
            if utils::command::run_command(&format!("rpm -q {}",c)) == true {        
                vec_command.push(format!("pkexec rpm -ev  {} --nodeps --quiet",c));
            }
        }
        if vec_command.len() == 1 {
            utils::command::run_command_async_with_output(&vec_command[0],sender.clone(),cancellable);
        }else {
            let temp_result: utils::fs::TempFileResult = utils::fs::get_temp_file(vec_command);
            match temp_result {
                utils::fs::TempFileResult::Admin(commands) => {
                    utils::command::run_command_async_with_output(&format!("pkexec bash {}",commands),sender.clone(),cancellable);
                },
                utils::fs::TempFileResult::NonAdmin(commands) => {
                    utils::command::run_command_async_with_output(&format!("bash {}",commands),sender.clone(),cancellable);
                        
                },
                _ =>  {let _ = sender.unbounded_send(OutMesseageType::Error);},
            }
        }

    }

         
    fn metadata(&self) -> &PluginMetaData {
        &self.metadata
    }
    
    fn get_need_install(&self) -> bool { 
        self.need_install
    }
    
    fn set_need_install(&mut self,value: bool) { 
        self.need_install = value;
    }
    fn get_install_is_running(&self) -> bool { 
        self.install_is_running
    }
    fn set_install_is_running(&mut self,value: bool) { 
        self.install_is_running = value;
    }
}

impl DnfInstaller {
    pub fn create(plugin: PluginMetaData,
                          packages_name: &'static [&'static str],
                          need_rpmfusion_repo: bool,
                          run_commands_before: &'static [&'static str],
                          run_commands_after : &'static [&'static str],
                          files_to_dowmload: Box<[Arc<Mutex<Option<DownloadTask>>>]>) -> Self {
        Self {
             metadata            : plugin,
             packages_name       : packages_name,
             need_install        : false,
             install_is_running  : false,
             need_rpmfusion_repo : need_rpmfusion_repo,
             run_commands_before : run_commands_before,
             run_commands_after  : run_commands_after,
             files_to_dowmload   : files_to_dowmload
            
            }
        
        }
}


#[derive(Debug)]
pub struct FlatpakInstaller {
    pub metadata: PluginMetaData,
    pub packages_name: &'static [&'static str],
    pub need_install: bool,
    pub install_is_running: bool,
    pub run_commands_before: &'static [&'static str],
    pub run_commands_after: &'static [&'static str],
    pub files_to_dowmload: Box<[Arc<Mutex<Option<DownloadTask>>>]>,
}


impl PluginTools for FlatpakInstaller {
    fn need_install(&self) -> bool {
        for package_name in self.packages_name {
            let package_name = package_name.trim();
            if package_name.starts_with("/") {
                if package_name.ends_with("/") {
                    if  !utils::fs::is_dir(package_name,true) {
                         return true;
                    }
                }else{
                    if  !utils::fs::is_file(package_name,true) {
                         return true;
                    }
                }
            }else{
                if let Some(new_package_name) = package_name.strip_prefix("pkexec") {
                    if !utils::command::run_command(&format!("flatpak  info {}",new_package_name)) {
                        return true;
                    }
                }else{
                    if !utils::command::run_command(&format!("flatpak  info {}",package_name)) {
                        return true;
                    }
                }
            }
        }
    false
    }
    fn get_downlods_files_length(&self) -> usize {
        self.files_to_dowmload.len()
    }
    fn download_files(&self,sender:UnboundedSender<OutMesseageType>,cancellable:gio::Cancellable) {
        if self.files_to_dowmload.len() > 0{
            let sender_clone = sender.clone();
            let files_to_dowmload_clone = self.files_to_dowmload.clone();
            let cancellable_clone       = cancellable.clone();
            glib::spawn_future_local(async move {
                utils::command::download_and_save_with_progress_async(
                    files_to_dowmload_clone,
                    cancellable_clone,
                    move |msg: OutMesseageType| {
                        // الـ Callback ينفذ هذا الكود مع كل رسالة جديدة
                        let _ = sender_clone.unbounded_send(msg);
                    }
                ).await;
            });
        }else {
            self.install(sender.clone(),cancellable,None);
        }
    }
    fn install(&self,sender:UnboundedSender<OutMesseageType>,cancellable:gio::Cancellable,downloads_files_info:Option<Vec<TempFileDirPath>>) {
        let mut vec_command: Vec<String> = Vec::new();
        if let Some(downlodfiles_info) = downloads_files_info {
            for (index, temp_file_dir_path) in downlodfiles_info.iter().enumerate() {
                vec_command.push(format!("export TARGET_FILE{}='{}'",index +1, temp_file_dir_path.file_path));
                vec_command.push(format!("export TARGET_DIR{}='{}'", index +1,temp_file_dir_path.dir_path));
                vec_command.push(format!("export USER_HOME_DIR='{}'",HOMEDIR.get().unwrap()));
                vec_command.push(format!("export USER_CONFIG_DIR='{}'",CONFIGDIR.get().unwrap()));
                vec_command.push(format!("export USER_DOWNLOADS_DIR='{}'",DOWNLOADSDIR.get().unwrap()));
            }
        }
        for co in self.run_commands_before {
            vec_command.push(format!("{}",co));
        }
        for package_name in self.packages_name {
            let package_name = package_name.trim();
            if package_name.starts_with("/") || package_name.ends_with("/") {
                continue;
            } else{
                let (flathub_repo,repo) = {
                    if package_name.ends_with("beta") {
                        ("flathub-beta","beta-repo")
                    }else{
                        ("flathub","repo")
                    }
                };
                if let Some(new_package_name) = package_name.strip_prefix("pkexec"){
                    vec_command.push(format!("pkexec flatpak remote-add --if-not-exists {0} https://dl.flathub.org/{1}/{0}.flatpakrepo",flathub_repo,repo));
                    vec_command.push(format!("pkexec flatpak  install {} {} -y ",flathub_repo,new_package_name));
                }else{
                    vec_command.push(format!("pkexec flatpak remote-add --if-not-exists {0} https://dl.flathub.org/{1}/{0}.flatpakrepo --user",flathub_repo,repo));
                    vec_command.push(format!("flatpak  --user install {} {} -y ",flathub_repo,package_name));
                }
            }
        }
        for co in self.run_commands_after {
            vec_command.push(format!("{}",co));
        }
        if vec_command.len() == 1 {
            utils::command::run_command_async_with_output(&vec_command[0],sender.clone(),cancellable);
        }else {
            let temp_result: utils::fs::TempFileResult = utils::fs::get_temp_file(vec_command);
            match temp_result {
                utils::fs::TempFileResult::Admin(commands) => {
                    utils::command::run_command_async_with_output(&format!("pkexec bash {}",commands),sender.clone(),cancellable);
                },
                utils::fs::TempFileResult::NonAdmin(commands) => {
                    utils::command::run_command_async_with_output(&format!("bash {}",commands),sender.clone(),cancellable);
                        
                },
                _ =>  {let _ = sender.unbounded_send(OutMesseageType::Error);},
            }
        }
    }
    
    fn remove(&self,sender:UnboundedSender<OutMesseageType>,cancellable:gio::Cancellable) {
        let mut vec_command: Vec<String> = Vec::new();
        for package_name in self.packages_name {
            let package_name = package_name.trim();
            if package_name.starts_with("/") || package_name.ends_with("/") {
                continue;
            }
            if let Some(new_package_name) = package_name.strip_prefix("pkexec"){
                if utils::command::run_command(&format!("flatpak  --system info {}",new_package_name)){
                    vec_command.push(format!("pkexec flatpak   uninstall  {} -y --noninteractive",new_package_name));
                }else {
                    vec_command.push(format!("flatpak  --user uninstall  {} -y --noninteractive",new_package_name));
                }
            }else {
                if utils::command::run_command(&format!("flatpak  --system info {}",package_name)){
                    vec_command.push(format!("pkexec flatpak   uninstall  {} -y --noninteractive",package_name));
                }else {
                    vec_command.push(format!("flatpak  --user uninstall  {} -y --noninteractive",package_name));
                }
            }
        }
        
        if vec_command.len() == 1 {
            utils::command::run_command_async_with_output(&vec_command[0],sender.clone(),cancellable);
        }else {
            let temp_result: utils::fs::TempFileResult = utils::fs::get_temp_file(vec_command);
            match temp_result {
                utils::fs::TempFileResult::Admin(commands) => {
                    utils::command::run_command_async_with_output(&format!("pkexec bash {}",commands),sender.clone(),cancellable);
                },
                utils::fs::TempFileResult::NonAdmin(commands) => {
                    utils::command::run_command_async_with_output(&format!("bash {}",commands),sender.clone(),cancellable);
                        
                },
                _ =>  {let _ = sender.unbounded_send(OutMesseageType::Error);},
            }
        }
    }
         
    fn metadata(&self) -> &PluginMetaData {
        &self.metadata
    }
    
    fn get_need_install(&self) -> bool { 
        self.need_install
    }
    
    fn set_need_install(&mut self,value: bool) { 
        self.need_install = value;
    }
    fn get_install_is_running(&self) -> bool { 
        self.install_is_running
    }
    
    fn set_install_is_running(&mut self,value: bool) { 
        self.install_is_running = value;
    }
}


impl FlatpakInstaller {
    pub fn create(plugin: PluginMetaData,
                          packages_name: &'static [&'static str],
                          run_commands_before: &'static [&'static str],
                          run_commands_after: &'static [&'static str],
                          files_to_dowmload: Box<[Arc<Mutex<Option<DownloadTask>>>]>) -> Self {
        Self {
             metadata      : plugin,
             packages_name : packages_name,
             need_install  : false,
             install_is_running  : false,
             run_commands_before : run_commands_before,
             run_commands_after  : run_commands_after,
             files_to_dowmload   : files_to_dowmload 
            
            }
        
        }
}



#[derive(Debug)]
pub struct CustomInstaller {
    pub metadata: PluginMetaData,
    pub need_install: bool,
    pub install_is_running: bool,
    pub commands_to_check: &'static [&'static str],
    pub commands_to_run_install: &'static [&'static str],
    pub commands_to_run_remove: &'static [&'static str],
    pub files_to_dowmload: Box<[Arc<Mutex<Option<DownloadTask>>>]>,
}


impl PluginTools for CustomInstaller {
    fn need_install(&self) -> bool {
        for c in self.commands_to_check {
            if c.starts_with("/") {
                if c.ends_with("/") {
                    if  !utils::fs::is_dir(c,true) {
                         return true;
                    }
                }else{
                    if  !utils::fs::is_file(c,true) {
                         return true;
                    }
                }
            }else{
                if utils::command::run_command(&format!("{}",c)) == false {
                    return true;
                }
            }
        }
    false
    }
    fn get_downlods_files_length(&self) -> usize {
        self.files_to_dowmload.len()
    }
    fn download_files(&self,sender:UnboundedSender<OutMesseageType>,cancellable:gio::Cancellable) {
        if self.files_to_dowmload.len() > 0{
            let sender_clone = sender.clone();
            let files_to_dowmload_clone = self.files_to_dowmload.clone();
            let cancellable_clone       = cancellable.clone();
            glib::spawn_future_local(async move {
                utils::command::download_and_save_with_progress_async(
                    files_to_dowmload_clone,
                    cancellable_clone,
                    move |msg: OutMesseageType| {
                        // الـ Callback ينفذ هذا الكود مع كل رسالة جديدة
                        let _ = sender_clone.unbounded_send(msg);
                    }
                ).await;
            });
        }else {
            self.install(sender.clone(),cancellable,None);
        }
    }
    fn install(&self,sender:UnboundedSender<OutMesseageType>,cancellable:gio::Cancellable,downloads_files_info:Option<Vec<TempFileDirPath>>) {
        let mut vec_command: Vec<String> = Vec::new();
        if let Some(downlodfiles_info) = downloads_files_info {
            for (index, temp_file_dir_path) in downlodfiles_info.iter().enumerate() {
                vec_command.push(format!("export TARGET_FILE{}='{}'",index +1, temp_file_dir_path.file_path));
                vec_command.push(format!("export TARGET_DIR{}='{}'", index +1,temp_file_dir_path.dir_path));
                vec_command.push(format!("export USER_HOME_DIR='{}'",HOMEDIR.get().unwrap()));
                vec_command.push(format!("export USER_CONFIG_DIR='{}'",CONFIGDIR.get().unwrap()));
                vec_command.push(format!("export USER_DOWNLOADS_DIR='{}'",DOWNLOADSDIR.get().unwrap()));
            }
        }
        for co in self.commands_to_run_install {
            vec_command.push(format!("{}",co));
        }

        if vec_command.len() == 1 {
            utils::command::run_command_async_with_output(&vec_command[0],sender.clone(),cancellable);
        }else {
            let temp_result: utils::fs::TempFileResult = utils::fs::get_temp_file(vec_command);
            match temp_result {
                utils::fs::TempFileResult::Admin(commands) => {
                    utils::command::run_command_async_with_output(&format!("pkexec bash {}",commands),sender.clone(),cancellable);
                },
                utils::fs::TempFileResult::NonAdmin(commands) => {
                    utils::command::run_command_async_with_output(&format!("bash {}",commands),sender.clone(),cancellable);
                        
                },
                _ =>  {let _ = sender.unbounded_send(OutMesseageType::Error);},
            }
        }
    }
    
    fn remove(&self,sender:UnboundedSender<OutMesseageType>,cancellable:gio::Cancellable) {
        let mut vec_command: Vec<String> = Vec::new();
        for co in self.commands_to_run_remove {
            vec_command.push(format!("{}",co));
        }
        if vec_command.len() == 1 {
            utils::command::run_command_async_with_output(&vec_command[0],sender.clone(),cancellable);
        }else {
            let temp_result: utils::fs::TempFileResult = utils::fs::get_temp_file(vec_command);
            match temp_result {
                utils::fs::TempFileResult::Admin(commands) => {
                    utils::command::run_command_async_with_output(&format!("pkexec bash {}",commands),sender.clone(),cancellable);
                },
                utils::fs::TempFileResult::NonAdmin(commands) => {
                    utils::command::run_command_async_with_output(&format!("bash {}",commands),sender.clone(),cancellable);
                        
                },
                _ =>  {let _ = sender.unbounded_send(OutMesseageType::Error);},
            }
        }
    }
         
    fn metadata(&self) -> &PluginMetaData {
        &self.metadata
    }
    
    fn get_need_install(&self) -> bool { 
        self.need_install
    }
    
    fn set_need_install(&mut self,value: bool) { 
        self.need_install = value;
    }
    fn get_install_is_running(&self) -> bool { 
        self.install_is_running
    }
    
    fn set_install_is_running(&mut self,value: bool) { 
        self.install_is_running = value;
    }
}


impl CustomInstaller {
    pub fn create(plugin: PluginMetaData,
                          commands_to_check: &'static [&'static str],
                          commands_to_run_install: &'static [&'static str],
                          commands_to_run_remove: &'static [&'static str],
                          files_to_dowmload: Box<[Arc<Mutex<Option<DownloadTask>>>]>) -> Self {
        Self {
             metadata      : plugin,
             need_install  : false,
             install_is_running  : false,
             commands_to_check   : commands_to_check,
             commands_to_run_install : commands_to_run_install,
             commands_to_run_remove  : commands_to_run_remove,
             files_to_dowmload : files_to_dowmload
            
            }
        
        }
}

