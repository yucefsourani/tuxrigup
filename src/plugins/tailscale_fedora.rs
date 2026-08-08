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
                                category                       : Category::Utility,
                                desktop_env                    : &["all"],
                                display_type                   : &["all"],
                                title                          : "Tailscale",
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
                                subtitle                       : "VPN cross platform way to use WireGuard + oauth2 + 2FA/SSO",
                                icon_name                      : "tailscale.png",
                                licenses                       : &[&["License\nUNKNOWN","https://github.com/tailscale/tailscale"]],
                                website                        : &["WebSite","https://tailscale.com"],
    };
                            

   DnfInstaller::create(metadataplugin,
                         &["tailscale"],
                         true, // install and enable rpmfusion  first 
                         // command run before install
                         &[
                            "pkexec echo -e '[tailscale-stable]\nname=Tailscale stable\nbaseurl=https://pkgs.tailscale.com/stable/fedora/$basearch\nenabled=1\ntype=rpm\nrepo_gpgcheck=1\ngpgcheck=1\ngpgkey=https://pkgs.tailscale.com/stable/fedora/repo.gpg' > /etc/yum.repos.d/tailscale.repo",
                            "pkexec rpm --import https://pkgs.tailscale.com/stable/fedora/repo.gpg"
                         ], 
                         &[],
                         Box::new([])
                         )


}

 
