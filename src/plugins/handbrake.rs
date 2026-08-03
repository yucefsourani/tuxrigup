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
                                title                          : "HandBrake",
                                button_install_label           : "Install",
                                button_remove_label            : "Remove",
                                button_install_running_label   : "Install Running",
                                button_remove_running_label    : "Remove Running",
                                button_waiting_label           : "Waiting...",
                                install_yes_or_no_header       : "Run Install HandBrake Task",
                                install_yes_or_no_label        : "Start Install HandBrake Task?",
                                remove_yes_or_no_header        : "Run Remove HandBrake Task",
                                remove_yes_or_no_label         : "Start Remove HandBrake Task?",
                                custom_cancel_warning_message  : None,
                                after_success_install_message  : None,
                                after_success_remove_message   : None,
                                subtitle                       : "Video Transcoder\n(Flatpak User Wide)",
                                icon_name                      : "HandBrake_Icon.png",
                                licenses                       : &[&["License\nGPL V2.0+","https://www.gnu.org/licenses/old-licenses/gpl-2.0.html"]],
                                website                        : &["WebSite","https://handbrake.fr/"],
    };
                            

   FlatpakInstaller::create(metadataplugin,
                         &["fr.handbrake.ghb"],
                         &[],
                         &[],
                         Box::new([])
                         )


}

 
