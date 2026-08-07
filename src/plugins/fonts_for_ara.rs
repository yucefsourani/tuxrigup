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
                                category                       : Category::System,
                                desktop_env                    : &["all"],
                                display_type                   : &["all"],
                                title                          : "Fonts For Arabic Language",
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
                                subtitle                       : "Fonts For Support Arabic Language",
                                icon_name                      : "",
                                licenses                       : &[&["License\nUNKNOWN",""]],
                                website                        : &["WebSite",""],
    };
                            

   DnfInstaller::create(metadataplugin,
                         &["KEEP_dejavu-sans-fonts", "KEEP_dejavu-sans-mono-fonts",
                          "kacst-art-fonts", "kacst-book-fonts",
                          "kacst-decorative-fonts",
                          "kacst-digital-fonts", "kacst-farsi-fonts",
                          "kacst-letter-fonts", "kacst-naskh-fonts", 
                          "kacst-office-fonts", "kacst-one-fonts", 
                          "kacst-pen-fonts", "kacst-poster-fonts", 
                          "kacst-qurn-fonts", "kacst-screen-fonts", 
                          "kacst-title-fonts", "kacst-titlel-fonts", 
                          "paktype-naqsh-fonts", 
                          "paktype-tehreer-fonts",
                          "sil-lateef-fonts", 
                          "google-noto-sans-arabic-fonts", 
                          "google-noto-naskh-arabic-fonts", 
                          "google-noto-naskh-arabic-ui-fonts", 
                          "google-noto-sans-old-south-arabian-fonts",
                          "amiri-quran-fonts",
                          "amiri-fonts",
                          "amiri-quran-colored-fonts"],
                         false, // install and enable rpmfusion  first 
                         &[], 
                         &[],
                         Box::new([])
                         )


}

 
