use crate::baseplugin::base::{PluginMetaData,Category,PluginType,CustomInstaller,DownloadTask};
use std::sync::{Arc,Mutex};



pub fn get_plugin() -> CustomInstaller {
    let metadataplugin:PluginMetaData =  PluginMetaData {
                                install_in_queue               : false,
                                yes_or_no                      : true,
                                if_true_skip                   : false,
                                type_                          : PluginType::Installer,
                                arch                           : &["all"],
                                distro_name                    : &["all"],
                                distro_version                 : &["all"],
                                category                       : Category::Internet,
                                desktop_env                    : &["all"],
                                display_type                   : &["all"],
                                title                          : "Xdman",
                                button_install_label           : "Install",
                                button_remove_label            : "Remove",
                                button_install_running_label   : "Install Running",
                                button_remove_running_label    : "Remove Running",
                                button_waiting_label           : "Waiting...",
                                yes_or_no_header               : "Run Xdman Task",
                                yes_or_no_label                : "Start Install/Remove Xdman Task?",
                                custom_cancel_warning_message  : None,
                                after_success_install_message  : None,
                                after_success_remove_message   : None,
                                subtitle                       : "Xtreme Download Manager",
                                icon_name                      : "brave-origin.png",
                                keywords                       : "downloder xdman",
                                licenses                       : &[&["License\nGPL V2.0","https://www.gnu.org/licenses/gpl-2.0.html"]],
                                website                        : &["WebSite","https://github.com/subhra74/xdm"],
    };
                            
    let download_task1 = DownloadTask {
                                        link: "https://github.com/subhra74/xdm/releases/download/7.2.11/xdm-setup-7.2.11.tar.xz",
                                        dir_download_location: None,
                                        file_name: "xdm-setup-7.2.11.tar.xz",
                                    };

   CustomInstaller::create(metadataplugin,
                         &["ls /opt/xdman/uninstall.sh"],
                         &["tar -xJf \"$TARGET_FILE1\" -C  \"$TARGET_DIR1\"","chmod 755  \"$TARGET_DIR1\"/install.sh","pkexec \"$TARGET_DIR1\"/install.sh"],
                         &["pkexec chmod 755 /opt/xdman/uninstall.sh", "pkexec /opt/xdman/uninstall.sh"],
                         Box::new([Arc::new(Mutex::new(Some(download_task1)))])
                         )


}
/*

    
    link plugin
    
    launcher plugin 
    
    about 
    
    settings
    
    

 
 */
 
