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
                                category                       : Category::Utility,
                                desktop_env                    : &["all"],
                                display_type                   : &["all"],
                                title                          : "Gradia",
                                button_install_label           : "Install",
                                button_remove_label            : "Remove",
                                button_install_running_label   : "Install Running",
                                button_remove_running_label    : "Remove Running",
                                button_waiting_label           : "Waiting...",
                                install_yes_or_no_header       : "Run Install Gradia Task",
                                install_yes_or_no_label        : "Start Install Gradia Task?",
                                remove_yes_or_no_header        : "Run Remove Gradia Task",
                                remove_yes_or_no_label         : "Start Remove Gradia Task?",
                                custom_cancel_warning_message  : None,
                                after_success_install_message  : None,
                                after_success_remove_message   : None,
                                subtitle                       : "Gradia helps you get your screenshots ready for sharing, whether quickly with friends or colleagues, or professionally with the entire world.\n(Flatpak User Wide)",
                                icon_name                      : "be.alexandervanhee.gradia.Source.png",
                                licenses                       : &[&["License\nGPL V3.0","https://www.gnu.org/licenses/gpl-3.0.html"]],
                                website                        : &["WebSite","https://gradia.alexandervanhee.be/"],
    };
                            

   FlatpakInstaller::create(metadataplugin,
                         &["be.alexandervanhee.gradia"],
                         &[],
                         &[],
                         Box::new([])
                         )


}

 
