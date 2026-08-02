use crate::baseplugin::base::{PluginMetaData, Category, PluginType, CustomInstaller, DownloadTask};
use std::sync::{Arc, Mutex};

pub fn get_plugin() -> CustomInstaller {
    let metadataplugin: PluginMetaData = PluginMetaData {
        // If true, install/remove tasks will wait in a queue and run sequentially (FIFO).
        install_in_queue: false,
        
        // If true, displays a confirmation dialog before starting the install/remove process.
        yes_or_no: true, 
        
        // If true, this plugin will be completely ignored and not displayed in the UI.
        if_true_skip: false,
        
        // Plugin behavior type.
        // - PluginType::Website: install/remove commands are ignored, acts as a GTK LinkButton .
        // - PluginType::Oneshot: Ensure the check command always evaluates to true (e.g., &["true"]).
        type_: PluginType::Installer,
        
        // Supported architectures (e.g., &["x86_64", "aarch64"] or &["all"]).
        arch: &["x86_64"],
        
        // Supported Linux distributions. Matches 'ID_LIKE' or 'ID' from /etc/os-release.
        distro_name: &["all"],
        
        // Supported OS versions. Matches 'VERSION_ID' from /etc/os-release (e.g., &["40", "41"] or &["all"]).
        distro_version: &["all"],
        
        // The UI category where this plugin will appear.
        category: Category::Internet,
        
        // Supported desktop environments. Matches $XDG_CURRENT_DESKTOP (e.g., &["GNOME", "KDE"] or &["all"]).
        desktop_env: &["all"],
        
        // Supported display servers. Matches $XDG_SESSION_TYPE (e.g., &["wayland", "x11"] or &["all"]).
        display_type: &["all"],
        
        // UI Labels and Text
        title: "Xdman",
        subtitle: "Xtreme Download Manager",
        button_install_label: "Install",
        button_remove_label: "Remove",
        button_install_running_label: "Install Running...",
        button_remove_running_label: "Remove Running...",
        button_waiting_label: "Waiting...",
        
        // Confirmation Dialogs
        install_yes_or_no_header: "Run Install Xdman Task",
        install_yes_or_no_label: "Start Install Xdman Task?",
        remove_yes_or_no_header: "Run Remove Xdman Task",
        remove_yes_or_no_label: "Start Remove Xdman Task?",
        
        // Optional Messages
        custom_cancel_warning_message: None, // Custom warning if the user cancels the task
        after_success_install_message: None, // e.g., Some("Install Done.") to show a Toast notification
        after_success_remove_message: None,  // e.g., Some("Remove Done.") to show a Toast notification
        
        // Metadata
        icon_name: "xdman.png", // Image file name in the 'images' folder
        licenses: &[&["License\nGPL V2.0", "https://www.gnu.org/licenses/gpl-2.0.html"]],
        website: &["WebSite", "https://github.com/subhra74/xdm"],
    };

   /*use crate::HOMEDIR;

    use crate::CONFIGDIR;

    use crate::DOWNLOADSDIR;

    use crate::utils::fs::join_paths;

    let home = HOMEDIR.get().unwrap();

    let user_pictures_location = join_paths(home,"Pictures");*/ 
    let download_task1 = DownloadTask {
        link: "https://github.com/subhra74/xdm/releases/download/7.2.11/xdm-setup-7.2.11.tar.xz",
        
        // Download location. 
        // If None, a random temporary folder in /tmp will be created.
        // You can also use paths like Some("/tmp") or Some(DOWNLOADSDIR.get().unwrap()) or Some(user_pictures_location)
        dir_download_location: None, 
        
        // The file name to save the download as.
        file_name: "xdm-setup-7.2.11.tar.xz", 
    };

    CustomInstaller::create(
        metadataplugin,
        
        // [1] Check Conditions:
        // Determines if the software is installed (returns 0) or not.
        // - Starts with '/' and NO trailing slash: checks if a FILE exists.
        // - Starts with '/' AND ends with '/': checks if a FOLDER exists.
        // - Otherwise: executes as a shell command and checks if the return code is 0.
        // If ALL conditions pass, the UI shows the "Remove" button.
        &["/opt/xdman/uninstall.sh"],
        
        // [2] Install Commands:
        // Commands to run for installation. Prefix with 'pkexec' for root privileges.
        // Available variables:
        // - $TARGET_FILE1  : Full path of the downloaded file (Location + file_name) from task 1.
        // - $TARGET_DIR1   : The directory where task 1 was downloaded.
        // - $USER_HOME_DIR : Current user's home directory (/home/username/).
        // - $USER_CONFIG_DIR, $USER_DOWNLOADS_DIR : standard user directories.
        &[
            "tar -xJf \"$TARGET_FILE1\" -C \"$TARGET_DIR1\"",
            "chmod 755 \"$TARGET_DIR1\"/install.sh",
            "pkexec \"$TARGET_DIR1\"/install.sh"
        ],
        
        // [3] Remove Commands:
        // Commands to run when the "Remove" button is clicked.
        &[
            "pkexec chmod 755 /opt/xdman/uninstall.sh", 
            "pkexec /opt/xdman/uninstall.sh"
        ],
        
        // [4] Download Tasks:
        // List of files to download before running the install commands.
        // If empty, use: Box::new([])
        Box::new([Arc::new(Mutex::new(Some(download_task1)))])
    )
}
