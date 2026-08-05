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
                                desktop_env                    : &["plasma"],
                                display_type                   : &["all"],
                                title                          : "Ffmpegthumbs",
                                button_install_label           : "Install",
                                button_remove_label            : "Remove",
                                button_install_running_label   : "Install Running",
                                button_remove_running_label    : "Remove Running",
                                button_waiting_label           : "Waiting...",
                                install_yes_or_no_header       : "Run Install ffmpegthumbs Task",
                                install_yes_or_no_label        : "Start Install ffmpegthumbs Task?",
                                remove_yes_or_no_header        : "Run Remove ffmpegthumbs Task",
                                remove_yes_or_no_label         : "Start Remove ffmpegthumbs Task?",
                                custom_cancel_warning_message  : None,
                                after_success_install_message  : None,
                                after_success_remove_message   : None,
                                subtitle                       : "For KDE Plasma File Manager Video Thumbnails",
                                icon_name                      : "81dz0FkILtL.png",
                                licenses                       : &[&["License\nUNKNOWN",""]],
                                website                        : &["WebSite","https://apps.kde.org/ffmpegthumbs/"],
    };
                            

   DnfInstaller::create(metadataplugin,
                         &["ffmpegthumbs"],
                         true, // install and enable rpmfusion  first 
                         &[], 
                         &[],
                         Box::new([])
                         )


}

 
