use crate::baseplugin::base::{PluginMetaData,Category,PluginType,FlatpakInstaller};




pub fn get_plugin() -> FlatpakInstaller {
    let metadataplugin:PluginMetaData =  PluginMetaData {
                                install_in_queue               : false,
                                yes_or_no                      : true,
                                if_true_skip                   : false,
                                type_                          : PluginType::Installer,
                                arch                           : &["all"],
                                distro_name                    : &["all"],
                                distro_version                 : &["all"],
                                category                       : Category::Internet,
                                desktop_env                    : &["all"],
                                display_type                   : &["all"],
                                title                          : "ALBahseer",
                                button_install_label           : "Install",
                                button_remove_label            : "Remove",
                                button_install_running_label   : "Install Running",
                                button_remove_running_label    : "Remove Running",
                                button_waiting_label           : "Waiting...",
                                install_yes_or_no_header       : "Run Install ALBahseer Task",
                                install_yes_or_no_label        : "Start Install ALBahseer Task?",
                                remove_yes_or_no_header        : "Run Remove ALBahseer Task",
                                remove_yes_or_no_label         : "Start Remove ALBahseer Task?",
                                custom_cancel_warning_message  : None,
                                after_success_install_message  : None,
                                after_success_remove_message   : None,
                                subtitle                       : "ALBahseer",
                                icon_name                      : "brave-origin.png",
                                keywords                       : "downloder xdman",
                                licenses                       : &[&["License\nGPL V2.0","https://www.gnu.org/licenses/gpl-2.0.html"]],
                                website                        : &["WebSite","https://github.com/subhra74/xdm"],
    };
                            

   FlatpakInstaller::create(metadataplugin,
                         &["com.github.yucefsourani.albasheer-electronic-quran-browser"],
                         &[],
                         &[],
                         Box::new([])
                         )


}
/*

    
    link plugin
    
    launcher plugin 
    
    about 
    
    settings
    
    

 
 */
 
