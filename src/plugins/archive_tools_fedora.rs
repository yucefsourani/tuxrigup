use crate::baseplugin::base::{PluginMetaData,Category,PluginType,CustomInstaller};




pub fn get_plugin() -> CustomInstaller {
    let metadataplugin: PluginMetaData = PluginMetaData {
        install_in_queue: true,
        yes_or_no: false,
        if_true_skip: false,
        type_: PluginType::Oneshot,
        arch: &["all"],
        distro_name: &["fedora"],
        distro_version: &["all"],
        category: Category::System,
        desktop_env: &["all"],
        display_type: &["all"],
        title: "Compression Utility",
        button_install_label: "Oneshot",
        button_remove_label: "Oneshot",
        button_install_running_label: "Install Running",
        button_remove_running_label: "Install Running",
        button_waiting_label: "Waiting...",
        install_yes_or_no_header: "",
        install_yes_or_no_label: "",
        remove_yes_or_no_header: "",
        remove_yes_or_no_label: "",
        custom_cancel_warning_message: None,
        after_success_install_message: None,
        after_success_remove_message: None,
        subtitle: "Utilities for listing/extracting/archiving files",
        icon_name: "tools_settings_tool_preferences-512.png",
        licenses: &[&["License\nUNKNOWN", ""]],
        website: &[],
    };

    CustomInstaller::create(
        metadataplugin,
        &["false"],
        &["pkexec stdbuf -o1 dnf install zip p7zip gzip cpio unar p7zip-plugins 7zip-standalone 7zip-standalone-all 7zip-reduced -y --best --color=never"],
        &["pkexec stdbuf -o1 dnf install zip p7zip gzip cpio unar p7zip-plugins -y --best --color=never"],
        Box::new([])
    )
}

 
