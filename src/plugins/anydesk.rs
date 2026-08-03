use crate::baseplugin::base::{PluginMetaData,Category,PluginType,FlatpakInstaller};




pub fn get_plugin() -> FlatpakInstaller {
    let metadataplugin:PluginMetaData =  PluginMetaData {
                                install_in_queue               : true,
                                yes_or_no                      : true,
                                if_true_skip                   : false,
                                type_                          : PluginType::Installer,
                                arch                           : &["all"],
                                distro_name                    : &["all"],
                                distro_version                 : &["all"],
                                category                       : Category::Other,
                                desktop_env                    : &["all"],
                                display_type                   : &["all"],
                                title                          : "Anydesk",
                                button_install_label           : "Install",
                                button_remove_label            : "Remove",
                                button_install_running_label   : "Install Running",
                                button_remove_running_label    : "Remove Running",
                                button_waiting_label           : "Waiting...",
                                install_yes_or_no_header       : "Run Install Anydesk Task",
                                install_yes_or_no_label        : "Start Install Anydesk Task?",
                                remove_yes_or_no_header        : "Run Remove Anydesk Task",
                                remove_yes_or_no_label         : "Start Remove Anydesk Task?",
                                custom_cancel_warning_message  : None,
                                after_success_install_message  : None,
                                after_success_remove_message   : None,
                                subtitle                       : "Connect to a computer remotely\n(Flatpak User Wide)",
                                icon_name                      : "com.anydesk.Anydesk.png",
                                licenses                       : &[&["License\nProprietary","https://www.anydesk.com/"]],
                                website                        : &["WebSite","https://www.anydesk.com/"],
    };
                            

   FlatpakInstaller::create(metadataplugin,
                         &["com.anydesk.Anydesk"],
                         &[],
                         &[],
                         Box::new([])
                         )


}

 
