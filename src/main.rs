use adw::prelude::*;
use std::sync::{Arc, Mutex,OnceLock};
use std::sync::atomic::{AtomicI32, Ordering,AtomicBool};
use std::cell::{RefCell,Cell};
use std::rc::Rc;
use std::collections::HashMap;
use gio::prelude::AppInfoExt;
use adw::prelude::SettingsExtManual;
use std::process::Command;
use gio::prelude::IconExt;

mod plugins;
mod baseplugin;
mod utils;
//mod widgets;

pub static DISTRO_NAME: OnceLock<String> = OnceLock::new();
pub static DISTRO_VERSION: OnceLock<String> = OnceLock::new();
pub static DESKTOP_TYPE: OnceLock<String> = OnceLock::new();
pub static DISPLAY_TYPE: OnceLock<String> = OnceLock::new();
pub static CONFIGDIR: OnceLock<String> = OnceLock::new();
pub static HOMEDIR: OnceLock<String> = OnceLock::new();
pub static DOWNLOADSDIR: OnceLock<String> = OnceLock::new();
pub const  CURRENT_ARCH: &str = std::env::consts::ARCH;
pub const  VERSION: &str = "1.0";

pub const CSS: &str  = "
.wait-action-button,
.running-destructive-action-button {
  min-width: 0;
  min-height: 0;
  padding: 2px 6px; /* مسافة داخلية صغيرة جداً للحفاظ على شكل الزر */
  margin: 0; 
  font-size: 11px;  /* أزل التعليق إذا أردت تصغير خط النص أيضاً */
}

/* الألوان الخاصة بكل زر */
.wait-action-button {
  background-color: @warning_bg_color;
  color: @accent_fg_color;
}

.running-destructive-action-button {
  background-color: @success_bg_color;
  color: @accent_fg_color;
}

/* 
  الخصائص المشتركة للأزرار الثلاثة
  لضمان أن تكون صغيرة الحجم وموحدة المظهر
*/
.btn-state-install,
.btn-state-remove,
.btn-state-waiting {
  /*min-width: 0;
  min-height: 0;*/
  padding: 4px 10px; /* مساحة صغيرة تضمن وضوح الكلمة */
  margin: 0;
  font-weight: bold; /* لجعل النص أكثر وضوحاً مع الألوان */
}

/* 1. حالة التثبيت (أزرق) */
.btn-state-install {
  background-color: @accent_bg_color;
  color: @accent_fg_color;
}

/* 2. حالة الإزالة (أحمر) */
.btn-state-remove {
  background-color: @destructive_bg_color;
  color: white;
}

/* 3. حالة الانتظار (أصفر/برتقالي) */
.btn-state-waiting {
  background-color: @warning_bg_color;
  /* نستخدم لون نص داكن هنا لأن اللون الأبيض غير مقروء جيداً فوق الخلفية الصفراء */
  color: rgba(0, 0, 0, 0.8); 
}
progressbar > trough {
    background-color: #9FC0D9;
}
";

fn init_distro_info() {
    // 2. جلب البيانات وتخزينها مرة واحدة فقط عند بداية تشغيل البرنامج
    let (name, version) = utils::runtimeconfig::get_distro_info(true).unwrap();
    let _ = DISTRO_NAME.set(name);
    let _ = DISTRO_VERSION.set(version);
    let _ = DESKTOP_TYPE.get_or_init(|| {
        let raw_mode = std::env::var("XDG_SESSION_DESKTOP")
            .unwrap_or_else(|_| String::from("UNKNOWN"));
        raw_mode.to_lowercase()
    });
    let _ = DISPLAY_TYPE.get_or_init(|| {
        let raw_mode = std::env::var("XDG_SESSION_TYPE")
            .unwrap_or_else(|_| String::from("UNKNOWN"));
        raw_mode.to_lowercase()
    });
    let _ = HOMEDIR.get_or_init(|| {
        let home_dir = glib::home_dir();
        home_dir.to_string_lossy().to_string()
        
    });
    let _ = CONFIGDIR.get_or_init(|| {
        let config_dir = glib::user_config_dir();
        config_dir.to_string_lossy().to_string()

    });
    let _ = DOWNLOADSDIR.get_or_init(|| {
        //let downloads_dir = glib::user_special_dir(glib::UserDirectory::Downloads);
        let mut dir = glib::home_dir();
        dir.push("Downloads");
        dir.to_string_lossy().to_string()

    });
    
}

