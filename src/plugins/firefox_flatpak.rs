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
                                category                       : Category::Internet,
                                desktop_env                    : &["all"],
                                display_type                   : &["all"],
                                title                          : "Firefox",
                                button_install_label           : "Install",
                                button_remove_label            : "Remove",
                                button_install_running_label   : "Install Running",
                                button_remove_running_label    : "Remove Running",
                                button_waiting_label           : "Waiting...",
                                install_yes_or_no_header       : "Run Install Firefox Task",
                                install_yes_or_no_label        : "Start Install Firefox Task?",
                                remove_yes_or_no_header        : "Run Remove Firefox Task",
                                remove_yes_or_no_label         : "Start Remove Firefox Task?",
                                custom_cancel_warning_message  : None,
                                after_success_install_message  : None,
                                after_success_remove_message   : None,
                                subtitle                       : "Fast, Private and Safe Web Browser\n(Flatpak User Wide)",
                                icon_name                      : "appicns_Firefox.png",
                                licenses                       : &[&["License\nMPL-2.0","https://www.mozilla.org/en-US/MPL/2.0/"]],
                                website                        : &["WebSite","https://www.mozilla.org/en-US/firefox/"],
    };
                            

   FlatpakInstaller::create(metadataplugin,
                         &["org.mozilla.firefox"],
                         &[],
                         &[],
                         Box::new([])
                         )


}

 
