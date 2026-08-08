use crate::baseplugin::base::{PluginMetaData,Category,PluginType,DnfInstaller};




pub fn get_plugin() -> DnfInstaller {
    let metadataplugin:PluginMetaData =  PluginMetaData {
                                install_in_queue               : true,
                                yes_or_no                      : false,
                                if_true_skip                   : false,
                                type_                          : PluginType::Installer,
                                arch                           : &["all"],
                                distro_name                    : &["fedora"],
                                distro_version                 : &["all"],
                                category                       : Category::Multimedia,
                                desktop_env                    : &["all"],
                                display_type                   : &["all"],
                                title                          : "Vlc",
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
                                subtitle                       : "VLC media player, the open-source multimedia player",
                                icon_name                      : "vlc-player-logo.png",
                                licenses                       : &[&["License\nGPL-2.0+","https://www.gnu.org/licenses/old-licenses/gpl-2.0.en.html"]],
                                website                        : &["WebSite","https://www.videolan.org/vlc/"],
    };
                            

   DnfInstaller::create(metadataplugin,
                         &["vlc","vlc-cli"],
                         true, // install and enable rpmfusion  first 
                         // command run before install audacity-freeworld to remove audacity package first (|| true) to force return 0 if audacity not installed
                         &[], 
                         &[],
                         Box::new([])
                         )


}

 
