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
                                title                          : "Arduino IDE",
                                button_install_label           : "Install",
                                button_remove_label            : "Remove",
                                button_install_running_label   : "Install Running",
                                button_remove_running_label    : "Remove Running",
                                button_waiting_label           : "Waiting...",
                                install_yes_or_no_header       : "Run Install Arduino IDE Task",
                                install_yes_or_no_label        : "Start Install Arduino IDE Task?",
                                remove_yes_or_no_header        : "Run Remove Arduino IDE Task",
                                remove_yes_or_no_label         : "Start Remove Arduino IDE Task?",
                                custom_cancel_warning_message  : None,
                                after_success_install_message  : None,
                                after_success_remove_message   : None,
                                subtitle                       : "Open-source electronics prototyping platform\n(Flatpak User Wide)",
                                icon_name                      : "arduino.svg",
                                licenses                       : &[&["License\nLGPL V2.1","https://www.gnu.org/licenses/old-licenses/lgpl-2.1.html"]],
                                website                        : &["WebSite","https://www.arduino.cc/"],
    };
                            

   FlatpakInstaller::create(metadataplugin,
                         &["cc.arduino.arduinoide"],
                         &[],
                         &[],
                         Box::new([])
                         )


}

 
