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
                                title                          : "Edge",
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
                                subtitle                       : "Introducing the new Microsoft Edge web browser",
                                icon_name                      : "edge.png",
                                licenses                       : &[&["License\nUNKNOWN","https://www.microsoft.com/en-us/edge"]],
                                website                        : &["WebSite","https://www.microsoft.com/en-us/edge"],
    };
                            

   DnfInstaller::create(metadataplugin,
                         &["microsoft-edge-stable"],
                         true, // install and enable rpmfusion  first 
                         // command run before install
                         &[
                            "pkexec dnf config-manager addrepo --from-repofile=https://packages.microsoft.com/yumrepos/edge/config.repo  --save-filename=microsoft-edge --overwrite",
                            "pkexec rpm --import https://packages.microsoft.com/yumrepos/edge/repodata/repomd.xml.key",
                            "pkexec rpm --import https://packages.microsoft.com/keys/microsoft.asc",
                            "pkexec dnf config-manager setopt microsoft-edge.gpgcheck=1",
                         ], 
                         &[],
                         Box::new([])
                         )


}

 
