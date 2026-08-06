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
                                title                          : "Gimp <span color='red'>Beta</span>",
                                button_install_label           : "Install",
                                button_remove_label            : "Remove",
                                button_install_running_label   : "Install Running",
                                button_remove_running_label    : "Remove Running",
                                button_waiting_label           : "Waiting...",
                                install_yes_or_no_header       : "Run Install Gimp Beta Task",
                                install_yes_or_no_label        : "Start Install Gimp Beta Task?",
                                remove_yes_or_no_header        : "Run Remove Gimp Beta Task",
                                remove_yes_or_no_label         : "Start Remove Gimp Beta Task?",
                                custom_cancel_warning_message  : None,
                                after_success_install_message  : None,
                                after_success_remove_message   : None,
                                subtitle                       : "GIMP is an acronym for GNU Image Manipulation Program\n(Flatpak User wide)",
                                icon_name                      : "gimp.png",
                                licenses                       : &[&["License\nGPL v3.0+","https://www.gnu.org/licenses/gpl-3.0.html"],&["License\nLGPL-3.0+","https://www.gnu.org/licenses/lgpl-3.0.en.html"]],
                                website                        : &["WebSite","https://www.gimp.org/"],
    };
                            

   FlatpakInstaller::create(metadataplugin,
                         &["org.gimp.GIMP//beta"],
                         &[
                            "pkexec flatpak  remove org.gimp.GIMP -y || true",
                            "flatpak  remove org.gimp.GIMP -y || true",
                         ],
                         &[],
                         Box::new([])
                         )


}

 
