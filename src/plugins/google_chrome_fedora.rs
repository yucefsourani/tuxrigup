use crate::baseplugin::base::{PluginMetaData,Category,PluginType,DnfInstaller};




pub fn get_plugin() -> DnfInstaller {
    let metadataplugin:PluginMetaData =  PluginMetaData {
                                install_in_queue               : true,
                                yes_or_no                      : false,
                                if_true_skip                   : false,
                                type_                          : PluginType::Installer,
                                arch                           : &["x86_64"],
                                distro_name                    : &["fedora"],
                                distro_version                 : &["all"],
                                category                       : Category::Internet,
                                desktop_env                    : &["all"],
                                display_type                   : &["all"],
                                title                          : "Google Chrome",
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
                                subtitle                       : "Google Chrome Web Browser",
                                icon_name                      : "Chrome.png",
                                licenses                       : &[&["License\nUNKNOWN","https://www.google.com/chrome"]],
                                website                        : &["WebSite","https://www.google.com/chrome"],
    };
                            

   DnfInstaller::create(metadataplugin,
                         &["google-chrome-stable"],
                         true, // install and enable rpmfusion  first 
                         // command run before install
                         &[
                            "pkexec echo -e '[google-chrome]\nname=google-chrome\nbaseurl=http://dl.google.com/linux/chrome/rpm/stable/x86_64\nenabled=1\ngpgcheck=1\ngpgkey=https://dl.google.com/linux/linux_signing_key.pub' > /etc/yum.repos.d/google-chrome.repo",
                            "pkexec dnf config-manager enable google-chrome"
                         ], 
                         &[],
                         Box::new([])
                         )


}

 
