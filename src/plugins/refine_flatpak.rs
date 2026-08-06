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
                                category                       : Category::Gnome,
                                desktop_env                    : &["gnome"],
                                display_type                   : &["all"],
                                title                          : "Refine",
                                button_install_label           : "Install",
                                button_remove_label            : "Remove",
                                button_install_running_label   : "Install Running",
                                button_remove_running_label    : "Remove Running",
                                button_waiting_label           : "Waiting...",
                                install_yes_or_no_header       : "Run Install Refine Task",
                                install_yes_or_no_label        : "Start Install Refine Task?",
                                remove_yes_or_no_header        : "Run Remove Refine Task",
                                remove_yes_or_no_label         : "Start Remove Refine Task?",
                                custom_cancel_warning_message  : None,
                                after_success_install_message  : None,
                                after_success_remove_message   : None,
                                subtitle                       : "Refine helps discover advanced and experimental features in GNOME.\n(Flatpak User Wide)",
                                icon_name                      : "page.tesk.Refine.svg",
                                licenses                       : &[&["License\nGPL V3.0","https://www.gnu.org/licenses/gpl-3.0.html"]],
                                website                        : &["WebSite","https://tesk.page/refine/"],
    };
                            

   FlatpakInstaller::create(metadataplugin,
                         &["page.tesk.Refine"],
                         &[],
                         &[],
                         Box::new([])
                         )


}

 
