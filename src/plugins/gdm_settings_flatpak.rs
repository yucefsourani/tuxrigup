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
                                category                       : Category::Gnome,
                                desktop_env                    : &["gnome"],
                                display_type                   : &["all"],
                                title                          : "GDM Settings",
                                button_install_label           : "Install",
                                button_remove_label            : "Remove",
                                button_install_running_label   : "Install Running",
                                button_remove_running_label    : "Remove Running",
                                button_waiting_label           : "Waiting...",
                                install_yes_or_no_header       : "Run Install GDM Settings Task",
                                install_yes_or_no_label        : "Start Install GDM Settings Task?",
                                remove_yes_or_no_header        : "Run Remove GDM Settings Task",
                                remove_yes_or_no_label         : "Start Remove GDM Settings Task?",
                                custom_cancel_warning_message  : None,
                                after_success_install_message  : None,
                                after_success_remove_message   : None,
                                subtitle                       : "Change GDM Settings; Apply theme and background, change cursor theme, icon theme and night light settings, among other things.\n(Flatpak User Wide)",
                                icon_name                      : "io.github.realmazharhussain.GdmSettings.svg",
                                licenses                       : &[&["License\nGPL V3.0","https://www.gnu.org/licenses/gpl-3.0.html"]],
                                website                        : &["WebSite","https://gdm-settings.github.io/"],
    };
                            

   FlatpakInstaller::create(metadataplugin,
                         &["io.github.realmazharhussain.GdmSettings"],
                         &[],
                         &[],
                         Box::new([])
                         )


}

 
