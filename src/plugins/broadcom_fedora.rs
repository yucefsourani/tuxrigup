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
                                category                       : Category::System,
                                desktop_env                    : &["all"],
                                display_type                   : &["all"],
                                title                          : "Broadcom",
                                button_install_label           : "Install",
                                button_remove_label            : "Remove",
                                button_install_running_label   : "Install Running",
                                button_remove_running_label    : "Remove Running",
                                button_waiting_label           : "Waiting...",
                                install_yes_or_no_header       : "Run Install Broadcom Drive Task",
                                install_yes_or_no_label        : "Start Install Broadcom Drive Task?",
                                remove_yes_or_no_header        : "Run Remove Broadcom Drive Task",
                                remove_yes_or_no_label         : "Start Remove Broadcom Drive Task?",
                                custom_cancel_warning_message  : None,
                                after_success_install_message  : None,
                                after_success_remove_message   : None,
                                subtitle                       : "Driver For Broadcom wifi",
                                icon_name                      : "broadcom.jpg",
                                licenses                       : &[&["License\nUNKNOWN","https://www.broadcom.com/"]],
                                website                        : &["WebSite","https://www.broadcom.com/"],
    };
                            

   DnfInstaller::create(metadataplugin,
                         &["akmod-wl", "broadcom-wl"],
                         true, // install and enable rpmfusion  first 
                         &[], 
                         &[],
                         Box::new([])
                         )


}

 
