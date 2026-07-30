use crate::baseplugin::base::{PluginMetaData,Category,PluginType,CustomInstaller,DownloadTask};
use std::sync::{Arc,Mutex};



pub fn get_plugin() -> CustomInstaller {
    let metadataplugin:PluginMetaData =  PluginMetaData {
                                install_in_queue               : false,
                                yes_or_no                      : true,
                                if_true_skip                   : false,
                                type_                          : PluginType::Website,
                                arch                           : &["all"],
                                distro_name                    : &["fedora"],
                                distro_version                 : &["all"],
                                category                       : Category::Internet,
                                desktop_env                    : &["all"],
                                display_type                   : &["all"],
                                title                          : "Xterm",
                                button_install_label           : "Install",
                                button_remove_label            : "Remove",
                                button_install_running_label   : "Install Running",
                                button_remove_running_label    : "Remove Running",
                                button_waiting_label           : "Waiting...",
                                yes_or_no_header               : "Waiting...",
                                yes_or_no_label                : "Waiting...",
                                custom_cancel_warning_message  : None,
                                after_success_install_message  : Some("<b>Install</b> <i>Success</i>."),
                                after_success_remove_message   : Some("<b>Remove</b> <i>Success</i>."),
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
    let download_task1 = DownloadTask {
                                        link: "https://github.com/Hamza5/Learn-to-program-with-C_AR/releases/download/v1.0.1/Learn_C_Language_v1.0.1.pdf?v=123",
                                        dir_download_location: Some("/tmp".to_string()),
                                        file_name: "Learn_C_Language_v1.0.1.pdf",
                                    };
    let download_task2 = DownloadTask{
                                        link: "https://github.com/Hamza5/Learn-to-program-with-C_AR/releases/download/v1.0.1/Learn_C_Language_v1.0.1.pdf?v=123",
                                        dir_download_location: Some("/tmp".to_string()),
                                        file_name: "Learn_C_Language_v2.0.1.pdf",
                                    };
   CustomInstaller::create(metadataplugin,
                         &["l"],
                         &["sleep 5","sleep 5"],
                         &["sleep 5","sleep 5"],
                         Box::new([Arc::new(Mutex::new(Some(download_task1))),
                                   Arc::new(Mutex::new(Some(download_task2)))
                                   ])
                         )


}
/*
    
    fix launcher plugin icons 
    
    about 
    
    settings
    
    

 
 */
 
