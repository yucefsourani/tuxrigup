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
                                category                       : Category::Developertools,
                                desktop_env                    : &["all"],
                                display_type                   : &["all"],
                                title                          : "GitKraken",
                                button_install_label           : "Install",
                                button_remove_label            : "Remove",
                                button_install_running_label   : "Install Running",
                                button_remove_running_label    : "Remove Running",
                                button_waiting_label           : "Waiting...",
                                install_yes_or_no_header       : "Run Install GitKraken Task",
                                install_yes_or_no_label        : "Start Install GitKraken Task?",
                                remove_yes_or_no_header        : "Run Remove GitKraken Task",
                                remove_yes_or_no_label         : "Start Remove GitKraken Task?",
                                custom_cancel_warning_message  : None,
                                after_success_install_message  : None,
                                after_success_remove_message   : None,
                                subtitle                       : "For repo management, in-app code editing and issue tracking\n(Flatpak User Wide)",
                                icon_name                      : "GitKraken.png",
                                licenses                       : &[&["License\nProprietary","https://www.gitkraken.com/"]],
                                website                        : &["WebSite","https://www.gitkraken.com/"],
    };
                            

   FlatpakInstaller::create(metadataplugin,
                         &["com.axosoft.GitKraken"],
                         &[],
                         &[],
                         Box::new([])
                         )


}

 
