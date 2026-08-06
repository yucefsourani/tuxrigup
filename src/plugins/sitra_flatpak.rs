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
                                category                       : Category::System,
                                desktop_env                    : &["all"],
                                display_type                   : &["all"],
                                title                          : "Sitra",
                                button_install_label           : "Install",
                                button_remove_label            : "Remove",
                                button_install_running_label   : "Install Running",
                                button_remove_running_label    : "Remove Running",
                                button_waiting_label           : "Waiting...",
                                install_yes_or_no_header       : "Run Install Sitra Task",
                                install_yes_or_no_label        : "Start Install Sitra Task?",
                                remove_yes_or_no_header        : "Run Remove Sitra Task",
                                remove_yes_or_no_label         : "Start Remove Sitra Task?",
                                custom_cancel_warning_message  : None,
                                after_success_install_message  : None,
                                after_success_remove_message   : None,
                                subtitle                       : "Get your fonts from online sources with a sleek, friendly user interface.\n(Flatpak User Wide)",
                                icon_name                      : "io.github.sitraorg.sitra.png",
                                licenses                       : &[&["License\nGPL v3.0","https://www.gnu.org/licenses/old-licenses/gpl-3.0.html"]],
                                website                        : &["WebSite","https://github.com/sitraorg/sitra"],
    };
                            

   FlatpakInstaller::create(metadataplugin,
                         &["io.github.sitraorg.sitra"],
                         &[],
                         &[],
                         Box::new([])
                         )


}

 
