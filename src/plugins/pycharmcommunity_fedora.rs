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
                                category                       : Category::Developertools,
                                desktop_env                    : &["all"],
                                display_type                   : &["all"],
                                title                          : "PyCharm Community",
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
                                subtitle                       : "The most intelligent Python IDE (Fedora RPM)",
                                icon_name                      : "pycharmcommunity.png",
                                licenses                       : &[&["License\nUNKNOWN","https://www.jetbrains.com/pycharm/"]],
                                website                        : &["WebSite","https://www.jetbrains.com/pycharm/"],
    };
                            

   DnfInstaller::create(metadataplugin,
                         &["pycharm-community","pycharm-community-plugins"],
                         false, // install and enable rpmfusion  first 
                         &["pkexec dnf copr enable phracek/PyCharm -y"], 
                         &[],
                         Box::new([])
                         )


}

 