pub fn get_all_plugins() -> Vec<Box<dyn baseplugin::base::PluginTools>> {
    vec![
        Box::new(plugins::xdm::get_plugin()),
        Box::new(plugins::albasheer::get_plugin()),
        Box::new(plugins::arduinoide::get_plugin()),
        Box::new(plugins::arduinoidev2::get_plugin()),
        Box::new(plugins::ciano::get_plugin()),
        Box::new(plugins::fritzing::get_plugin()),
        Box::new(plugins::gitkraken::get_plugin()),
        Box::new(plugins::handbrake::get_plugin()),
        Box::new(plugins::marker::get_plugin()),
        Box::new(plugins::mypaint::get_plugin()),
        Box::new(plugins::pulseeffects::get_plugin()),
        Box::new(plugins::vidcutter::get_plugin()),
        Box::new(plugins::android_studio::get_plugin()),
        Box::new(plugins::anydesk::get_plugin()),
        Box::new(plugins::audacity_freeworld_fedora::get_plugin()),
        Box::new(plugins::blender_fedora::get_plugin()),
        Box::new(plugins::blender_flatpak::get_plugin()),
        Box::new(plugins::brave_fedora::get_plugin()),
        Box::new(plugins::brave_origin_fedora::get_plugin()),
        Box::new(plugins::broadcom_fedora::get_plugin()),
        Box::new(plugins::chromium_fedora::get_plugin()),
        Box::new(plugins::codeblocks_fedora::get_plugin()),
        Box::new(plugins::codecs_fedora::get_plugin()),
        Box::new(plugins::archive_tools_fedora::get_plugin()),
        Box::new(plugins::darktable_fedora::get_plugin()),
        Box::new(plugins::darktable_flatpak::get_plugin()),
        Box::new(plugins::dnf_fastestmirror::get_plugin()),
        Box::new(plugins::dnf_keepcache::get_plugin()),
        Box::new(plugins::kde_ffmpegthumbs_fedora::get_plugin()),
        Box::new(plugins::xfce_ffmpegthumbnailer_fedora::get_plugin()),
        Box::new(plugins::firefox_fedora::get_plugin()),
        Box::new(plugins::firefox_flatpak::get_plugin()),
        Box::new(plugins::sitra_flatpak::get_plugin()),
        Box::new(plugins::gradia_flatpak::get_plugin()),
        Box::new(plugins::flatseal_flatpak::get_plugin()),
        Box::new(plugins::gnome_extension_m_flatpak::get_plugin()),
        Box::new(plugins::bazaar_flatpak::get_plugin()),
        Box::new(plugins::gdm_settings_flatpak::get_plugin()),
        Box::new(plugins::refine_flatpak::get_plugin()),
        ]
}


