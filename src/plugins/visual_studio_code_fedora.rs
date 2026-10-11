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
                                category                       : Category::Developertools,
                                desktop_env                    : &["all"],
                                display_type                   : &["all"],
                                title                          : "Visual Studio Code",
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
                                subtitle                       : "Visual Studio Code Editor",
                                icon_name                      : "visual-studio-code-logo.png",
                                licenses                       : &[&["License\nUNKNOWN","https://code.visualstudio.com/license"]],
                                website                        : &["WebSite","https://code.visualstudio.com"],
    };
                            

   DnfInstaller::create(metadataplugin,
                         &["code"],
                         true, // install and enable rpmfusion  first 
                         // command run before install
                         &[
                            "pkexec echo -e '[code]\nname=Visual Studio Code\nbaseurl=https://packages.microsoft.com/yumrepos/vscode\nenabled=1\nautorefresh=1\ntype=rpm-md\ngpgcheck=1\ngpgkey=https://packages.microsoft.com/keys/microsoft.asc' > /etc/yum.repos.d/vscode.repo",
                            "pkexec rpm --import https://packages.microsoft.com/keys/microsoft.asc"
                         ], 
                         &[],
                         Box::new([])
                         )


}

 
