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
                                category                       : Category::Graphics,
                                desktop_env                    : &["all"],
                                display_type                   : &["all"],
                                title                          : "MyPaint",
                                button_install_label           : "Install",
                                button_remove_label            : "Remove",
                                button_install_running_label   : "Install Running",
                                button_remove_running_label    : "Remove Running",
                                button_waiting_label           : "Waiting...",
                                install_yes_or_no_header       : "Run Install MyPaint Task",
                                install_yes_or_no_label        : "Start Install MyPaint Task?",
                                remove_yes_or_no_header        : "Run Remove MyPaint Task",
                                remove_yes_or_no_label         : "Start Remove MyPaint Task?",
                                custom_cancel_warning_message  : None,
                                after_success_install_message  : None,
                                after_success_remove_message   : None,
                                subtitle                       : "Painting program for digital artists\n(Flatpak User Wide)",
                                icon_name                      : "mypaint.png",
                                licenses                       : &[&["License\nGPL-2.0+","https://www.gnu.org/licenses/old-licenses/gpl-2.0.html"]],
                                website                        : &["WebSite","https://flathub.org/apps/details/org.mypaint.MyPaint"],
    };
                            

   FlatpakInstaller::create(metadataplugin,
                         &["org.mypaint.MyPaint"],
                         &[],
                         &[],
                         Box::new([])
                         )


}

 
