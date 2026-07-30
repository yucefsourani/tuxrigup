use crate::baseplugin::base::{PluginMetaData,Category,PluginType,CustomInstaller};



pub fn get_plugin() -> CustomInstaller {
    let metadataplugin:PluginMetaData =  PluginMetaData {
                                install_in_queue               : false,
                                yes_or_no                      : true,
                                if_true_skip                   : false,
                                type_                          : PluginType::EnableDisable,
                                arch                           : &["all"],
                                distro_name                    : &["fedora"],
                                distro_version                 : &["all"],
                                category                       : Category::Multimedia,
                                desktop_env                    : &["all"],
                                display_type                   : &["all"],
                                title                          : "Xterm",
                                button_install_label           : "Oneshot",
                                button_remove_label            : "Oneshot",
                                button_install_running_label   : "Oneshot Running",
                                button_remove_running_label    : "Oneshot Running",
                                button_waiting_label           : "Waiting...",
                                install_yes_or_no_header       : "Waiting...",
                                install_yes_or_no_label        : "Waiting...",
                                remove_yes_or_no_header        : "Waiting...",
                                remove_yes_or_no_label         : "Waiting...",
                                custom_cancel_warning_message  : None,
                                after_success_install_message  : Some("<b>Oneshot</b> <i>Success</i>."),
                                after_success_remove_message   : Some("<b>Oneshot</b> <i>Success</i>."),
                                subtitle                       : "Fast, Private and Safe Web Browser",
                                icon_name                      : "brave-origin.png",
                                keywords                       : "xterm",
                                licenses                       : &[&["License\nMPL-2.0","https://www.mozilla.org/en-US/MPL/2.0/"]],
                                website                        : &["WebSite","https://www.mozilla.org/en-US/firefox/"],
    };
                            
   /* DnfInstaller::create(metadataplugin,
                         &["xterm"],
                         true,
                         &["sleep 10"],
                         &["echo hello22222222222222"]
                         )*/
   CustomInstaller::create(metadataplugin,
                         &["false"],
                         &["sleep 5"],
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
 
