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
                                category                       : Category::Utility,
                                desktop_env                    : &["all"],
                                display_type                   : &["all"],
                                title                          : "Kde Connect",
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
                                subtitle                       : "Files and links. Shared between devices (Fedora RPM)",
                                icon_name                      : "kde-connect.png",
                                licenses                       : &[&["License\nGPLv2+","https://www.gnu.org/licenses/old-licenses/gpl-2.0.en.html"]],
                                website                        : &["WebSite","https://kdeconnect.kde.org/"],
    };
   use crate::DESKTOP_TYPE;
   let current_desktop = DESKTOP_TYPE.get().unwrap().as_str();
   let commands_vec = {
        if current_desktop.contains("gnome") {
            vec!["kdeconnectd","kde-connect","kde-connect-libs","kde-connect-nautilus"]
        }else {
            vec!["kdeconnectd","kde-connect","kde-connect-libs"]
        }
    };
   let static_commands_array: &'static [&'static str] = Box::leak(commands_vec.into_boxed_slice());
   DnfInstaller::create(metadataplugin,
                         &static_commands_array,
                         false, // install and enable rpmfusion  first 
                         &[], 
                         &["pkexec firewall-cmd --zone=$(firewall-cmd --get-default-zone) --add-service=kdeconnect --permanent","pkexec firewall-cmd --reload"],
                         Box::new([])
                         )


}

 
