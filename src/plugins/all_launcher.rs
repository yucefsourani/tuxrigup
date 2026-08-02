use crate::baseplugin;

pub fn get_all_launcher() -> &'static  [baseplugin::base::LauncherFileInfo] {
    let all_ = &[
            baseplugin::base::LauncherFileInfo{
                                                laucher_file_name   : "vlc.desktop",
                                                type_               : baseplugin::base::PluginType::Launcher,
                                                category            : baseplugin::base::Category::Launcher,
                                                arch                : &["all"],
                                                distro_name         : &["all"],
                                                distro_version      : &["all"],
                                                desktop_env         : &["all"],
                                                display_type        : &["all"],
                                                icon_name           : "UNKNOW", //if icon_name not in images folders will get icon name from  vlc.desktop and load from system
                                                custom_button_label : None,// Option<&'stati>,
                                                },
            baseplugin::base::LauncherFileInfo{
                                                laucher_file_name   : "com.github.yucefsourani.albasheer-electronic-quran-browser.desktop",
                                                type_               : baseplugin::base::PluginType::Launcher,
                                                category            : baseplugin::base::Category::Launcher,
                                                arch                : &["all"],
                                                distro_name         : &["all"],
                                                distro_version      : &["all"],
                                                desktop_env         : &["all"],
                                                display_type        : &["all"],
                                                icon_name           : "UNKNOW",
                                                custom_button_label : None,// Option<&'static str>,
                                                },
            
            ];
    all_
}
