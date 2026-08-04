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
                                title                          : "Codeblocks",
                                button_install_label           : "Install",
                                button_remove_label            : "Remove",
                                button_install_running_label   : "Install Running",
                                button_remove_running_label    : "Remove Running",
                                button_waiting_label           : "Waiting...",
                                install_yes_or_no_header       : "Run Install Codeblocks Task",
                                install_yes_or_no_label        : "Start Install Codeblocks Task?",
                                remove_yes_or_no_header        : "Run Remove Codeblocks Task",
                                remove_yes_or_no_label         : "Start Remove Codeblocks Task?",
                                custom_cancel_warning_message  : None,
                                after_success_install_message  : None,
                                after_success_remove_message   : None,
                                subtitle                       : "An open source, cross platform, free C++ IDE",
                                icon_name                      : "codeblocks.png",
                                licenses                       : &[&["License\nGPL v3.0+","https://www.gnu.org/licenses/gpl-3.0.html"]],
                                website                        : &["WebSite","http://www.codeblocks.org/"],
    };
                            

   DnfInstaller::create(metadataplugin,
                         &["codeblocks","KEEP_gcc","KEEP_gcc-c++","KEEP_xterm"], // Start Package with KEEP_ to save from remove
                         false, // install and enable rpmfusion  first 
                         &[], 
                         &[],
                         Box::new([])
                         )


}

 
