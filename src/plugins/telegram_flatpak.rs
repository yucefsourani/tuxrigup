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
                                title                          : "Telegram Desktop",
                                button_install_label           : "Install",
                                button_remove_label            : "Remove",
                                button_install_running_label   : "Install Running",
                                button_remove_running_label    : "Remove Running",
                                button_waiting_label           : "Waiting...",
                                install_yes_or_no_header       : "Run Install Telegram Desktop Task",
                                install_yes_or_no_label        : "Start Install Telegram Desktop Task?",
                                remove_yes_or_no_header        : "Run Remove Telegram Desktop Task",
                                remove_yes_or_no_label         : "Start Remove Telegram Desktop Task?",
                                custom_cancel_warning_message  : None,
                                after_success_install_message  : None,
                                after_success_remove_message   : None,
                                subtitle                       : "Cloud-based mobile and desktop messaging app\n(Flatpak User wide)",
                                icon_name                      : "telegram.png",
                                licenses                       : &[&["License\nGPL-3.0","https://www.gnu.org/licenses/gpl-3.0.en.html"]],
                                website                        : &["WebSite","https://desktop.telegram.org/"],
    };
                            

   FlatpakInstaller::create(metadataplugin,
                         &["org.telegram.desktop"],
                         &[],
                         &[],
                         Box::new([])
                         )


}

 
