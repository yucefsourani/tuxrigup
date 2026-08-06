use crate::baseplugin::base::{PluginMetaData,Category,PluginType,DnfInstaller};




pub fn get_plugin() -> DnfInstaller {
    let metadataplugin:PluginMetaData =  PluginMetaData {
                                install_in_queue               : true,
                                yes_or_no                      : false,
                                if_true_skip                   : false,
                                type_                          : PluginType::Installer,
                                arch                           : &["all"],
                                distro_name                    : &["fedora"],
                                distro_version                 : &["all"],
                                category                       : Category::Graphics,
                                desktop_env                    : &["all"],
                                display_type                   : &["all"],
                                title                          : "Gimp",
                                button_install_label           : "Install",
                                button_remove_label            : "Remove",
                                button_install_running_label   : "Install Running",
                                button_remove_running_label    : "Remove Running",
                                button_waiting_label           : "Waiting...",
                                install_yes_or_no_header       : "Run Install Gimp Task",
                                install_yes_or_no_label        : "Start Install Gimp Task?",
                                remove_yes_or_no_header        : "Run Remove Gimp Task",
                                remove_yes_or_no_label         : "Start Remove Gimp Task?",
                                custom_cancel_warning_message  : None,
                                after_success_install_message  : None,
                                after_success_remove_message   : None,
                                subtitle                       : "GIMP is an acronym for GNU Image Manipulation Program (Fedora rpm)",
                                icon_name                      : "gimp.png",
                                licenses                       : &[&["License\nGPL v3.0+","https://www.gnu.org/licenses/gpl-3.0.html"],&["License\nLGPL-3.0+","https://www.gnu.org/licenses/lgpl-3.0.en.html"]],
                                website                        : &["WebSite","https://www.gimp.org/"],
    };
                            

   DnfInstaller::create(metadataplugin,
                         &["gimp"],
                         false, // install and enable rpmfusion  first 
                         &[], 
                         &[],
                         Box::new([])
                         )


}

 
