use crate::baseplugin::base::{PluginMetaData,Category,PluginType,CustomInstaller};
use crate::DISTRO_VERSION;



pub fn get_plugin() -> CustomInstaller {
    let metadataplugin: PluginMetaData = PluginMetaData {
        install_in_queue: true,
        yes_or_no: false,
        if_true_skip: false,
        type_: PluginType::Oneshot,
        arch: &["all"],
        distro_name: &["fedora"],
        distro_version: &["all"],
        category: Category::Multimedia,
        desktop_env: &["all"],
        display_type: &["all"],
        title: "Codecs",
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
        subtitle: "Multimedia coder/decoder + mesa freeworld drivers/vulkan",
        icon_name: "codecs.png",
        licenses: &[&["License\nUNKNOWN", ""]],
        website: &[],
    };

    let distro_version: &str = DISTRO_VERSION.get().unwrap();
    let rpmfusion_install_command = format!(
        "pkexec stdbuf -o1 dnf install --best -y --nogpgcheck --color=never \
        http://download1.rpmfusion.org/free/fedora/rpmfusion-free-release-{0}.noarch.rpm \
        http://download1.rpmfusion.org/nonfree/fedora/rpmfusion-nonfree-release-{0}.noarch.rpm",
        distro_version
    );
    
    let static_command: &'static str = Box::leak(rpmfusion_install_command.into_boxed_str());


    let commands_vec = vec![
        "pkexec rpm -v --nodeps -e ffmpeg-free        || true",
        "pkexec rpm -v --nodeps -e libavcodec-free    || true",
        "pkexec rpm -v --nodeps -e libavdevice-free   || true",
        "pkexec rpm -v --nodeps -e libavfilter-free   || true",
        "pkexec rpm -v --nodeps -e libavformat-free   || true",
        "pkexec rpm -v --nodeps -e libavutil-free     || true",
        "pkexec rpm -v --nodeps -e libswresample-free || true",
        "pkexec rpm -v --nodeps -e libpostproc-free   || true",
        "pkexec rpm -v --nodeps -e libswscale-free    || true",
        "pkexec rpm -v --nodeps -e mesa-va-drivers    || true",
        "pkexec rpm -v --nodeps -e mesa-vdpau-drivers || true",
        "pkexec rpm -v --nodeps -e mesa-vulkan-drivers.i686 || true",
        "pkexec rpm -v --nodeps -e mesa-vulkan-drivers  || true",
        static_command,
        "pkexec dnf config-manager enable  rpmfusion-free",
        "pkexec dnf config-manager enable  rpmfusion-free-updates",
        "pkexec dnf config-manager enable  rpmfusion-nonfree",
        "pkexec dnf config-manager enable  rpmfusion-nonfree-updates",
        "pkexec dnf config-manager enable  rpmfusion-nonfree-nvidia-driver",
        "pkexec dnf config-manager enable  rpmfusion-nonfree-steam",
        "pkexec dnf config-manager enable  fedora-cisco-openh264",
        "pkexec stdbuf -o1 dnf group install multimedia -y --best --color=never",
        "pkexec stdbuf -o1 dnf install gstreamer1-plugin-openh264 -y --best --color=never",
        "pkexec stdbuf -o1 dnf install mozilla-openh264 -y --best --color=never",
        "pkexec stdbuf -o1 dnf install ffmpeg -y --best --color=never",
        "pkexec stdbuf -o1 dnf install ffmpeg-libs -y --best --color=never",
        "pkexec stdbuf -o1 dnf install gstreamer1-plugins-bad-free-extras -y --best --color=never",
        "pkexec stdbuf -o1 dnf install mesa-va-drivers-freeworld intel-media-driver mesa-vulkan-drivers-freeworld.i686 mesa-vulkan-drivers-freeworld libva-intel-driver mesa-va-drivers-freeworld.i686 libva-nvidia-driver libva-nvidia-driver.i686  -y --best --color=never",
    ];

    let static_commands_array: &'static [&'static str] = Box::leak(commands_vec.into_boxed_slice());

    CustomInstaller::create(
        metadataplugin,
        &["false"],
        static_commands_array,
        static_commands_array,
        Box::new([])
    )
}

 
