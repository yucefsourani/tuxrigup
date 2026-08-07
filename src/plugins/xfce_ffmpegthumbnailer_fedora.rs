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
                                category                       : Category::System,
                                desktop_env                    : &["xfce"],
                                display_type                   : &["all"],
                                title                          : "XFCE Video Thumbnails",
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
                                subtitle                       : "Packages For XFCE File Manager Video Thumbnails",
                                icon_name                      : "1200px-Thunar.png.png",
                                licenses                       : &[&["License\nUNKNOWN",""]],
                                website                        : &["WebSite",""],
    };
                            

   DnfInstaller::create(metadataplugin,
                         &["tumbler", "tumbler-extras", "ffmpegthumbnailer"],
                         true, // install and enable rpmfusion  first 
                         &[], 
                         &[],
                         Box::new([])
                         )


}

 
