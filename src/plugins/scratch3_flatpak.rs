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
                                title                          : "Scratch3",
                                button_install_label           : "Install",
                                button_remove_label            : "Remove",
                                button_install_running_label   : "Install Running",
                                button_remove_running_label    : "Remove Running",
                                button_waiting_label           : "Waiting...",
                                install_yes_or_no_header       : "Run Install Scratch3 Task",
                                install_yes_or_no_label        : "Start Install Scratch3 Task?",
                                remove_yes_or_no_header        : "Run Remove Scratch3 Task",
                                remove_yes_or_no_label         : "Start Remove Scratch3 Task?",
                                custom_cancel_warning_message  : None,
                                after_success_install_message  : None,
                                after_success_remove_message   : None,
                                subtitle                       : "Create stories, games , and animations, share with others around the world\n(Flatpak User wide)",
                                icon_name                      : "app-icon-scratch.png",
                                licenses                       : &[&["License\nBSD-3-Clause","https://opensource.org/license/bsd-3-clause/"]],
                                website                        : &["WebSite","https://scratch.mit.edu/"],
    };
                            

   FlatpakInstaller::create(metadataplugin,
                         &["edu.mit.Scratch"],
                         &[],
                         &[],
                         Box::new([])
                         )


}

 
