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
mod widgets;

pub static DISTRO_NAME: OnceLock<String> = OnceLock::new();
pub static DISTRO_VERSION: OnceLock<String> = OnceLock::new();
pub static DESKTOP_TYPE: OnceLock<String> = OnceLock::new();
pub static DISPLAY_TYPE: OnceLock<String> = OnceLock::new();
pub static CONFIGDIR: OnceLock<String> = OnceLock::new();
pub static HOMEDIR: OnceLock<String> = OnceLock::new();
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
    
}

pub fn get_all_plugins() -> Vec<Box<dyn baseplugin::base::PluginTools>> {
    // ... [لا تغيير على بنية الـ get_all_plugins] ...
    vec![
        //Box::new(plugins::firefox::FirefoxPlugin::create()),
        //Box::new(plugins::firefox::FirefoxPlugin::create()),
        //Box::new(plugins::xterm::XtermPlugin::create()),
        //Box::new(plugins::xterm::XtermPlugin::create()),
        Box::new(plugins::firefox::get_plugin()),
        Box::new(plugins::xdm::get_plugin()),
        Box::new(plugins::albasheer::get_plugin()),
        Box::new(plugins::xterm::get_plugin()),
        Box::new(plugins::xterm::get_plugin()),
        Box::new(plugins::xterm::get_plugin()),
        Box::new(plugins::xterm::get_plugin()),
        Box::new(plugins::codecs::get_plugin()),
        // ... بقية الـ plugins تظل كما هي
    ]
}


