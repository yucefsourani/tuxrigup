use crate::baseplugin::base::{PluginMetaData, Category, PluginType, CustomInstaller};


pub fn get_plugin() -> CustomInstaller {
    let metadataplugin: PluginMetaData = PluginMetaData {
        // If true, install/remove tasks will wait in a queue and run sequentially (FIFO).
        install_in_queue: false,
        
        // If true, displays a confirmation dialog before starting the install/remove process.
        yes_or_no: false, 
        
        // If true, this plugin will be completely ignored and not displayed in the UI.
        if_true_skip: false,
        
        // Plugin behavior type.
        // - PluginType::Website: install/remove commands are ignored, acts as a GTK LinkButton .
        // - PluginType::Oneshot: Ensure the check command always evaluates to true (e.g., &["true"]).
        type_: PluginType::EnableDisable,
        
        // Supported architectures (e.g., &["x86_64", "aarch64"] or &["all"]).
        arch: &["all"],
        
        // Supported Linux distributions. Matches 'ID_LIKE' or 'ID' from /etc/os-release.
        distro_name: &["all"],
        
        // Supported OS versions. Matches 'VERSION_ID' from /etc/os-release (e.g., &["40", "41"] or &["all"]).
        distro_version: &["all"],
        
        // The UI category where this plugin will appear.
        category: Category::Gnome,
        
        // Supported desktop environments. Matches $XDG_CURRENT_DESKTOP (e.g., &["GNOME", "KDE"] or &["all"]).
        desktop_env: &["gnome","gnome-xorg"],
        
        // Supported display servers. Matches $XDG_SESSION_TYPE (e.g., &["wayland", "x11"] or &["all"]).
        display_type: &["all"],
        
        // UI Labels and Text
        title: "Sound Overamplificatio",
        subtitle: "Enable/Disable Allow volume to exceed 100%",
        button_install_label: "Enable",
        button_remove_label: "Disable",
        button_install_running_label: "Enable",
        button_remove_running_label: "Disable",
        button_waiting_label: "",
        
        // Confirmation Dialogs
        install_yes_or_no_header: "",
        install_yes_or_no_label: "",
        remove_yes_or_no_header: "",
        remove_yes_or_no_label: "",
        
        // Optional Messages
        custom_cancel_warning_message: None, // Custom warning if the user cancels the task
        after_success_install_message: None, // e.g., Some("Install Done.") to show a Toast notification
        after_success_remove_message: None,  // e.g., Some("Remove Done.") to show a Toast notification
        
        // Metadata
        icon_name: "audio-volume-high.png", // Image file name in the 'images' folder
        licenses: &[],
        website: &[],
    };


    CustomInstaller::create(
        metadataplugin,
        
        // [1] Check Conditions:
        // Determines if the software is installed (returns 0) or not.
        // - Starts with '/' and NO trailing slash: checks if a FILE exists.
        // - Starts with '/' AND ends with '/': checks if a FOLDER exists.
        // - Otherwise: executes as a shell command and checks if the return code is 0.
        // If ALL conditions pass, the UI shows the "Remove" button.
        &["gsettings get org.gnome.desktop.sound allow-volume-above-100-percent | grep true"],
        
        // [2] Install Commands:
        // Commands to run for installation. Prefix with 'pkexec' for root privileges.
        // Available variables:
        // - $TARGET_FILE1  : Full path of the downloaded file (Location + file_name) from task 1.
        // - $TARGET_DIR1   : The directory where task 1 was downloaded.
        // - $USER_HOME_DIR : Current user's home directory (/home/username/).
        // - $USER_CONFIG_DIR, $USER_DOWNLOADS_DIR : standard user directories.
        &["gsettings set org.gnome.desktop.sound allow-volume-above-100-percent true"],
        
        // [3] Remove Commands:
        // Commands to run when the "Remove" button is clicked.
        &["gsettings set org.gnome.desktop.sound allow-volume-above-100-percent false"],
        
        // [4] Download Tasks:
        // List of files to download before running the install commands.
        // If empty, use: Box::new([])
        Box::new([])
    )
}
