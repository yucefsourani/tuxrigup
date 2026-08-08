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
                                title                          : "Opera",
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
                                subtitle                       : "Opera web browser",
                                icon_name                      : "Opera-logo-256x256.png",
                                licenses                       : &[&["License\nUNKNOWN","https://www.opera.com/"]],
                                website                        : &["WebSite","https://www.opera.com/"],
    };
                            

   DnfInstaller::create(metadataplugin,
                         &["opera-stable"],
                         false, // install and enable rpmfusion  first 
                         // command run before install
                         &[
                            "pkexec echo  -e  '[opera]\nname=Opera packages\ntype=rpm-md\nbaseurl=https://rpm.opera.com/rpm\ngpgcheck=1\ngpgkey=https://rpm.opera.com/rpmrepo.key\nenabled=1'  >  /etc/yum.repos.d/opera.repo",
                         ], 
                         &[],
                         Box::new([])
                         )


}

 