fn main() {
    init_distro_info();
// 1. إنشاء بيئة Tokio لتعمل في الخلفية لمعالجة الشبكات
    let tokio_rt = tokio::runtime::Runtime::new().expect("فشل في إنشاء Tokio Runtime");
    
    // 2. إدخال المسار الحالي في سياق Tokio
    // المتغير _guard مهم جداً، بقاؤه حياً يعني بقاء سياق Tokio متاحاً طوال فترة عمل التطبيق
    let _guard = tokio_rt.enter();
    let app = adw::Application::builder().application_id("com.github.yucefmsourani.gtk4_ex").build();

    app.connect_activate(|app| {
        utils::gui::load_custom_css(CSS);

        let all_plugin = get_all_plugins();
        let mainwindow = adw::ApplicationWindow::builder().application(app).title("Gtk4 Example").build();

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
        mainwindow.add_breakpoint(breakpoint);
 
        
        let mut category_map: HashMap<&str,gtk::ListBox> = HashMap::new();
        let main_box_page          = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        let main_page_stack        = adw::ViewStack::new();
        main_page_stack.set_hexpand(true);
        main_page_stack.set_vexpand(true);
        let main_page_viewswitcher = adw::ViewSwitcherSidebar::new();
        main_page_viewswitcher.set_mode(adw::SidebarMode::Sidebar);
        main_page_viewswitcher.set_halign(gtk::Align::Start);
        main_page_viewswitcher.set_size_request(140,-1);
        main_page_viewswitcher.set_stack(Some(&main_page_stack));
        main_box_page.append(&main_page_viewswitcher);
        main_box_page.append(&main_page_stack);
        
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
            clamp.set_maximum_size(500);
            let clamp_sw      = gtk::ScrolledWindow::new();
            let clamp_listbox = gtk::ListBox::new();
            clamp_listbox.set_css_classes(&["boxed-list"]);
            clamp.set_child(Some(&clamp_sw));
            clamp_sw.set_child(Some(&clamp_listbox));
            vbox.append(&clamp);
            let _ = main_page_stack.add_titled(&vbox,Some(category),category);
            category_map.insert(category,clamp_listbox);
        }
        
        
        
        let output_box_page  = gtk::Box::new(gtk::Orientation::Vertical, 0);
        //let about_box_page   = gtk::Box::new(gtk::Orientation::Vertical, 0);
        let _ = mainstack.add_titled_with_icon(&main_box_page,Some("mhbox"),"Main","application-x-executable-symbolic");
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
            for l_file  in launche_files {
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
                                    let b = gtk::Button::with_label("Launch");
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
                                    
                                    let icon: widgets::ImagePaint  = {
                                        if let Some(icon_path) = utils::fs::get_icon_path(l_file.icon_name)  {
                                            widgets::ImagePaint::new(icon_path, 50.0, 50.0,None)
                                        }else{
                                            if let Some(icon_) = app_i.icon(){
                                                if let Some(icon_name) = icon_.to_string(){
                                                    widgets::ImagePaint::new("", 50.0, 50.0,Some(format!("{}",icon_name)))
                                                }else {
                                                    widgets::ImagePaint::new("", 50.0, 50.0,Some("image-missing-symbolic".to_string()))
                                                }
                                            }else{
                                                widgets::ImagePaint::new("", 50.0, 50.0,Some("image-missing-symbolic".to_string()))
                                            }
                                        }
                                    };

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

            if metadata.type_ != baseplugin::base::PluginType::Website  {
                if metadata.install_in_queue {
                    let clone_click_handler_count = Arc::clone(&click_handler_count);
                    let action_row = adw::ExpanderRow::new();
                    action_row.set_title_lines(1);
                    action_row.set_subtitle_lines(4);
                    action_row.set_title(metadata.title);
                    action_row.set_subtitle(metadata.subtitle);
                    let spinner = gtk::Spinner::new();
                    let b = utils::gui::queue_create_plugin_button(
                        &mainwindow,
                        plugin_arc,
                        Rc::clone(&lazy_fn_vec),
                        clone_click_handler_count,
                        text_view.clone(),
                        spinner.clone(),
                        toastoverlay.clone()
                    );
                    b.set_has_frame(false);
                    b.set_valign(gtk::Align::Center);
                    let spinner_button_box = gtk::Box::new(gtk::Orientation::Horizontal,2);
                    spinner_button_box.set_valign(gtk::Align::Center);
                    spinner_button_box.append(&spinner);
                    spinner_button_box.append(&b);
                    action_row.add_suffix(&spinner_button_box);
                    let category = baseplugin::base::Category::get_catagory_label(metadata.category);
                    if let Some(listbox) = category_map.get(category) {
                        listbox.append(&action_row);
                    }
                    
                    let icon = {
                        if let Some(icon_path) = utils::fs::get_icon_path(metadata.icon_name) {
                            widgets::ImagePaint::new(icon_path, 50.0, 50.0,None)
                        }else {
                            widgets::ImagePaint::new("", 50.0, 50.0,Some("image-missing-symbolic".to_string()))
                        }
                    };
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
                    
                    let licenses_info = metadata.licenses;
                    for license in licenses_info {
                        let license_web  = license[0];
                        let license_type = license[1];
                        let link_button  = gtk::LinkButton::with_label(&license_type,&license_web);
                        link_button.set_css_classes(&["running-destructive-action-button","flat"]);
                        row_box.append(&link_button);
                    }
                    
                    
                    
                }else {
                    let action_row = adw::ExpanderRow::new();
                    action_row.set_title_lines(1);
                    action_row.set_subtitle_lines(4);
                    action_row.set_title(metadata.title);
                    action_row.set_subtitle(metadata.subtitle);
                    let spinner = gtk::Spinner::new();
                    let b = utils::gui::create_plugin_button(&mainwindow,plugin_arc,text_view.clone(),spinner.clone(),toastoverlay.clone(),Arc::clone(&non_queue_task_running_state));
                    let spinner_button_box = gtk::Box::new(gtk::Orientation::Horizontal,2);
                    spinner_button_box.set_valign(gtk::Align::Center);
                    spinner_button_box.append(&spinner);
                    spinner_button_box.append(&b);
                    
                    action_row.add_suffix(&spinner_button_box);
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
                    
                    let licenses_info = metadata.licenses;
                    for license in licenses_info {
                        let license_web  = license[0];
                        let license_type = license[1];
                        let link_button  = gtk::LinkButton::with_label(&license_type,&license_web);
                        link_button.set_css_classes(&["running-destructive-action-button","flat"]);
                        row_box.append(&link_button);
                    }
                    
                    let icon = {
                        if let Some(icon_path) = utils::fs::get_icon_path(metadata.icon_name) {
                            widgets::ImagePaint::new(icon_path, 50.0, 50.0,None)
                        }else {
                            widgets::ImagePaint::new("", 50.0, 50.0,Some("image-missing-symbolic".to_string()))
                        }
                    };
                    icon.set_valign(gtk::Align::Center);
                    action_row.add_prefix(&icon);

                    let category = baseplugin::base::Category::get_catagory_label(metadata.category);
                    if let Some(listbox) = category_map.get(category) {
                        listbox.append(&action_row);
                    }
                }
            } else  if metadata.type_ == baseplugin::base::PluginType::Website {
                let action_row = adw::ExpanderRow::new();
                action_row.set_title_lines(1);
                action_row.set_subtitle_lines(4);
                action_row.set_title(metadata.title);
                action_row.set_subtitle(metadata.subtitle);
                let b = gtk::LinkButton::with_label(metadata.website[1],"Open");
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
                type_b.set_label(baseplugin::base::PluginType::get_type_label(metadata.type_)); 
                type_b.set_css_classes(&["running-destructive-action-button","flat"]);
                row_box.append(&type_b);
                action_row.add_row(&row_box);
                
                let icon = {
                    if let Some(icon_path) = utils::fs::get_icon_path(metadata.icon_name) {
                        widgets::ImagePaint::new(icon_path, 50.0, 50.0,None)
                    }else{
                        widgets::ImagePaint::new("", 50.0, 50.0,Some("image-missing-symbolic".to_string()))
                    }
                    
                };
                icon.set_valign(gtk::Align::Center);
                action_row.add_prefix(&icon);

                let category = baseplugin::base::Category::get_catagory_label(metadata.category);
                if let Some(listbox) = category_map.get(category) {
                    listbox.append(&action_row);
                }
                
                
            }
        }

        mainwindow.present();
    });
    app.run();
}

