use crate::baseplugin::base::{PluginMetaData,Category,PluginType,CustomInstaller, DownloadTask};
use std::sync::{Arc, Mutex};



pub fn get_plugin() -> CustomInstaller {
    let metadataplugin:PluginMetaData =  PluginMetaData {
                                install_in_queue               : true,
                                yes_or_no                      : false,
                                if_true_skip                   : false,
                                type_                          : PluginType::Installer,
                                arch                           : &["x86_64"],
                                distro_name                    : &["fedora"],
                                distro_version                 : &["all"],
                                category                       : Category::Other,
                                desktop_env                    : &["all"],
                                display_type                   : &["all"],
                                title                          : "TeamViewer",
                                button_install_label           : "Install",
                                button_remove_label            : "Remove",
                                button_install_running_label   : "Install Running",
                                button_remove_running_label    : "Remove Running",
                                button_waiting_label           : "Waiting...",
                                install_yes_or_no_header       : "",
                                install_yes_or_no_label        : "",
                                remove_yes_or_no_header        : "",
                                remove_yes_or_no_label         : "",
                                custom_cancel_warning_message  : None,
                                after_success_install_message  : None,
                                after_success_remove_message   : None,
                                subtitle                       : "Remote access, control and support software",
                                icon_name                      : "Team_Viewer.png",
                                licenses                       : &[&["License\nProprietary","https://www.teamviewer.com/"]],
                                website                        : &["WebSite","https://www.teamviewer.com/"],
    };
                            
    let download_task = DownloadTask {
        link: "https://download.teamviewer.com/download/linux/teamviewer.x86_64.rpm",
        
        // Download location. 
        // If None, a random temporary folder in /tmp will be created.
        // You can also use paths like Some("/tmp") or Some(DOWNLOADSDIR.get().unwrap()) or Some(user_pictures_location)
        dir_download_location: None, 
        
        // The file name to save the download as.
        file_name: "teamviewer.x86_64.rpm", 
    };
   CustomInstaller::create(metadataplugin,
                         &["rpm -q teamviewer"],
                         &["pkexec rpm --import https://linux.teamviewer.com/pubkey/currentkey.asc","pkexec dnf install \"$TARGET_FILE1\" --best -y   --color=never"], 
                         &["pkexec rpm -ev  teamviewer  --nodeps --quiet"],
                         Box::new([Arc::new(Mutex::new(Some(download_task)))])
                         )


}

 
