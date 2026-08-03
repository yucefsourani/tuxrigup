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
                                title                          : "VidCutter",
                                button_install_label           : "Install",
                                button_remove_label            : "Remove",
                                button_install_running_label   : "Install Running",
                                button_remove_running_label    : "Remove Running",
                                button_waiting_label           : "Waiting...",
                                install_yes_or_no_header       : "Run Install VidCutter Task",
                                install_yes_or_no_label        : "Start Install VidCutter Task?",
                                remove_yes_or_no_header        : "Run Remove VidCutter Task",
                                remove_yes_or_no_label         : "Start Remove VidCutter Task?",
                                custom_cancel_warning_message  : None,
                                after_success_install_message  : None,
                                after_success_remove_message   : None,
                                subtitle                       : "Media Cutter + Joiner\n(Flatpak User Wide)",
                                icon_name                      : "vidcutter.png",
                                licenses                       : &[&["License\nGPL V3.0+","https://www.gnu.org/licenses/gpl-3.0.html"]],
                                website                        : &["WebSite","https://github.com/ozmartian/vidcutter"],
    };
                            

   FlatpakInstaller::create(metadataplugin,
                         &["com.ozmartians.VidCutter"],
                         &[],
                         &[],
                         Box::new([])
                         )


}

 
