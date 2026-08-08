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
                                category                       : Category::Other,
                                desktop_env                    : &["all"],
                                display_type                   : &["all"],
                                title                          : "VirtualBox",
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
                                after_success_install_message  : Some("Install VirtualBox Sucess\n\n<span foreground=\"red\">Update Your System And Reboot\n\nAnd Disable Secure Boot</span>"),
                                after_success_remove_message   : None,
                                subtitle                       : "Powerful x86 and AMD64/Intel64 virtualization product",
                                icon_name                      : "VBox.png",
                                licenses                       : &[&["License\nUNKNOWN","https://www.virtualbox.org/"]],
                                website                        : &["WebSite","https://www.virtualbox.org/"],
    };
                            

   DnfInstaller::create(metadataplugin,
                         &["VirtualBox", "akmod-VirtualBox", "VirtualBox-server", "VirtualBox-kmodsrc"],
                         true, // install and enable rpmfusion  first 
                         &["pkexec dnf remove VirtualBox-5.0 VirtualBox-5.1 VirtualBox-5.2 VirtualBox-5.3 VirtualBox-6.0 VirtualBox-6.1 VirtualBox-6.2 VirtualBox-6.3 VirtualBox-6.4 VirtualBox-6.5 VirtualBox-6.6 VirtualBox-6.7 VirtualBox-6.8 VirtualBox-6.9 --setopt=clean_requirements_on_remove=False -y || true"], 
                         &[],
                         Box::new([])
                         )


}

 