fn main() {
    init_distro_info();
// 1. إنشاء بيئة Tokio لتعمل في الخلفية لمعالجة الشبكات
    let tokio_rt = tokio::runtime::Runtime::new().expect("فشل في إنشاء Tokio Runtime");
    
    // 2. إدخال المسار الحالي في سياق Tokio
    // المتغير _guard مهم جداً، بقاؤه حياً يعني بقاء سياق Tokio متاحاً طوال فترة عمل التطبيق
    let _guard = tokio_rt.enter();
    let app = adw::Application::builder().application_id("com.github.yucefsourani.tuxrigup").build();

    app.connect_activate(|app| {
        utils::gui::load_custom_css(CSS);

        let all_plugin = get_all_plugins();
        let mainwindow = adw::ApplicationWindow::builder().application(app).title("TuxRigUp").build();

        let settings  =  adw::gio::Settings::with_path("com.github.yucefsourani.tuxrigup","/com/github/yucefsourani/tuxrigupe/");
        settings.bind("width",&mainwindow,"default-width").build();
        settings.bind("height",&mainwindow,"default-height").build();
        settings.bind("is-maximized",&mainwindow,"maximized").build();
        settings.bind("is-fullscreen",&mainwindow,"fullscreened").build();
        let toastoverlay = adw::ToastOverlay::new();
        mainwindow.set_content(Some(&toastoverlay));

        //let mainvbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
        let toolbarview = adw::ToolbarView::new();
        toastoverlay.set_child(Some(&toolbarview));
        
        let headerbar = gtk::HeaderBar::new();
        toolbarview.add_top_bar(&headerbar);


        let about_dialog = adw::AboutDialog::new();
        about_dialog.set_application_icon("com.github.yucefsourani.tuxrigup");
        about_dialog.set_application_name("TuxRigUp");
        about_dialog.set_copyright("© 2026 Yucef");
        about_dialog.set_developer_name("Yucef Sourani");
        about_dialog.set_license_type(gtk::License::Gpl30);
        about_dialog.set_version(VERSION);
        about_dialog.set_website("https://github.com/yucefsourani/tuxrigup");
        about_dialog.set_support_url("https://github.com/yucefsourani/tuxrigup");
        about_dialog.set_developers(
                                     &["yucef mouhammad nazih  sourani"]
                                    );
        let top_about_button = gtk::Button::from_icon_name("help-about-symbolic");
        
        let c1_about_dialog = about_dialog.clone();
        let c1_main_window = mainwindow.clone();
        top_about_button.connect_clicked(move|_button| {
            c1_about_dialog.present(Some(&c1_main_window));
            });
        headerbar.pack_end(&top_about_button);
        
        let view_switcher_bar   = adw::ViewSwitcherBar::new();
        let viewswitcher        = adw::ViewSwitcher::new();
        viewswitcher.set_policy(adw::ViewSwitcherPolicy::Wide);
        toolbarview.add_bottom_bar(&view_switcher_bar);
        let mainstack = adw::ViewStack::new();
        toolbarview.set_content(Some(&mainstack));
        
        viewswitcher.set_stack(Some(&mainstack));
        view_switcher_bar.set_stack(Some(&mainstack));
        headerbar.set_title_widget(Some(&viewswitcher));

        let breakpoint = adw::Breakpoint::new(
            adw::BreakpointCondition::parse("max-width: 550sp").unwrap()
        );
        
        breakpoint.add_setter(&view_switcher_bar, "reveal", Some(&true.to_value()));
        breakpoint.add_setter(&headerbar, "title-widget", Some(&gtk::Widget::NONE.to_value()));
       
 
        
        let mut category_map: HashMap<&str,gtk::ListBox> = HashMap::new();
        let overlaysplitview_collapsed_button = gtk::ToggleButton::builder().icon_name("sidebar-show-symbolic").build();
        let c_overlaysplitview_collapsed_button = overlaysplitview_collapsed_button.clone();
        mainstack.connect_visible_child_name_notify(move |stack| {
            if let Some(visible_child_name) = stack.visible_child_name() {
                if visible_child_name == "mhbox" {
                    c_overlaysplitview_collapsed_button.set_visible(true);
                    return;
                }else{
                    c_overlaysplitview_collapsed_button.set_visible(false);
                    return;
                }
            }
        });
        headerbar.pack_start(&overlaysplitview_collapsed_button);
        let overlaysplitview       = adw::OverlaySplitView::new();
        breakpoint.add_setter(&overlaysplitview, "collapsed", Some(&true.to_value()));

        overlaysplitview.set_max_sidebar_width(100.0);
        overlaysplitview
            .bind_property("show-sidebar", &overlaysplitview_collapsed_button, "active")
            .bidirectional()
            .sync_create()
            .build();
        let main_page_stack        = adw::ViewStack::new();
        let side_toolbarview       = adw::ToolbarView::new();
        let side_searchbar         = gtk::SearchBar::new();
        let side_searchentry       = gtk::SearchEntry::new();
        side_searchbar.set_key_capture_widget(Some(&overlaysplitview));
        side_searchbar.set_child(Some(&side_searchentry));
        side_searchbar.connect_entry(&side_searchentry);
        side_searchentry.set_placeholder_text(Some("Search..."));
        side_toolbarview.add_top_bar(&side_searchbar);
        
        main_page_stack.set_hexpand(true);
        main_page_stack.set_vexpand(true);
        let main_page_viewswitcher = adw::ViewSwitcherSidebar::new();
        main_page_viewswitcher.set_mode(adw::SidebarMode::Sidebar);
        main_page_viewswitcher.set_halign(gtk::Align::Start);
        main_page_viewswitcher.set_size_request(140,-1);
        main_page_viewswitcher.set_stack(Some(&main_page_stack));
        side_toolbarview.set_content(Some(&main_page_viewswitcher));
        overlaysplitview.set_sidebar(Some(&side_toolbarview));
        overlaysplitview.set_content(Some(&main_page_stack));
        
        
        mainwindow.add_breakpoint(breakpoint);
        for category in baseplugin::base::Category::get_str_list_catagory() {
            let vbox          = gtk::Box::new(gtk::Orientation::Vertical, 0);
            vbox.set_hexpand(true);
            vbox.set_vexpand(true);
            let clamp         = adw::Clamp::new();
            clamp.set_margin_top(10);
            clamp.set_margin_bottom(10);
            clamp.set_margin_start(5);
            clamp.set_margin_end(5);
            clamp.set_vexpand(true);
            clamp.set_maximum_size(600);
            let clamp_sw      = gtk::ScrolledWindow::new();
            let clamp_listbox = gtk::ListBox::new();
            let empty_status = adw::StatusPage::builder()
                                                .icon_name("system-search-symbolic") 
                                                .title("No Results Found")
                                                .vexpand(true)
                                                .description("We couldn't find what you're looking for in this section.")
                                                .build();
            clamp_listbox.set_placeholder(Some(&empty_status));
            clamp_listbox.set_css_classes(&["boxed-list"]);
            clamp.set_child(Some(&clamp_sw));
            clamp_sw.set_child(Some(&clamp_listbox));
            vbox.append(&clamp);
            let page  = main_page_stack.add_titled(&vbox,Some(category),category);
            //page.set_icon_name(Some(baseplugin::base::Category::get_catagory_icon_name(category)));

            
            let page_clone  = page.clone();
            let clamp_listbox_clone = clamp_listbox.clone();
            let main_page_stack_clone = main_page_stack.clone();
           side_searchentry.connect_search_changed(move |entry| {
                let text_to_search = entry.text().trim().to_lowercase();
                let mut has_results = false;
                
                if text_to_search.len() < 3 {
                    let mut child = clamp_listbox_clone.first_child();
                    while let Some(widget) = child {
                        if let Some(row) = widget.downcast_ref::<adw::ExpanderRow>() {
                            row.set_visible(true);
                        }
                        child = widget.next_sibling();
                    }
                    
                    page_clone.set_visible(true);
                    return; 
                }
                let mut child = clamp_listbox_clone.first_child();
                while let Some(widget) = child {
                    if let Some(row) = widget.downcast_ref::<adw::ExpanderRow>() {
                        
                        let title = row.title().to_lowercase();
                        let subtitle = row.subtitle().to_lowercase();
                        
                        let is_match = title.contains(&text_to_search) || subtitle.contains(&text_to_search);

                        row.set_visible(is_match);
                        
                        if is_match {
                            has_results = true;
                        }
                    }
                    child = widget.next_sibling();
                }
                page_clone.set_visible(has_results);
                if has_results {
                    let page_name = page_clone.name().unwrap();
                    if page_name !=  main_page_stack_clone.visible_child_name().unwrap(){
                        main_page_stack_clone.set_visible_child_name(&page_name);
                    }
                }
            });
            category_map.insert(category,clamp_listbox);
        }
        
        
        settings.bind("navigation-sidebar-visible-stack-child", &main_page_stack, "visible-child-name").build();
        let output_box_page  = gtk::Box::new(gtk::Orientation::Vertical, 0);
        //let about_box_page   = gtk::Box::new(gtk::Orientation::Vertical, 0);
        let _ = mainstack.add_titled_with_icon(&overlaysplitview,Some("mhbox"),"Main","application-x-executable-symbolic");
        let _ = mainstack.add_titled_with_icon(&output_box_page,Some("output"),"Output","utilities-terminal-symbolic");
        //let _ = mainstack.add_titled_with_icon(&about_box_page,Some("about"),"About","help-about-symbolic");
        settings.bind("visible-stack-child", &mainstack, "visible-child-name").build();


            

        let text_view = gtk::TextView::new();
        text_view.set_margin_top(10);
        text_view.set_margin_bottom(10);
        text_view.set_margin_start(5);
        text_view.set_margin_end(5);
        text_view.set_editable(false);
        text_view.set_wrap_mode(gtk::WrapMode::Word);
        let text_view_scrolled_window = gtk::ScrolledWindow::new();
        text_view_scrolled_window.set_hscrollbar_policy(gtk::PolicyType::Automatic);
        text_view_scrolled_window.set_vscrollbar_policy(gtk::PolicyType::Automatic);
        text_view_scrolled_window.set_vexpand(true);
        text_view_scrolled_window.set_hexpand(true);
        text_view_scrolled_window.set_child(Some(&text_view));
        output_box_page.append(&text_view_scrolled_window);
    
        

        
        // التعديل الحاسم: بناء المصفوفة بـ Rc + RefCell بدلاً من Arc + Mutex
        let lazy_fn_vec: Rc<RefCell<Vec<Box<utils::gui::Task>>>> = Rc::new(RefCell::new(Vec::new()));
        let click_handler_count             = Arc::new(AtomicI32::new(0));
        let c_click_handler_count           = Arc::clone(&click_handler_count);
        let non_queue_task_running_state    = Arc::new(AtomicBool::new(false));
        let c1_non_queue_task_running_state = Arc::clone(&non_queue_task_running_state);
        let confirmed_window_exit           = Rc::new(Cell::new(false));
        mainwindow.connect_close_request(move |window| {
            if c_click_handler_count.load(Ordering::SeqCst) <= 0  && c1_non_queue_task_running_state.load(Ordering::SeqCst) == false{
                return adw::glib::Propagation::Proceed;
            } 
            if confirmed_window_exit.get() == true {
                return adw::glib::Propagation::Proceed;
            }

            let yes_or_no_dialog = adw::AlertDialog::new(
                Some("⚠️ Critical Warning"), 
                Some("Closing the application while operations are in progress may cause system damage.\nDo you want to exit anyway?")
            );
            yes_or_no_dialog.add_response("Cancel", "Cancel");
            yes_or_no_dialog.add_response("Exit", "Force Exit");
            yes_or_no_dialog.set_default_response(Some("Cancel"));
            yes_or_no_dialog.set_response_appearance("Exit",adw::ResponseAppearance::Destructive);


            let c2_confirmed_window_exit = Rc::clone(&confirmed_window_exit);
            let weak_window = window.downgrade();
            yes_or_no_dialog.choose(Some(window),
                                    None::<&adw::gio::Cancellable>,
                                    move |response| {
                                        if response == "Exit" {
                                            if let Some(window_) = weak_window.upgrade(){ 
                                                c2_confirmed_window_exit.set(true);
                                                window_.close();
                                            }
                                        }
                                        
                                    });
            adw::glib::Propagation::Stop
            });

        let launche_files = plugins::all_launcher::get_all_launcher();
        if launche_files.len() >0 {
            let all_appinfo: Vec<gio::AppInfo>  = {
                let mut all_appi: Vec<gio::AppInfo>  = Vec::new();
                for app_i in  gio::AppInfo::all() {
                    if app_i.should_show() && app_i.commandline().is_some() && app_i.icon().is_some() && app_i.id().is_some(){
                        all_appi.push(app_i);
                        }
                    }
                all_appi
            };
            for l_file  in launche_files.into_iter() {
                let is_supported = l_file.arch.contains(&"all") 
                                        || l_file.arch.contains(&CURRENT_ARCH);
                if is_supported != true {continue};

                let is_supported = l_file.distro_name.contains(&"all") 
                                        || l_file.distro_name.contains(&DISTRO_NAME.get().unwrap().as_str());
                if is_supported != true {continue};
                
                let is_supported = l_file.distro_version.contains(&"all") 
                                        || l_file.distro_version.contains(&DISTRO_VERSION.get().unwrap().as_str());
                if is_supported != true {continue};
                
            
                let is_supported: bool = if l_file.desktop_env.contains(&"all") {
                                            true
                                        } else {
                                            let current_desktop = DESKTOP_TYPE.get().unwrap().as_str();

                                            l_file.desktop_env.iter().any(|&type_| {
                                                type_.contains(current_desktop)
                                            })
                                        };
                if is_supported != true {continue};

                let is_supported: bool =  if l_file.display_type.contains(&"all") {
                                            true
                                        } else {
                                            let current_desktop = DISPLAY_TYPE.get().unwrap().as_str();

                                            l_file.display_type.iter().any(|&type_| {
                                                type_.contains(current_desktop)
                                            })
                                        };
                if is_supported != true {continue};
                
                    for app_i in all_appinfo.iter(){
                        if let Some(id) = app_i.id() {
                            if id == l_file.laucher_file_name {
                                    let action_row = adw::ExpanderRow::new();
                                    action_row.set_title_lines(1);
                                    action_row.set_subtitle_lines(4);
                                    action_row.set_title(&app_i.display_name());
                                    if let Some(subtitle) = app_i.description() {
                                        action_row.set_subtitle(&subtitle);
                                    }
                                    let custom_button_label = l_file.custom_button_label;
                                    let b = { 
                                        if let Some(custom_label) = custom_button_label {
                                            gtk::Button::with_label(custom_label)
                                        }else{
                                            gtk::Button::with_label("Launch")
                                        }
                                    };
                                    b.set_valign(gtk::Align::Center);
                                    b.set_halign(gtk::Align::Center);
                                    b.set_width_request(85);
                                    b.set_height_request(35);
                                    let command = app_i.executable();
                                    if command.starts_with("/usr/bin/flatpak") || command.starts_with("/bin/flatpak") {
                                        if let Some(command) = app_i.commandline() {
                                            let command_for_closure = command.clone();
                                            b.connect_clicked(move |_button| {
                                                let cmd_string = command_for_closure.to_string_lossy();
                                                let _result = Command::new("sh")
                                                                        .arg("-c")
                                                                        .arg(&*cmd_string)
                                                                        .spawn();
                                            });
                                        
                                        }
                                    }else {
                                        let command_for_closure = command.clone();
                                        b.connect_clicked(move |_button| {
                                            let cmd_string = command_for_closure.to_string_lossy();
                                            let _result = Command::new("sh")
                                                                    .arg("-c")
                                                                    .arg(&*cmd_string) // تمرير النص كـ string slice
                                                                    .spawn();
                                        });
                                    }
                                    
                                    b.set_valign(gtk::Align::Center);
                                    b.set_css_classes(&["btn-state-install"]);
                                    b.set_has_frame(false);
                                    action_row.add_suffix(&b);
                                    let row_box =  gtk::Box::new(gtk::Orientation::Horizontal, 0);
                                    row_box.set_halign(gtk::Align::End);
                                    row_box.set_margin_start(5);
                                    row_box.set_margin_end(5);
                                    row_box.set_margin_top(2);
                                    row_box.set_margin_bottom(2);
                                    row_box.set_css_classes(&["linked","flat"]);
                                    let type_b = gtk::Button::new();
                                    type_b.set_label(baseplugin::base::PluginType::get_type_label(l_file.type_)); 
                                    type_b.set_css_classes(&["running-destructive-action-button","flat"]);
                                    row_box.append(&type_b);
                                    action_row.add_row(&row_box);
                                    let icon: gtk::Image  = {
                                        if let Some(icon_path) = utils::fs::get_icon_path(l_file.icon_name)  {
                                            gtk::Image::from_file(icon_path)
                                        }else{
                                            if let Some(icon_) = app_i.icon(){
                                                if let Some(icon_name) = icon_.to_string(){
                                                    gtk::Image::from_icon_name(&icon_name)
                                                }else {
                                                    gtk::Image::from_icon_name("image-missing-symbolic")
                                                }
                                            }else{
                                                gtk::Image::from_icon_name("image-missing-symbolic")
                                            }
                                        }
                                    };
                                    icon.set_icon_size(gtk::IconSize::Large);
                                    icon.set_valign(gtk::Align::Center);
                                    action_row.add_prefix(&icon);
                                    let category = baseplugin::base::Category::get_catagory_label(l_file.category);
                                    if let Some(listbox) = category_map.get(category) {
                                        listbox.append(&action_row);
                                    }
                                
                                }
                        }
                    }
                }
            }
            
            for plugin in all_plugin.into_iter() {
                let plugin_arc: Arc<Mutex<dyn baseplugin::base::PluginTools>> = Arc::new(Mutex::new(plugin));
                let clone_plugin_arc = Arc::clone(&plugin_arc);
                let clone_plugin_arc_guard = clone_plugin_arc.lock().unwrap();
                let metadata = clone_plugin_arc_guard.metadata();
                    
                if metadata.if_true_skip == true {continue};
                let is_supported = metadata.arch.contains(&"all") 
                                        || metadata.arch.contains(&CURRENT_ARCH);
                if is_supported != true {continue};

                let is_supported = metadata.distro_name.contains(&"all") 
                                        || metadata.distro_name.contains(&DISTRO_NAME.get().unwrap().as_str());
                if is_supported != true {continue};
                
                let is_supported = metadata.distro_version.contains(&"all") 
                                        || metadata.distro_version.contains(&DISTRO_VERSION.get().unwrap().as_str());
                if is_supported != true {continue};
                
            
                let is_supported: bool = if metadata.desktop_env.contains(&"all") {
                                            true
                                        } else {
                                            let current_desktop = DESKTOP_TYPE.get().unwrap().as_str();

                                            metadata.desktop_env.iter().any(|&type_| {
                                                type_.contains(current_desktop)
                                            })
                                        };
                if is_supported != true {continue};

                let is_supported: bool =  if metadata.display_type.contains(&"all") {
                                            true
                                        } else {
                                            let current_desktop = DISPLAY_TYPE.get().unwrap().as_str();

                                            metadata.display_type.iter().any(|&type_| {
                                                type_.contains(current_desktop)
                                            })
                                        };
                if is_supported != true {continue};

                if metadata.install_in_queue {
                    let clone_click_handler_count = Arc::clone(&click_handler_count);
                    let action_row = adw::ExpanderRow::new();
                    action_row.set_title_lines(1);
                    action_row.set_subtitle_lines(4);
                    action_row.set_title(metadata.title);
                    action_row.set_subtitle(metadata.subtitle);
                    let b = utils::gui::queue_create_plugin_button(
                        &mainwindow,
                        plugin_arc,
                        Rc::clone(&lazy_fn_vec),
                        clone_click_handler_count,
                        text_view.clone(),
                        toastoverlay.clone()
                    );
                    b.set_has_frame(false);
                    b.set_valign(gtk::Align::Center);
                    action_row.add_suffix(&b);
                    let category = baseplugin::base::Category::get_catagory_label(metadata.category);
                    if let Some(listbox) = category_map.get(category) {
                        listbox.append(&action_row);
                    }
                    
                    let icon = {
                        if let Some(icon_path) = utils::fs::get_icon_path(metadata.icon_name) {
                            gtk::Image::from_file(icon_path)
                        }else {
                            gtk::Image::from_icon_name("image-missing-symbolic")
                        }
                    };
                    icon.set_icon_size(gtk::IconSize::Large);
                    icon.set_valign(gtk::Align::Center);
                    action_row.add_prefix(&icon);
                    
                    let row_box =  gtk::Box::new(gtk::Orientation::Horizontal, 0);
                    row_box.set_halign(gtk::Align::End);
                    row_box.set_margin_start(5);
                    row_box.set_margin_end(5);
                    row_box.set_margin_top(2);
                    row_box.set_margin_bottom(2);
                    row_box.set_css_classes(&["linked","flat"]);
                    let files_to_download_count: usize = clone_plugin_arc_guard.get_downlods_files_length() ;
                    if  files_to_download_count > 0 {
                        let need_download_b = gtk::Button::new();
                        need_download_b.set_label(&format!("Downloads {}",files_to_download_count));
                        need_download_b.set_css_classes(&["wait-action-button","flat"]);
                        row_box.append(&need_download_b);
                    }
                    let type_b = gtk::Button::new();
                    type_b.set_label(baseplugin::base::PluginType::get_type_label(metadata.type_)); 
                    type_b.set_css_classes(&["running-destructive-action-button","flat"]);
                    row_box.append(&type_b);
                    action_row.add_row(&row_box);
                    let website_info = metadata.website;
                    if website_info.len() >=2 {
                        let link          = website_info[0];
                        let website_label = website_info[1];
                        let web_link_button  = gtk::LinkButton::with_label(&website_label,&link);
                        web_link_button.set_css_classes(&["wait-action-button","flat"]);
                        row_box.append(&web_link_button);
                    }
                    let queue_b = gtk::Button::new();
                    queue_b.set_label("Sequential"); 
                    queue_b.set_css_classes(&["running-destructive-action-button","flat"]);
                    row_box.append(&queue_b);
                    let licenses_info = metadata.licenses;
                    for license in licenses_info {
                        let license_web  = license[0];
                        let license_type = license[1];
                        let link_button  = gtk::LinkButton::with_label(&license_type,&license_web);
                        link_button.set_css_classes(&["wait-action-button","flat"]);
                        row_box.append(&link_button);
                    }
                    
                    
                    
                }else {
                    let action_row = adw::ExpanderRow::new();
                    action_row.set_title_lines(1);
                    action_row.set_subtitle_lines(4);
                    action_row.set_title(metadata.title);
                    action_row.set_subtitle(metadata.subtitle);
                    let b = utils::gui::create_plugin_button(&mainwindow,plugin_arc,text_view.clone(),toastoverlay.clone(),Arc::clone(&non_queue_task_running_state));
                    
                    action_row.add_suffix(&b);
                    let row_box =  gtk::Box::new(gtk::Orientation::Horizontal, 0);
                    row_box.set_halign(gtk::Align::End);
                    row_box.set_margin_start(5);
                    row_box.set_margin_end(5);
                    row_box.set_margin_top(2);
                    row_box.set_margin_bottom(2);
                    row_box.set_css_classes(&["linked","flat"]);
                    let files_to_download_count: usize = clone_plugin_arc_guard.get_downlods_files_length() ;
                    if  files_to_download_count > 0 {
                        let need_download_b = gtk::Button::new();
                        need_download_b.set_label(&format!("Downloads {}",files_to_download_count));
                        need_download_b.set_css_classes(&["wait-action-button","flat"]);
                        row_box.append(&need_download_b);
                    }
                    let type_b = gtk::Button::new();
                    type_b.set_label(baseplugin::base::PluginType::get_type_label(metadata.type_)); 
                    type_b.set_css_classes(&["running-destructive-action-button","flat"]);
                    row_box.append(&type_b);
                    action_row.add_row(&row_box);
                    let website_info = metadata.website;
                    if website_info.len() >=2 {
                        let link          = website_info[0];
                        let website_label = website_info[1];
                        let web_link_button  = gtk::LinkButton::with_label(&website_label,&link);
                        web_link_button.set_css_classes(&["wait-action-button","flat"]);
                        row_box.append(&web_link_button);
                    }
                    let queue_b = gtk::Button::new();
                    queue_b.set_label("Parallel"); 
                    queue_b.set_css_classes(&["running-destructive-action-button","flat"]);
                    row_box.append(&queue_b);
                    
                    let licenses_info = metadata.licenses;
                    for license in licenses_info {
                        let license_web  = license[0];
                        let license_type = license[1];
                        let link_button  = gtk::LinkButton::with_label(&license_type,&license_web);
                        link_button.set_css_classes(&["wait-action-button","flat"]);
                        row_box.append(&link_button);
                    }
                    
                    let icon = {
                        if let Some(icon_path) = utils::fs::get_icon_path(metadata.icon_name) {
                            gtk::Image::from_file(icon_path)
                        }else {
                            gtk::Image::from_icon_name("image-missing-symbolic")
                        }
                    };
                    icon.set_icon_size(gtk::IconSize::Large);
                    icon.set_valign(gtk::Align::Center);
                    action_row.add_prefix(&icon);

                    let category = baseplugin::base::Category::get_catagory_label(metadata.category);
                    if let Some(listbox) = category_map.get(category) {
                        listbox.append(&action_row);
                    }
                }
            } 
            let all_website_plugins = plugins::all_website_plugins::get_all_website_plugins();
            for website_plugin in all_website_plugins.into_iter() {
               let is_supported = website_plugin.arch.contains(&"all") 
                                        || website_plugin.arch.contains(&CURRENT_ARCH);
                if is_supported != true {continue};

                let is_supported = website_plugin.distro_name.contains(&"all") 
                                        || website_plugin.distro_name.contains(&DISTRO_NAME.get().unwrap().as_str());
                if is_supported != true {continue};
                
                let is_supported = website_plugin.distro_version.contains(&"all") 
                                        || website_plugin.distro_version.contains(&DISTRO_VERSION.get().unwrap().as_str());
                if is_supported != true {continue};
                
            
                let is_supported: bool = if website_plugin.desktop_env.contains(&"all") {
                                            true
                                        } else {
                                            let current_desktop = DESKTOP_TYPE.get().unwrap().as_str();

                                            website_plugin.desktop_env.iter().any(|&type_| {
                                                type_.contains(current_desktop)
                                            })
                                        };
                if is_supported != true {continue};

                let is_supported: bool =  if website_plugin.display_type.contains(&"all") {
                                            true
                                        } else {
                                            let current_desktop = DISPLAY_TYPE.get().unwrap().as_str();

                                            website_plugin.display_type.iter().any(|&type_| {
                                                type_.contains(current_desktop)
                                            })
                                        };
                if is_supported != true {continue};


                let action_row = adw::ExpanderRow::new();
                action_row.set_title_lines(1);
                action_row.set_subtitle_lines(4);
                action_row.set_title(website_plugin.title);
                action_row.set_subtitle(website_plugin.subtitle);
                let b = {
                    if let Some(custom_label) = website_plugin.custom_button_label {
                        gtk::LinkButton::with_label(website_plugin.link,custom_label)
                    }
                    else {
                        gtk::LinkButton::with_label(website_plugin.link,"Open")
                    }
                };
                b.set_valign(gtk::Align::Center);
                b.set_halign(gtk::Align::Center);
                b.set_width_request(85);
                b.set_height_request(35);
                b.set_css_classes(&["btn-state-install"]);
                b.set_has_frame(false);
                action_row.add_suffix(&b);
                let row_box =  gtk::Box::new(gtk::Orientation::Horizontal, 0);
                row_box.set_halign(gtk::Align::End);
                row_box.set_margin_start(5);
                row_box.set_margin_end(5);
                row_box.set_margin_top(2);
                row_box.set_margin_bottom(2);
                row_box.set_css_classes(&["linked","flat"]);
                let type_b = gtk::Button::new();
                type_b.set_label(baseplugin::base::PluginType::get_type_label(website_plugin.type_)); 
                type_b.set_css_classes(&["running-destructive-action-button","flat"]);
                row_box.append(&type_b);
                action_row.add_row(&row_box);

                let icon = {
                    if let Some(icon_path) = utils::fs::get_icon_path(website_plugin.icon_name) {
                        gtk::Image::from_file(icon_path)
                    }else {
                        gtk::Image::from_icon_name("image-missing-symbolic")
                    }
                };
                icon.set_icon_size(gtk::IconSize::Large);
                icon.set_valign(gtk::Align::Center);
                action_row.add_prefix(&icon);

                let category = baseplugin::base::Category::get_catagory_label(website_plugin.category);
                if let Some(listbox) = category_map.get(category) {
                    listbox.append(&action_row);
                }
                
                
            }
        
        mainwindow.present();
    });
    app.run();
}

