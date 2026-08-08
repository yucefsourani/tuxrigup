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
                                title                          : "Mpv",
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
                                subtitle                       : "Movie player playing most video formats and DVDs (Fedora RPM)",
                                icon_name                      : "mpv.png",
                                licenses                       : &[&["License\nGPLv2+","https://www.gnu.org/licenses/old-licenses/gpl-2.0.en.html"],&["License\nLGPLv2.1+","https://www.gnu.org/licenses/old-licenses/lgpl-2.1.html"]],
                                website                        : &["WebSite","https://mpv.io/"],
    };
                            

   DnfInstaller::create(metadataplugin,
                         &["mpv"],
                         false, // install and enable rpmfusion  first 
                         &[], 
                         &[],
                         Box::new([])
                         )


}

 
