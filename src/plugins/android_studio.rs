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
                                title                          : "Android Studio",
                                button_install_label           : "Install",
                                button_remove_label            : "Remove",
                                button_install_running_label   : "Install Running",
                                button_remove_running_label    : "Remove Running",
                                button_waiting_label           : "Waiting...",
                                install_yes_or_no_header       : "Run Install Android Studio Task",
                                install_yes_or_no_label        : "Start Install Android Studio Task?",
                                remove_yes_or_no_header        : "Run Remove Android Studio Task",
                                remove_yes_or_no_label         : "Start Remove Android Studio Task?",
                                custom_cancel_warning_message  : None,
                                after_success_install_message  : None,
                                after_success_remove_message   : None,
                                subtitle                       : "Integrated development environment for Google's Android platform\n(Flatpak User Wide)",
                                icon_name                      : "Android_Studio.png",
                                licenses                       : &[&["License\nProprietary","https://developer.android.com/studio/"]],
                                website                        : &["WebSite","https://developer.android.com/studio/"],
    };
                            

   FlatpakInstaller::create(metadataplugin,
                         &["com.google.AndroidStudio"],
                         &[],
                         &[],
                         Box::new([])
                         )


}

 
