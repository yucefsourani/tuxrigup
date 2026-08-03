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
                                title                          : "Brave Origin",
                                button_install_label           : "Install",
                                button_remove_label            : "Remove",
                                button_install_running_label   : "Install Running",
                                button_remove_running_label    : "Remove Running",
                                button_waiting_label           : "Waiting...",
                                install_yes_or_no_header       : "Run Install Brave Origin  Task",
                                install_yes_or_no_label        : "Start Install Brave Origin Browser Task?",
                                remove_yes_or_no_header        : "Run Remove Brave Origin  Task",
                                remove_yes_or_no_label         : "Start Remove Brave Origin Browser Task?",
                                custom_cancel_warning_message  : None,
                                after_success_install_message  : None,
                                after_success_remove_message   : None,
                                subtitle                       : "Brave Origin Web Browser",
                                icon_name                      : "brave_origin.png",
                                licenses                       : &[&["License\nUNKNOWN","https://brave.com/terms-of-use/"]],
                                website                        : &["WebSite","https://brave.com/"],
    };
                            

   DnfInstaller::create(metadataplugin,
                         &["brave-origin"],
                         true, // install and enable rpmfusion  first 
                         // command run before install
                         &[
                            "pkexec echo -e '[brave-browser]\nname=Brave Browser\nbaseurl=https://brave-browser-rpm-release.s3.brave.com/$basearch\nenabled=1' > /etc/yum.repos.d/brave-browser.repo",
                            "pkexec rpm --import https://brave-browser-rpm-release.s3.brave.com/brave-core.asc"
                         ], 
                         &[],
                         Box::new([])
                         )


}

 
