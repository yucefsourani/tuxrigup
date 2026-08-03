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
                                title                          : "Marker",
                                button_install_label           : "Install",
                                button_remove_label            : "Remove",
                                button_install_running_label   : "Install Running",
                                button_remove_running_label    : "Remove Running",
                                button_waiting_label           : "Waiting...",
                                install_yes_or_no_header       : "Run Install Marker Task",
                                install_yes_or_no_label        : "Start Install Marker Task?",
                                remove_yes_or_no_header        : "Run Remove Marker Task",
                                remove_yes_or_no_label         : "Start Remove Marker Task?",
                                custom_cancel_warning_message  : None,
                                after_success_install_message  : None,
                                after_success_remove_message   : None,
                                subtitle                       : "Powerful markdown editor for the GNOME desktop\n(Flatpak User Wide)",
                                icon_name                      : "com.github.fabiocolacio.marker.svg",
                                licenses                       : &[&["License\nGPL V3.0","https://www.gnu.org/licenses/gpl-3.0.html"]],
                                website                        : &["WebSite","https://github.com/fabiocolacio/Marker"],
    };
                            

   FlatpakInstaller::create(metadataplugin,
                         &["com.github.fabiocolacio.marker"],
                         &[],
                         &[],
                         Box::new([])
                         )


}

 
