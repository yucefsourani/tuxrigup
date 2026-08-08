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
                                category                       : Category::Multimedia,
                                desktop_env                    : &["all"],
                                display_type                   : &["all"],
                                title                          : "Natron",
                                button_install_label           : "Install",
                                button_remove_label            : "Remove",
                                button_install_running_label   : "Install Running",
                                button_remove_running_label    : "Remove Running",
                                button_waiting_label           : "Waiting...",
                                install_yes_or_no_header       : "Run Install Natron Task",
                                install_yes_or_no_label        : "Start Install Natron Task?",
                                remove_yes_or_no_header        : "Run Remove Natron Task",
                                remove_yes_or_no_label         : "Start Remove Natron Task?",
                                custom_cancel_warning_message  : None,
                                after_success_install_message  : None,
                                after_success_remove_message   : None,
                                subtitle                       : "Open-source video compositing software\n(Flatpak User wide)",
                                icon_name                      : "natron.png",
                                licenses                       : &[&["License\nGPL-2.0-or-later","https://www.gnu.org/licenses/old-licenses/gpl-2.0.en.html"]],
                                website                        : &["WebSite","https://natrongithub.github.io/"],
    };
                            

   FlatpakInstaller::create(metadataplugin,
                         &["fr.natron.Natron"],
                         &[],
                         &[],
                         Box::new([])
                         )


}

 
