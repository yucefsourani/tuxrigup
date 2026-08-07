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
                                title                          : "Krita",
                                button_install_label           : "Install",
                                button_remove_label            : "Remove",
                                button_install_running_label   : "Install Running",
                                button_remove_running_label    : "Remove Running",
                                button_waiting_label           : "Waiting...",
                                install_yes_or_no_header       : "",
                                install_yes_or_no_label        : "",
                                remove_yes_or_no_header        : "",
                                remove_yes_or_no_label         : "",
                                custom_cancel_warning_message  : None,
                                after_success_install_message  : None,
                                after_success_remove_message   : None,
                                subtitle                       : "Digital Painting, Creative Freedom (Fedora RPM)",
                                icon_name                      : "krita.png",
                                licenses                       : &[&["License\nGPL-3.0-only","https://www.gnu.org/licenses/gpl-3.0.en.html"]],
                                website                        : &["WebSite","https://krita.org/"],
    };
                            

   DnfInstaller::create(metadataplugin,
                         &["krita"],
                         false, // install and enable rpmfusion  first 
                         &[], 
                         &[],
                         Box::new([])
                         )


}

 
