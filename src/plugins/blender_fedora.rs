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
                                title                          : "Blender",
                                button_install_label           : "Install",
                                button_remove_label            : "Remove",
                                button_install_running_label   : "Install Running",
                                button_remove_running_label    : "Remove Running",
                                button_waiting_label           : "Waiting...",
                                install_yes_or_no_header       : "Run Install Blender Task",
                                install_yes_or_no_label        : "Start Install Blender Task?",
                                remove_yes_or_no_header        : "Run Remove Blender Task",
                                remove_yes_or_no_label         : "Start Remove Blender Task?",
                                custom_cancel_warning_message  : None,
                                after_success_install_message  : None,
                                after_success_remove_message   : None,
                                subtitle                       : "Free and open source 3D creation suite (Fedora RPM)",
                                icon_name                      : "Blender-icon.png",
                                licenses                       : &[&["License\nGPL v3.0","https://www.gnu.org/licenses/gpl-3.0.html"]],
                                website                        : &["WebSite","https://www.blender.org/"],
    };
                            

   DnfInstaller::create(metadataplugin,
                         &["blender"],
                         false, // install and enable rpmfusion  first 
                         &[], 
                         &[],
                         Box::new([])
                         )


}

 
