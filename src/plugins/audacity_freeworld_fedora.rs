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
                                category                       : Category::Multimedia,
                                desktop_env                    : &["all"],
                                display_type                   : &["all"],
                                title                          : "Audacity",
                                button_install_label           : "Install",
                                button_remove_label            : "Remove",
                                button_install_running_label   : "Install Running",
                                button_remove_running_label    : "Remove Running",
                                button_waiting_label           : "Waiting...",
                                install_yes_or_no_header       : "Run Install Audacity Freeworld Task",
                                install_yes_or_no_label        : "Start Install Audacity Freeworld Task?",
                                remove_yes_or_no_header        : "Run Remove Audacity Freeworld Task",
                                remove_yes_or_no_label         : "Start Remove Audacity Freeworld Task?",
                                custom_cancel_warning_message  : None,
                                after_success_install_message  : None,
                                after_success_remove_message   : None,
                                subtitle                       : "Record and edit audio files (freeworld version)",
                                icon_name                      : "audacity.png",
                                licenses                       : &[&["License\nUNKNOWN","https://www.audacityteam.org/"]],
                                website                        : &["WebSite","https://www.audacityteam.org/"],
    };
                            

   DnfInstaller::create(metadataplugin,
                         &["audacity-freeworld"],
                         true, // install and enable rpmfusion  first 
                         // command run before install audacity-freeworld to remove audacity package first (|| true) to force return 0 if audacity not installed
                         &["rpm -v --nodeps -e audacity || true"], 
                         &[],
                         Box::new([])
                         )


}

 
