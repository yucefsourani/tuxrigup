use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicI32, Ordering,AtomicBool};
use std::cell::RefCell;
use std::rc::Rc;
use futures::channel;
use futures::StreamExt;
use std::thread;
use adw::prelude::*;
use crate::baseplugin;
use gio::prelude::CancellableExt;

pub fn append_text_with_smart_scroll(text_view: &gtk::TextView, new_text: &str) {
    // 1. التحقق من حالة التمرير قبل إدراج النص
    let is_at_bottom = if let Some(vadj) = text_view.vadjustment() {
        // أقصى قيمة يمكن أن يصل إليها شريط التمرير
        let max_value = vadj.upper() - vadj.page_size();
        let current_value = vadj.value();
        
        // استخدام هامش خطأ بسيط (2.0 بكسل) لتفادي مشاكل الأرقام العشرية (Float precision)
        max_value - current_value <= 2.0
    } else {
        true // إذا لم نتمكن من الحصول على الشريط، نفعل التمرير كخيار افتراضي
    };

    // 2. إدراج النص الجديد
    let buffer = text_view.buffer();
    let mut iter = buffer.end_iter();
    buffer.insert(&mut iter, new_text);

    // 3. التمرير فقط إذا كان المستخدم عند النهاية مسبقاً
    if is_at_bottom {
        let end_iter = buffer.end_iter();
        buffer.place_cursor(&end_iter);
        
        if let Some(mark) = buffer.mark("insert") {
            text_view.scroll_to_mark(
                &mark, 
                0.0, 
                true, 
                0.0, 
                1.0
            );
        }
    }
}

pub fn append_markup_with_smart_scroll(text_view: &gtk::TextView, markup_text: &str) {
    // 1. التحقق من حالة التمرير
    let is_at_bottom = if let Some(vadj) = text_view.vadjustment() {
        let max_value = vadj.upper() - vadj.page_size();
        let current_value = vadj.value();
        max_value - current_value <= 2.0
    } else {
        true
    };

    // 2. إدراج النص بصيغة Markup
    let buffer = text_view.buffer();
    let mut iter = buffer.end_iter();
    buffer.insert_markup(&mut iter, markup_text);

    // 3. التمرير إذا كان المستخدم عند النهاية
    if is_at_bottom {
        let end_iter = buffer.end_iter();
        buffer.place_cursor(&end_iter);
        
        if let Some(mark) = buffer.mark("insert") {
            text_view.scroll_to_mark(
                &mark, 
                0.0, 
                true, 
                0.0, 
                1.0
            );
        }
    }
}

pub struct Task {
    lazy_fn: RefCell<Option<Box<dyn FnOnce()>>>,
    button: gtk::Button,
    plugin: Arc<Mutex<dyn baseplugin::base::PluginTools + 'static>>,
    label: gtk::Label,
}

pub fn load_custom_css(css_data: &str) {

    // Load the CSS file and add it to the provider
    let provider = gtk::CssProvider::new();
    provider.load_from_string(css_data);

    // Add the provider to the default screen
    gtk::style_context_add_provider_for_display(
        &gtk::gdk::Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

pub fn create_plugin_button(
    parent: &adw::ApplicationWindow, 
    plugin: Arc<Mutex<dyn baseplugin::base::PluginTools + 'static>>,
    textview: gtk::TextView,
    spinner: gtk::Spinner,
    toastoverlay: adw::ToastOverlay,
    non_queue_task_running_state: Arc<AtomicBool>,
) -> gtk::Button {

    let progressbar = gtk::ProgressBar::new();
    progressbar.set_visible(false);
    let parent_clone = parent.clone();
    let b_label = gtk::Label::new(Some("Loading..."));
    let btn = gtk::Button::builder()
        .margin_end(5)
        .sensitive(false)
        .build();
    btn.set_valign(gtk::Align::Center);
    btn.set_has_frame(false);
    let button_label_progressbar_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
    button_label_progressbar_box.set_hexpand(false);
    button_label_progressbar_box.set_valign(gtk::Align::Center);
    button_label_progressbar_box.append(&b_label);
    button_label_progressbar_box.append(&progressbar);
    btn.set_child(Some(&button_label_progressbar_box));
    
    let (tx, rx) = channel::oneshot::channel();
    let plugin_clone1 = Arc::clone(&plugin);
    let weak_button = btn.downgrade();
    let spinner_clone1 = spinner.clone();
    spinner.start();
    btn.set_css_classes(&["btn-state-waiting"]);
    non_queue_task_running_state.store(true,Ordering::SeqCst);
    let c1_non_queue_task_running_state = Arc::clone(&non_queue_task_running_state);
    let c1_b_label = b_label.clone();
    glib::MainContext::default().spawn_local(async move {
        if let Ok(_message) = rx.await {
            spinner_clone1.stop();
            c1_non_queue_task_running_state.store(false,Ordering::SeqCst);
            if let Some(button) = weak_button.upgrade() {
                let need_install: bool = plugin_clone1.lock().unwrap().get_need_install();
                let pc = plugin_clone1.clone();
                drop(plugin_clone1);
                glib::idle_add_local_once(move || {
                    button.set_sensitive(true);
                    let guard = pc.lock().unwrap();
                    let metadata = guard.metadata();
                    if need_install {
                        c1_b_label.set_label(metadata.button_install_label);
                        button.set_css_classes(&["btn-state-install"]);
                    } else {
                        c1_b_label.set_label(metadata.button_remove_label);
                        if metadata.type_ != baseplugin::base::PluginType::Oneshot{
                            button.set_css_classes(&["btn-state-remove"]);
                        }
                    }
                });
            }
        }
    });
    
    let plugin_clone = Arc::clone(&plugin);
    thread::spawn(move || {
        let mut guard = plugin_clone.lock().unwrap();
        let need_install: bool = guard.need_install();
        guard.set_need_install(need_install);
        let _ = tx.send("");
    });

    // 1. تعريف مخزن مشترك للـ Cancellable الفعال خارج حدث الضغط ليعيش بين النقرات المتعددة
    let active_cancellable: Rc<RefCell<Option<gio::Cancellable>>> = Rc::new(RefCell::new(None));
    let active_cancellable_clone = Rc::clone(&active_cancellable);

    let spinner_clone2 = spinner.clone();
    let clone1_toastoverlay = toastoverlay.clone();
    let c2_non_queue_task_running_state = Arc::clone(&non_queue_task_running_state);
    let progressbar1 = progressbar.clone();
    let c2_b_label = b_label.clone();
    btn.connect_clicked(move |button| {
        let choice2_clone_plugin = Arc::clone(&plugin);
        let plugin_guard = plugin.lock().unwrap();
        let is_running = plugin_guard.get_install_is_running();
        drop(plugin_guard);
        let c3_non_queue_task_running_state = Arc::clone(&c2_non_queue_task_running_state);
        let spinner_clone3 = spinner_clone2.clone();
        if is_running {
            if let Some(current_cancellable) = active_cancellable_clone.borrow().clone() {
                let choice_clone_cancellable = current_cancellable.clone();
                let choice_parent_clone = parent_clone.clone();
                let inner_non_queue_task_running_state = Arc::clone(&c2_non_queue_task_running_state);
                glib::MainContext::default().spawn_local(async move { 
                    let custom_cancel_warning_message = choice2_clone_plugin.lock().unwrap().metadata().custom_cancel_warning_message;
                    let yes_or_no_dialog = {
                        if let Some(header_body_msg) =  custom_cancel_warning_message {
                            adw::AlertDialog::new(Some(header_body_msg[0]), Some(header_body_msg[1]))
                        }else{
                            adw::AlertDialog::new(Some("Warning: Potential System Corruption"), Some("Warning: Canceling this action while it is running poses a high risk to your system's integrity. It may result in broken components or data corruption.\n Are you sure you want to abort?"))
                        }};
                    yes_or_no_dialog.add_response("Yes", "Cancel Anyway");
                    yes_or_no_dialog.add_response("No", "Keep Running");
                    yes_or_no_dialog.set_default_response(Some("No"));
                    yes_or_no_dialog.set_response_appearance("Yes",adw::ResponseAppearance::Destructive);
                    yes_or_no_dialog.set_body_use_markup(true);
                    yes_or_no_dialog.set_heading_use_markup(true);
                    let response = yes_or_no_dialog.choose_future(Some(&choice_parent_clone)).await;
                    if response == "Yes" {
                        // إلغاء الكائن الفعلي الصحيح الذي تستمع إليه دالة read_bytes_async
                        choice_clone_cancellable.cancel();
                        spinner_clone3.stop();
                        inner_non_queue_task_running_state.store(false,Ordering::SeqCst);
                    }
                });
            }
            return;
        }
        
        let clonetextview = textview.clone();
        let button_clone = button.clone();
        let button_clone2 = button.clone();
        
        let plugin_clone3 = Arc::clone(&plugin);
        let plugin_clone4 = Arc::clone(&plugin);
        let parent_clone2 = parent_clone.clone();
        
        // إنشاء كائن الإلغاء الخاص بهذه العملية وحفظه في المخزن المشترك
        let new_cancellable = gio::Cancellable::new();
        *active_cancellable_clone.borrow_mut() = Some(new_cancellable.clone());
        let clone1_cancellable = new_cancellable.clone();

        let (tx, mut rx) = futures::channel::mpsc::unbounded::<baseplugin::base::OutMesseageType>();
        let clone_tx = tx.clone();
        let spinner_clone4 = spinner_clone2.clone();
        let c4_non_queue_task_running_state = Arc::clone(&c3_non_queue_task_running_state);
        let progressbar2 = progressbar1.clone();
        let c_b_label = c2_b_label.clone();
        glib::MainContext::default().spawn_local(async move {
            let mut plugin_clone4_guard     = plugin_clone4.lock().unwrap();
            let (dialog_header, dialog_label) = {
                let is_need_install: bool = plugin_clone4_guard.get_need_install();
                let metadata =  plugin_clone4_guard.metadata();
                if is_need_install{
                    (
                        metadata.install_yes_or_no_header,
                        metadata.install_yes_or_no_label
                    )
                }else {
                    (
                        metadata.remove_yes_or_no_header,
                        metadata.remove_yes_or_no_label
                    )
                }
            };
            let plugin_clone4_metadata  = plugin_clone4_guard.metadata(); 
            if plugin_clone4_metadata.yes_or_no {
                let yes_or_no_dialog = adw::AlertDialog::new(Some(dialog_header), Some(dialog_label));
                yes_or_no_dialog.add_response("No", "No");
                yes_or_no_dialog.add_response("Yes", "Yes");
                yes_or_no_dialog.set_default_response(Some("Yes"));
                yes_or_no_dialog.set_response_appearance("Yes",adw::ResponseAppearance::Suggested);
                yes_or_no_dialog.set_body_use_markup(true);
                yes_or_no_dialog.set_heading_use_markup(true);
                let response = yes_or_no_dialog.choose_future(Some(&parent_clone2)).await;
                if response == "No" {
                    button_clone2.set_sensitive(true);
                    return;
                }
            }
            

           
            c3_non_queue_task_running_state.store(true,Ordering::SeqCst);
            let need_install: bool = plugin_clone4_guard.get_need_install();
            spinner_clone4.start();
            if need_install {
                c_b_label.set_label(plugin_clone4_metadata.button_install_running_label);
            } else {
                c_b_label.set_label(plugin_clone4_metadata.button_remove_running_label);
            }
            let files_to_dowmload_count: usize = plugin_clone4_guard.get_downlods_files_length();
            plugin_clone4_guard.set_install_is_running(true);
            if plugin_clone4_guard.get_need_install() {
                if files_to_dowmload_count > 0 {
                    progressbar2.set_visible(true);
                    plugin_clone4_guard.download_files(clone_tx, clone1_cancellable);
                }else {
                    plugin_clone4_guard.install(clone_tx, clone1_cancellable,None);
                }
            } else {
                plugin_clone4_guard.remove(clone_tx, clone1_cancellable);
            }
            
        });
        let spinner_clone5 = spinner_clone2.clone();
        let active_cancellable_clear = Rc::clone(&active_cancellable_clone);
        let clone2_toastoverlay = clone1_toastoverlay.clone();
        let c5_non_queue_task_running_state = Arc::clone(&c4_non_queue_task_running_state);
        let progressbar3 = progressbar1.clone();
        let clone_tx2 = tx.clone();
        let clone1_cancellable2 = new_cancellable.clone();
        let c3_b_label = c2_b_label.clone();
        glib::MainContext::default().spawn_local(async move {
            let clone_tx3 = clone_tx2.clone();
            let clone1_cancellable3 = clone1_cancellable2.clone();
            while let Some(message) = rx.next().await { 
                let mut plugin_clone3_guard =  plugin_clone3.lock().unwrap();
                let plugin_clone3_metadata = plugin_clone3_guard.metadata();
                let mut new_need_install: bool  = plugin_clone3_guard.get_need_install();
                let original_need_install: bool = plugin_clone3_guard.get_need_install();
                match message {
                    baseplugin::base::OutMesseageType::Progress(downloadfractioninfo) => {
                        let percent = (downloadfractioninfo.fraction * 100.0) as i32;
                        c3_b_label.set_label(&format!("Download({}/{}) {}%",downloadfractioninfo.filenumber,downloadfractioninfo.countfiles, percent));
                        progressbar3.set_fraction(downloadfractioninfo.fraction);
                    },
                    baseplugin::base::OutMesseageType::DownloadState(files_location) => {
                        progressbar3.set_visible(false);
                        progressbar3.set_fraction(0.0);
                        match files_location {
                            Some(f_location) =>  { 
                                    append_text_with_smart_scroll(&clonetextview,&format!("Download Done.\n"));
                                    if original_need_install {
                                        c3_b_label.set_label(plugin_clone3_metadata.button_install_running_label);
                                    } else {
                                        c3_b_label.set_label(plugin_clone3_metadata.button_remove_running_label);
                                    }
                                    plugin_clone3_guard.install(clone_tx3.clone(),clone1_cancellable3.clone(),Some(f_location));
                                    },
                                _ => {
                                    append_text_with_smart_scroll(&clonetextview,&format!("Download Failed\n"));
                                    c5_non_queue_task_running_state.store(false,Ordering::SeqCst);
                                    *active_cancellable_clear.borrow_mut() = None;
                                    spinner_clone5.stop();
                                    button_clone.set_sensitive(true);
                                    if new_need_install {
                                        c3_b_label.set_label(plugin_clone3_metadata.button_install_label);
                                    } else {
                                        c3_b_label.set_label(plugin_clone3_metadata.button_remove_label);
                                    }
                                    plugin_clone3_guard.set_install_is_running(false);
                                },
                        };
                    },
                    baseplugin::base::OutMesseageType::DownloadCancelled => {
                        progressbar3.set_fraction(0.0);
                        progressbar3.set_visible(false);
                        append_text_with_smart_scroll(&clonetextview,&format!("Download Cancelled\n"));
                        c5_non_queue_task_running_state.store(false,Ordering::SeqCst);
                        *active_cancellable_clear.borrow_mut() = None;
                        spinner_clone5.stop();
                        button_clone.set_sensitive(true);
                        if new_need_install {
                            c3_b_label.set_label(plugin_clone3_metadata.button_install_label);
                        } else {
                            c3_b_label.set_label(plugin_clone3_metadata.button_remove_label);
                        }
                        plugin_clone3_guard.set_install_is_running(false);
                    },
                    baseplugin::base::OutMesseageType::DownloadError => {
                        progressbar3.set_fraction(0.0);
                        progressbar3.set_visible(false);
                        append_text_with_smart_scroll(&clonetextview,&format!("Download Failed\n"));
                        c5_non_queue_task_running_state.store(false,Ordering::SeqCst);
                        *active_cancellable_clear.borrow_mut() = None;
                        spinner_clone5.stop();
                        button_clone.set_sensitive(true);
                        if new_need_install {
                            c3_b_label.set_label(plugin_clone3_metadata.button_install_label);
                        } else {
                            c3_b_label.set_label(plugin_clone3_metadata.button_remove_label);
                        }
                        plugin_clone3_guard.set_install_is_running(false);
                    },
                    baseplugin::base::OutMesseageType::Message(msg) => {
                        append_text_with_smart_scroll(&clonetextview,&format!("{}\n", msg));
                    },
                    baseplugin::base::OutMesseageType::Cancelled => {
                        append_text_with_smart_scroll(&clonetextview,&format!("Cancelled\n",));
                        c5_non_queue_task_running_state.store(false,Ordering::SeqCst);
                        *active_cancellable_clear.borrow_mut() = None;
                        spinner_clone5.stop();
                        button_clone.set_sensitive(true);
                        if new_need_install {
                            c3_b_label.set_label(plugin_clone3_metadata.button_install_label);
                        } else {
                            c3_b_label.set_label(plugin_clone3_metadata.button_remove_label);
                        }
                        plugin_clone3_guard.set_install_is_running(false);
                    },
                    baseplugin::base::OutMesseageType::State(state) => {
                        // بمجرد انتهاء العملية (بنجاح أو فشل أو إلغاء) نقوم بتصفير المخزن لإنهاء حالة الـ Running
                        c5_non_queue_task_running_state.store(false,Ordering::SeqCst);
                        *active_cancellable_clear.borrow_mut() = None;
                        spinner_clone5.stop();
                        button_clone.set_sensitive(true);
                        if state == true {
                            append_text_with_smart_scroll(&clonetextview,&format!("Done.\n"));
                            new_need_install = !new_need_install;
                            if new_need_install {
                                c3_b_label.set_label(plugin_clone3_metadata.button_install_label);
                                button_clone.set_css_classes(&["btn-state-install"]);
                                let after_success_remove_message = plugin_clone3_metadata.after_success_remove_message;
                                if let Some(remove_success_msg)  = after_success_remove_message {
                                    let toast = adw::Toast::new(remove_success_msg);
                                    toast.set_use_markup(true);
                                    clone2_toastoverlay.add_toast(toast);
                                    append_markup_with_smart_scroll(&clonetextview,&format!("{}\n",remove_success_msg));
                                }
                            } else {
                                c3_b_label.set_label(plugin_clone3_metadata.button_remove_label);
                                if plugin_clone3_metadata.type_ != baseplugin::base::PluginType::Oneshot{
                                    button_clone.set_css_classes(&["btn-state-remove"]);
                                }
                                let after_success_install_message = plugin_clone3_metadata.after_success_install_message;
                                if let Some(install_success_msg)  = after_success_install_message {
                                    let toast = adw::Toast::new(install_success_msg);
                                    toast.set_use_markup(true);
                                    clone2_toastoverlay.add_toast(toast);
                                    append_markup_with_smart_scroll(&clonetextview,&format!("{}\n",install_success_msg));
                                }
                            }

                        } else {
                            if new_need_install {
                                c3_b_label.set_label(plugin_clone3_metadata.button_install_label);
                                button_clone.set_css_classes(&["btn-state-install"]);
                            } else {
                                c3_b_label.set_label(plugin_clone3_metadata.button_remove_label);
                                if plugin_clone3_metadata.type_ != baseplugin::base::PluginType::Oneshot{
                                    button_clone.set_css_classes(&["btn-state-remove"]);
                                }
                            }
                            append_text_with_smart_scroll(&clonetextview,&format!("Failed or Cancelled.\n"));
                        }
                    plugin_clone3_guard.set_need_install(new_need_install);
                    plugin_clone3_guard.set_install_is_running(false);
                    },
                    _ => {
                        c5_non_queue_task_running_state.store(false,Ordering::SeqCst);
                        *active_cancellable_clear.borrow_mut() = None;
                        spinner_clone5.stop();
                        button_clone.set_sensitive(true);
                        if new_need_install {
                            c3_b_label.set_label(plugin_clone3_metadata.button_install_label);
                        } else {
                            c3_b_label.set_label(plugin_clone3_metadata.button_remove_label);
                        }
                        append_text_with_smart_scroll(&clonetextview,&format!("ERROR.\n"));
                        plugin_clone3_guard.set_install_is_running(false);
                    },
                }    
            }
        });
    });

    btn
}


pub fn _queue_create_plugin_button(
    parent: &adw::ApplicationWindow, 
    plugin: Arc<Mutex<dyn baseplugin::base::PluginTools + 'static>>,
    // تم التغيير لـ Rc + RefCell لمنع استخدام Mutex في خيط الواجهة وحل مشكلة الـ Deadlocks نهائياً
    lazy_fn_vec: Rc<RefCell<Vec<Box<Task>>>>,
    click_handler_count: Arc<AtomicI32>,
    textview: gtk::TextView,
    spinner: gtk::Spinner,
    toastoverlay: adw::ToastOverlay,
) -> gtk::Button {

    let parent_clone   = parent.clone();
    let btn = gtk::Button::builder()
        .label("Loading...")
        .margin_end(5)
        .sensitive(false)
        .build();

    let progressbar =  gtk::ProgressBar::new();
    progressbar.set_visible(false);
    let button_progressbar_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
    button_progressbar_box.set_hexpand(false);
    button_progressbar_box.set_valign(gtk::Align::Center);
    let b_label = gtk::Label::new(Some("Loading..."));
    button_progressbar_box.append(&b_label);
    button_progressbar_box.append(&progressbar);
    btn.set_child(Some(&button_progressbar_box));

    let (tx, rx) = channel::oneshot::channel();
    let plugin_clone1 = Arc::clone(&plugin);
    let weak_button = btn.downgrade();
    let spinner_clone1 = spinner.clone();
    spinner.start();
    btn.set_css_classes(&["btn-state-waiting"]);
    let c_b_label = b_label.clone();
    glib::MainContext::default().spawn_local(async move  {
        let c2_b_label = c_b_label.clone();
        if let Ok(_message) = rx.await {
            spinner_clone1.stop();
            if let Some(button) = weak_button.upgrade(){
                let need_install: bool = plugin_clone1.lock().unwrap().get_need_install();
                let pc = plugin_clone1.clone();
                drop(plugin_clone1);
                glib::idle_add_local_once(move || {
                    button.set_sensitive(true);
                    if need_install {
                        c2_b_label.set_label(pc.lock().unwrap().metadata().button_install_label);
                        button.set_css_classes(&["btn-state-install"]);
                    }else {
                        c2_b_label.set_label(pc.lock().unwrap().metadata().button_remove_label);
                        if pc.lock().unwrap().metadata().type_ != baseplugin::base::PluginType::Oneshot{
                            button.set_css_classes(&["btn-state-remove"]);
                        }
                    }
                });

            }};

    });

    let plugin_clone = Arc::clone(&plugin);
    thread::spawn( move || {
        let need_install: bool = plugin_clone.lock().unwrap().need_install();
        plugin_clone.lock().unwrap().set_need_install(need_install);
        let _ = tx.send("");
    });

    let _clone_click_handler_count = Arc::clone(&click_handler_count);
    let _clone_lazy_fn_vec = Rc::clone(&lazy_fn_vec);

    //let cancellable = gio::Cancellable::new();
    //let clone1_cancellable = cancellable.clone();
    let active_cancellable: Rc<RefCell<Option<gio::Cancellable>>> = Rc::new(RefCell::new(None));
    let active_cancellable_clone = Rc::clone(&active_cancellable);
    let clone1_toastoverlay = toastoverlay.clone();
    let spinner_clone2 = spinner.clone();
    let progressbar_clone1 = progressbar.clone();
    let c3_b_label = b_label.clone();
    btn.connect_clicked(move |button| {

        let clone_click_handler_count = Arc::clone(&_clone_click_handler_count);
        let clone_lazy_fn_vec = Rc::clone(&_clone_lazy_fn_vec);

        let mut queue = clone_lazy_fn_vec.borrow_mut();
        let c4_b_label = c3_b_label.clone();
        if let Some(index) = queue.iter().position(|item| item.button == *button) {
            queue.remove(index);
            drop(queue); 

            clone_click_handler_count.fetch_sub(1, Ordering::SeqCst);
            let need_install: bool = plugin.lock().unwrap().get_need_install();
            if need_install {
                c4_b_label.set_label(plugin.lock().unwrap().metadata().button_install_label);
                button.set_css_classes(&["btn-state-install"]);
            } else {
                c4_b_label.set_label(plugin.lock().unwrap().metadata().button_remove_label);
                if plugin.lock().unwrap().metadata().type_ != baseplugin::base::PluginType::Oneshot{
                    button.set_css_classes(&["btn-state-remove"]);
                }
            }
            return;
        }
        drop(queue); 


        let choice_clone_plugin = plugin.clone();
        let choice2_clone_plugin = plugin.clone();
        let spinner_clone3 = spinner_clone2.clone();

        if choice_clone_plugin.lock().unwrap().get_install_is_running() == true {
            if let Some(current_cancellable) = active_cancellable_clone.borrow().clone() {
                let choice_clone_cancellable = current_cancellable.clone();
                let choice_parent_clone = parent_clone.clone();
                glib::MainContext::default().spawn_local(async move { 
                        let custom_cancel_warning_message = choice2_clone_plugin.lock().unwrap().metadata().custom_cancel_warning_message;
                        let yes_or_no_dialog = {
                            if let Some(header_body_msg) =  custom_cancel_warning_message {
                                adw::AlertDialog::new(Some(header_body_msg[0]), Some(header_body_msg[1]))
                            }else{
                                adw::AlertDialog::new(Some("Warning: Potential System Corruption"), Some("Warning: Canceling this action while it is running poses a high risk to your system's integrity. It may result in broken components or data corruption.\nAre you sure you want to abort?"))
                            }};

                        yes_or_no_dialog.add_response("No", "Keep Running");
                        yes_or_no_dialog.add_response("Yes", "Cancel Anyway");
                        yes_or_no_dialog.set_default_response(Some("No"));
                        yes_or_no_dialog.set_response_appearance("Yes",adw::ResponseAppearance::Destructive);
                        yes_or_no_dialog.set_body_use_markup(true);
                        yes_or_no_dialog.set_heading_use_markup(true);
                        let response = yes_or_no_dialog.choose_future(Some(&choice_parent_clone)).await;

                        if response == "Yes" {
                            choice_clone_cancellable.cancel();
                            spinner_clone3.stop();
                        }
                });
            }
        return;
        }
        let _button_clone   = button.clone();
        let _parent_clone   = parent_clone.clone();
        let _plugin_clone   = Arc::clone(&plugin);
        let _textview = textview.clone();
        let new_cancellable = gio::Cancellable::new();
        *active_cancellable_clone.borrow_mut() = Some(new_cancellable.clone());

        let clone_cancellable = new_cancellable.clone();

        let spinner_clone4 = spinner_clone2.clone();
        let clone2_toastoverlay = clone1_toastoverlay.clone();
        let progressbar_clone2 = progressbar_clone1.clone();
        let c5_b_label = c3_b_label.clone();
        if _plugin_clone.lock().unwrap().get_install_is_running() == false{
            let progressbar_clone3 = progressbar_clone2.clone();
            glib::MainContext::default().spawn_local(async move { 
                let (dialog_header, dialog_label) = {
                    let guard    = _plugin_clone.lock().unwrap();
                    let is_need_install: bool = guard.get_need_install();
                    let metadata =  guard.metadata();
                    if is_need_install{
                        (
                            metadata.install_yes_or_no_header,
                            metadata.install_yes_or_no_label
                        )
                    }else {
                        (
                            metadata.remove_yes_or_no_header,
                            metadata.remove_yes_or_no_label
                        )
                    }
                };
                if _plugin_clone.lock().unwrap().metadata().yes_or_no {
                    let yes_or_no_dialog = adw::AlertDialog::new(Some(dialog_header), Some(dialog_label));
                    yes_or_no_dialog.add_response("No", "No");
                    yes_or_no_dialog.add_response("Yes", "Yes");
                    yes_or_no_dialog.set_default_response(Some("Yes"));
                    yes_or_no_dialog.set_response_appearance("Yes",adw::ResponseAppearance::Suggested);
                    yes_or_no_dialog.set_body_use_markup(true);
                    yes_or_no_dialog.set_heading_use_markup(true);
                    let response = yes_or_no_dialog.choose_future(Some(&_parent_clone)).await;

                    if response == "No" {
                        _button_clone.set_sensitive(true);
                        return;
                    }
                }

                // ب) اللحظة الحاسمة: المستخدم ضغط "Yes" وانتهى الانتظار!
                // الآن نأخذ قرار الجدولة بناءً على حالة العداد الفعلي في هذه الأجزاء من الثانية
                let current_active_tasks = clone_click_handler_count.load(Ordering::SeqCst);
                let is_queue_busy = current_active_tasks > 0;

                // تجهيز الـ Closure الخاص بالمهمة للتشغيل أو الحفظ
                let button_clone  = _button_clone.clone();
                let plugin_clone3 = Arc::clone(&_plugin_clone);
                let plugin_clone2 = Arc::clone(&_plugin_clone);
                let run_lazy_fn_vec = Rc::clone(&clone_lazy_fn_vec);
                let run_click_handler_count = Arc::clone(&clone_click_handler_count);
                let clonetextview = _textview.clone();
                let clone2_cancellable = clone_cancellable.clone();
                let spinner_clone5 = spinner_clone4.clone();
                let clone3_toastoverlay = clone2_toastoverlay.clone();
                let progressbar_clone4 = progressbar_clone3.clone();
                let c6_b_label    = c5_b_label.clone();
                let lazy_fn = move || {
                    //button_clone.set_sensitive(false);
                    spinner_clone5.start();
                    let need_install: bool = plugin_clone3.lock().unwrap().get_need_install();
                    if need_install {
                        c6_b_label.set_label(plugin_clone3.lock().unwrap().metadata().button_install_running_label);
                    } else {
                        c6_b_label.set_label(plugin_clone3.lock().unwrap().metadata().button_remove_running_label);
                    }

                    let (tx, mut rx) = futures::channel::mpsc::unbounded::<baseplugin::base::OutMesseageType>();
                    let clone_tx = tx.clone();
                    let i_clone1_tx = tx.clone();
                    let i_clone1_cancellable = clone2_cancellable.clone();
                    let files_to_dowmload_count: usize = plugin_clone2.lock().unwrap().get_downlods_files_length();
                    if plugin_clone2.lock().unwrap().get_need_install() {
                        if files_to_dowmload_count > 0 {
                            progressbar_clone4.set_visible(true);
                            plugin_clone2.lock().unwrap().download_files(clone_tx, clone2_cancellable);
                        }else {
                            plugin_clone2.lock().unwrap().install(clone_tx, clone2_cancellable,None);
                        }
                    } else {
                        plugin_clone2.lock().unwrap().remove(clone_tx,clone2_cancellable);
                    }

                    let inner_lazy_fn_vec = Rc::clone(&run_lazy_fn_vec);
                    let inner_click_handler_count = Arc::clone(&run_click_handler_count);
                    let spinner_clone6 = spinner_clone5.clone();
                    let clone4_toastoverlay = clone3_toastoverlay.clone();
                    let progressbar_clone5 = progressbar_clone4.clone();
                    let c7_b_label = c6_b_label.clone();
                    glib::MainContext::default().spawn_local(async move {
                        let i_clone2_tx = i_clone1_tx.clone();
                        let i_clone2_cancellable = i_clone1_cancellable.clone();
                        let progressbar_clone6 = progressbar_clone5.clone();
                        let c8_b_label = c7_b_label.clone();
                        while let Some(message) = rx.next().await { 
                            let mut new_need_install: bool = plugin_clone3.lock().unwrap().get_need_install();
                            let original_need_install: bool = plugin_clone3.lock().unwrap().get_need_install();
                            match message {
                                baseplugin::base::OutMesseageType::Progress(downloadfractioninfo) => {
                                    let percent = (downloadfractioninfo.fraction * 100.0) as i32;
                                    c8_b_label.set_label(&format!("Download({}/{}) {}%",downloadfractioninfo.filenumber,downloadfractioninfo.countfiles, percent));
                                    progressbar_clone6.set_fraction(downloadfractioninfo.fraction);
                                },
                                baseplugin::base::OutMesseageType::DownloadState(files_location) => {
                                    match files_location {
                                        Some(f_location) =>  {  
                                            progressbar_clone6.set_visible(false);
                                            progressbar_clone6.set_fraction(0.0);
                                            
                                            append_text_with_smart_scroll(&clonetextview,&format!("Download Done.\n"));
                                            if original_need_install {
                                                c8_b_label.set_label(plugin_clone3.lock().unwrap().metadata().button_install_running_label);
                                            } else {
                                                c8_b_label.set_label(plugin_clone3.lock().unwrap().metadata().button_remove_running_label);
                                            }
                                            plugin_clone3.lock().unwrap().install(i_clone2_tx.clone(),i_clone2_cancellable.clone(),Some(f_location));
                                        },
                                        _ => {
                                            inner_click_handler_count.fetch_sub(1, Ordering::SeqCst);
                                            append_text_with_smart_scroll(&clonetextview,&format!("Download Failed\n"));
                                            plugin_clone3.lock().unwrap().set_install_is_running(false);
                                            //*i_clone2_cancellable.borrow_mut() = None;
                                            spinner_clone6.stop();
                                            button_clone.set_sensitive(true);
                                            if original_need_install {
                                                c8_b_label.set_label(plugin_clone3.lock().unwrap().metadata().button_install_label);
                                            } else {
                                                c8_b_label.set_label(plugin_clone3.lock().unwrap().metadata().button_remove_label);
                                            }
                                            if inner_click_handler_count.load(Ordering::SeqCst) > 0 {
                                                let mut q = inner_lazy_fn_vec.borrow_mut();
                                                if let Some(task) = q.pop() {
                                                    if let Some(f) = task.lazy_fn.borrow_mut().take() {
                                                        let plugin_type = task.plugin.lock().unwrap().metadata().type_;
                                                        if task.plugin.lock().unwrap().get_need_install() {
                                                            task.label.set_label(task.plugin.lock().unwrap().metadata().button_install_running_label);
                                                            task.button.set_css_classes(&["btn-state-install"]);
                                                        } else {
                                                            task.label.set_label(task.plugin.lock().unwrap().metadata().button_remove_running_label);
                                                            if plugin_type != baseplugin::base::PluginType::Oneshot{
                                                                task.button.set_css_classes(&["btn-state-remove"]);
                                                            }
                                                        }
                                                        //task.button.set_sensitive(false);
                                                        task.plugin.lock().unwrap().set_install_is_running(true);
                                                        f();
                                                    }
                                                }
                                            }
                                        },
                                    }
                                },
                                baseplugin::base::OutMesseageType::DownloadCancelled => {
                                    progressbar_clone6.set_fraction(0.0);
                                    progressbar_clone6.set_visible(false);
                                    plugin_clone3.lock().unwrap().set_install_is_running(false);
                                    spinner_clone6.stop();
                                    inner_click_handler_count.fetch_sub(1, Ordering::SeqCst);
                                    append_text_with_smart_scroll(&clonetextview,&format!("Download Cancelled.\n"));
                                    if original_need_install {
                                        c8_b_label.set_label(plugin_clone3.lock().unwrap().metadata().button_install_label);
                                        button_clone.set_css_classes(&["btn-state-install"]);
                                    } else {
                                        c8_b_label.set_label(plugin_clone3.lock().unwrap().metadata().button_remove_label);
                                        if plugin_clone3.lock().unwrap().metadata().type_ != baseplugin::base::PluginType::Oneshot{
                                            button_clone.set_css_classes(&["btn-state-remove"]);
                                        }
                                    }
                                    if inner_click_handler_count.load(Ordering::SeqCst) > 0 {
                                        let mut q = inner_lazy_fn_vec.borrow_mut();
                                        if let Some(task) = q.pop() {
                                            if let Some(f) = task.lazy_fn.borrow_mut().take() {
                                                let plugin_type = task.plugin.lock().unwrap().metadata().type_;
                                                if task.plugin.lock().unwrap().get_need_install() {
                                                    task.label.set_label(task.plugin.lock().unwrap().metadata().button_install_running_label);
                                                    task.button.set_css_classes(&["btn-state-install"]);
                                                } else {
                                                    task.label.set_label(task.plugin.lock().unwrap().metadata().button_remove_running_label);
                                                    if plugin_type != baseplugin::base::PluginType::Oneshot{
                                                        task.button.set_css_classes(&["btn-state-remove"]);
                                                    }
                                                }
                                                //task.button.set_sensitive(false);
                                                task.plugin.lock().unwrap().set_install_is_running(true);
                                                f();
                                            }
                                        }
                                    }
                                },
                                baseplugin::base::OutMesseageType::DownloadError => {
                                    progressbar_clone6.set_fraction(0.0);
                                    progressbar_clone6.set_visible(false);
                                    button_clone.set_sensitive(true);
                                    spinner_clone6.stop();
                                    plugin_clone3.lock().unwrap().set_install_is_running(false);
                                    if original_need_install {
                                        c8_b_label.set_label(plugin_clone3.lock().unwrap().metadata().button_install_label);
                                        button_clone.set_css_classes(&["btn-state-install"]);
                                    } else {
                                        c8_b_label.set_label(plugin_clone3.lock().unwrap().metadata().button_remove_label);
                                        if plugin_clone3.lock().unwrap().metadata().type_ != baseplugin::base::PluginType::Oneshot{
                                            button_clone.set_css_classes(&["btn-state-remove"]);
                                        }
                                    }
                                    append_text_with_smart_scroll(&clonetextview,&format!("Download ERROR\n"));
                                    inner_click_handler_count.fetch_sub(1, Ordering::SeqCst);
                                    if inner_click_handler_count.load(Ordering::SeqCst) > 0 {
                                        let mut q = inner_lazy_fn_vec.borrow_mut();
                                        if let Some(task) = q.pop() {
                                            if let Some(f) = task.lazy_fn.borrow_mut().take() {
                                                let plugin_type = task.plugin.lock().unwrap().metadata().type_;
                                                if task.plugin.lock().unwrap().get_need_install() {
                                                    task.label.set_label(task.plugin.lock().unwrap().metadata().button_install_running_label);
                                                    task.button.set_css_classes(&["btn-state-install"]);
                                                } else {
                                                    task.label.set_label(task.plugin.lock().unwrap().metadata().button_remove_running_label);
                                                    if plugin_type != baseplugin::base::PluginType::Oneshot{
                                                        task.button.set_css_classes(&["btn-state-remove"]);
                                                    }
                                                }
                                                //task.button.set_sensitive(false);
                                                task.plugin.lock().unwrap().set_install_is_running(true);
                                                f();
                                            }
                                        }
                                    }

                                },
                                baseplugin::base::OutMesseageType::Message(msg) => {
                                    append_text_with_smart_scroll(&clonetextview,&format!("{}\n", msg));
                                },
                                baseplugin::base::OutMesseageType::Cancelled => {
                                    plugin_clone3.lock().unwrap().set_install_is_running(false);
                                    spinner_clone6.stop();
                                    inner_click_handler_count.fetch_sub(1, Ordering::SeqCst);
                                    append_text_with_smart_scroll(&clonetextview,&format!("Cancelled.\n"));
                                    if new_need_install {
                                        c8_b_label.set_label(plugin_clone3.lock().unwrap().metadata().button_install_label);
                                        button_clone.set_css_classes(&["btn-state-install"]);
                                    } else {
                                        c8_b_label.set_label(plugin_clone3.lock().unwrap().metadata().button_remove_label);
                                        if plugin_clone3.lock().unwrap().metadata().type_ != baseplugin::base::PluginType::Oneshot{
                                            button_clone.set_css_classes(&["btn-state-remove"]);
                                        }
                                    }
                                    if inner_click_handler_count.load(Ordering::SeqCst) > 0 {
                                        let mut q = inner_lazy_fn_vec.borrow_mut();
                                        if let Some(task) = q.pop() {
                                            if let Some(f) = task.lazy_fn.borrow_mut().take() {
                                                let plugin_type = task.plugin.lock().unwrap().metadata().type_;
                                                if task.plugin.lock().unwrap().get_need_install() {
                                                    task.label.set_label(task.plugin.lock().unwrap().metadata().button_install_running_label);
                                                    task.button.set_css_classes(&["btn-state-install"]);
                                                } else {
                                                    task.label.set_label(task.plugin.lock().unwrap().metadata().button_remove_running_label);
                                                    if plugin_type != baseplugin::base::PluginType::Oneshot{
                                                        task.button.set_css_classes(&["btn-state-remove"]);
                                                    }
                                                }
                                                //task.button.set_sensitive(false);
                                                task.plugin.lock().unwrap().set_install_is_running(true);
                                                f();
                                            }
                                        }
                                    }
                                },
                                baseplugin::base::OutMesseageType::State(state) => {
                                    button_clone.set_sensitive(true);
                                    spinner_clone6.stop();
                                    if state == true {
                                        new_need_install = !new_need_install;
                                        append_text_with_smart_scroll(&clonetextview,&format!("Done.\n"));
                                        plugin_clone3.lock().unwrap().set_need_install(new_need_install);
                                        plugin_clone3.lock().unwrap().set_install_is_running(false);
                                        if new_need_install {
                                            c8_b_label.set_label(plugin_clone3.lock().unwrap().metadata().button_install_label);
                                            button_clone.set_css_classes(&["btn-state-install"]);
                                            let after_success_remove_message = plugin_clone3.lock().unwrap().metadata().after_success_remove_message;
                                            if let Some(remove_success_msg)  = after_success_remove_message {
                                                let toast = adw::Toast::new(remove_success_msg);
                                                toast.set_use_markup(true);
                                                clone4_toastoverlay.add_toast(toast);
                                                append_markup_with_smart_scroll(&clonetextview,&format!("{}\n",remove_success_msg));
                                            }
                                        } else {
                                            c8_b_label.set_label(plugin_clone3.lock().unwrap().metadata().button_remove_label);
                                            if plugin_clone3.lock().unwrap().metadata().type_ != baseplugin::base::PluginType::Oneshot{
                                                button_clone.set_css_classes(&["btn-state-remove"]);
                                            }
                                            let after_success_install_message = plugin_clone3.lock().unwrap().metadata().after_success_install_message;
                                            if let Some(install_success_msg)  = after_success_install_message {
                                                let toast = adw::Toast::new(install_success_msg);
                                                toast.set_use_markup(true);
                                                clone4_toastoverlay.add_toast(toast);
                                                append_markup_with_smart_scroll(&clonetextview,&format!("{}\n",install_success_msg));
                                            }
                                        }
                                    } else {
                                        if new_need_install {
                                            c8_b_label.set_label(plugin_clone3.lock().unwrap().metadata().button_install_label);
                                            button_clone.set_css_classes(&["btn-state-install"]);
                                        } else {
                                            c8_b_label.set_label(plugin_clone3.lock().unwrap().metadata().button_remove_label);
                                            if plugin_clone3.lock().unwrap().metadata().type_ != baseplugin::base::PluginType::Oneshot{
                                                button_clone.set_css_classes(&["btn-state-remove"]);
                                            }
                                        }
                                        plugin_clone3.lock().unwrap().set_install_is_running(false);
                                        append_text_with_smart_scroll(&clonetextview,&format!("Failed\n"));
                                    }

                                    // خروج العملية الحالية بنجاح أو فشل وسحب التالية FIFO
                                    inner_click_handler_count.fetch_sub(1, Ordering::SeqCst);
                                    if inner_click_handler_count.load(Ordering::SeqCst) > 0 {
                                        let mut q = inner_lazy_fn_vec.borrow_mut();
                                        if let Some(task) = q.pop() {
                                            if let Some(f) = task.lazy_fn.borrow_mut().take() {
                                                let plugin_type = task.plugin.lock().unwrap().metadata().type_;
                                                if task.plugin.lock().unwrap().get_need_install() {
                                                    task.label.set_label(task.plugin.lock().unwrap().metadata().button_install_running_label);
                                                    task.button.set_css_classes(&["btn-state-install"]);
                                                } else {
                                                    task.label.set_label(task.plugin.lock().unwrap().metadata().button_remove_running_label);
                                                    if plugin_type != baseplugin::base::PluginType::Oneshot{
                                                        task.button.set_css_classes(&["btn-state-remove"]);
                                                    }
                                                }
                                                //task.button.set_sensitive(false);
                                                task.plugin.lock().unwrap().set_install_is_running(true);
                                                f();
                                            }
                                        }
                                    }
                                },
                                _ => {
                                    button_clone.set_sensitive(true);
                                    spinner_clone6.stop();
                                    plugin_clone3.lock().unwrap().set_install_is_running(false);
                                    if new_need_install {
                                        c8_b_label.set_label(plugin_clone3.lock().unwrap().metadata().button_install_label);
                                        button_clone.set_css_classes(&["btn-state-install"]);
                                    } else {
                                        c8_b_label.set_label(plugin_clone3.lock().unwrap().metadata().button_remove_label);
                                        if plugin_clone3.lock().unwrap().metadata().type_ != baseplugin::base::PluginType::Oneshot{
                                            button_clone.set_css_classes(&["btn-state-remove"]);
                                        }
                                    }
                                    append_text_with_smart_scroll(&clonetextview,&format!("ERROR\n"));

                                    inner_click_handler_count.fetch_sub(1, Ordering::SeqCst);
                                    if inner_click_handler_count.load(Ordering::SeqCst) > 0 {
                                        let mut q = inner_lazy_fn_vec.borrow_mut();
                                        if let Some(task) = q.pop() {
                                            if let Some(f) = task.lazy_fn.borrow_mut().take() {
                                                let plugin_type = task.plugin.lock().unwrap().metadata().type_;
                                                if task.plugin.lock().unwrap().get_need_install() {
                                                    task.label.set_label(task.plugin.lock().unwrap().metadata().button_install_running_label);
                                                    task.button.set_css_classes(&["btn-state-install"]);
                                                } else {
                                                    task.label.set_label(task.plugin.lock().unwrap().metadata().button_remove_running_label);
                                                    if plugin_type != baseplugin::base::PluginType::Oneshot{
                                                        task.button.set_css_classes(&["btn-state-remove"]);
                                                    }
                                                }
                                                //task.button.set_sensitive(false);
                                                task.plugin.lock().unwrap().set_install_is_running(true);
                                                f();
                                            }
                                        }
                                    }
                                },
                            }    
                        }
                    });
                };

                // ج) تنفيذ قرار الـ FIFO بناءً على الفحص الجديد بعد الـ .await
                if is_queue_busy {
                    // النظام ما زال مشغولاً، غيّر نص الزر لـ Waiting وضعه في الطابور
                    c5_b_label.set_label(_plugin_clone.lock().unwrap().metadata().button_waiting_label);
                    _button_clone.set_css_classes(&["btn-state-waiting"]);
                    let c7_b_label    = c5_b_label.clone();
                    let _plugin_clone2 = Arc::clone(&_plugin_clone);
                    let task = Task {
                        button: _button_clone.clone(),
                        lazy_fn: RefCell::new(Some(Box::new(lazy_fn))),
                        plugin: _plugin_clone2,
                        label : c7_b_label.clone(),
                    };
                    clone_lazy_fn_vec.borrow_mut().insert(0, Box::new(task));
                    clone_click_handler_count.fetch_add(1, Ordering::SeqCst);
                } else {
                    // المفاجأة السارة: الطابور أصبح فارغاً أثناء تصفح المستخدم للحوار!
                    // زيادة العداد وتشغيل الدالة فوراً دون الانتظار أو تجميد النص
                    clone_click_handler_count.fetch_add(1, Ordering::SeqCst);
                    spinner_clone4.start();
                    _plugin_clone.lock().unwrap().set_install_is_running(true);
                    lazy_fn();
                }
            });
        }
    });

    btn
}

pub fn queue_create_plugin_button(
    parent: &adw::ApplicationWindow, 
    plugin: Arc<Mutex<dyn baseplugin::base::PluginTools + 'static>>,
    lazy_fn_vec: Rc<RefCell<Vec<Box<Task>>>>,
    click_handler_count: Arc<AtomicI32>,
    textview: gtk::TextView,
    spinner: gtk::Spinner,
    toastoverlay: adw::ToastOverlay,
) -> gtk::Button {

    let parent_clone   = parent.clone();
    let btn = gtk::Button::builder()
        .label("Loading...")
        .margin_end(5)
        .sensitive(false)
        .build();

    let progressbar =  gtk::ProgressBar::new();
    progressbar.set_visible(false);
    let button_progressbar_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
    button_progressbar_box.set_hexpand(false);
    button_progressbar_box.set_valign(gtk::Align::Center);
    let b_label = gtk::Label::new(Some("Loading..."));
    button_progressbar_box.append(&b_label);
    button_progressbar_box.append(&progressbar);
    btn.set_child(Some(&button_progressbar_box));

    let (tx, rx) = futures::channel::oneshot::channel(); // استخدام futures channel
    let plugin_clone1 = Arc::clone(&plugin);
    let weak_button = btn.downgrade();
    let spinner_clone1 = spinner.clone();
    spinner.start();
    btn.set_css_classes(&["btn-state-waiting"]);
    let c_b_label = b_label.clone();
    
    glib::MainContext::default().spawn_local(async move  {
        let c2_b_label = c_b_label.clone();
        if let Ok(_message) = rx.await {
            spinner_clone1.stop();
            if let Some(button) = weak_button.upgrade(){
                let need_install: bool = plugin_clone1.lock().unwrap().get_need_install();
                let pc = plugin_clone1.clone();
                drop(plugin_clone1);
                glib::idle_add_local_once(move || {
                    button.set_sensitive(true);
                    if need_install {
                        c2_b_label.set_label(&pc.lock().unwrap().metadata().button_install_label);
                        button.set_css_classes(&["btn-state-install"]);
                    }else {
                        c2_b_label.set_label(&pc.lock().unwrap().metadata().button_remove_label);
                        if pc.lock().unwrap().metadata().type_ != baseplugin::base::PluginType::Oneshot{
                            button.set_css_classes(&["btn-state-remove"]);
                        }
                    }
                });
            }
        };
    });

    let plugin_clone = Arc::clone(&plugin);
    std::thread::spawn( move || {
        let need_install: bool = plugin_clone.lock().unwrap().need_install();
        plugin_clone.lock().unwrap().set_need_install(need_install);
        let _ = tx.send("");
    });

    let _clone_click_handler_count = Arc::clone(&click_handler_count);
    let _clone_lazy_fn_vec = Rc::clone(&lazy_fn_vec);

    let active_cancellable: Rc<RefCell<Option<gio::Cancellable>>> = Rc::new(RefCell::new(None));
    let active_cancellable_clone = Rc::clone(&active_cancellable);
    let clone1_toastoverlay = toastoverlay.clone();
    let spinner_clone2 = spinner.clone();
    let progressbar_clone1 = progressbar.clone();
    let c3_b_label = b_label.clone();
    
    btn.connect_clicked(move |button| {

        let clone_click_handler_count = Arc::clone(&_clone_click_handler_count);
        let clone_lazy_fn_vec = Rc::clone(&_clone_lazy_fn_vec);

        let mut queue = clone_lazy_fn_vec.borrow_mut();
        let c4_b_label = c3_b_label.clone();
        if let Some(index) = queue.iter().position(|item| item.button == *button) {
            queue.remove(index);
            drop(queue); 

            clone_click_handler_count.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
            let need_install: bool = plugin.lock().unwrap().get_need_install();
            if need_install {
                c4_b_label.set_label(&plugin.lock().unwrap().metadata().button_install_label);
                button.set_css_classes(&["btn-state-install"]);
            } else {
                c4_b_label.set_label(&plugin.lock().unwrap().metadata().button_remove_label);
                if plugin.lock().unwrap().metadata().type_ != baseplugin::base::PluginType::Oneshot{
                    button.set_css_classes(&["btn-state-remove"]);
                }
            }
            return;
        }
        drop(queue); 

        let _button_clone   = button.clone();
        let _parent_clone   = parent_clone.clone();
        let _plugin_clone   = Arc::clone(&plugin);
        let _textview = textview.clone();
        
        let spinner_clone4 = spinner_clone2.clone();
        let clone2_toastoverlay = clone1_toastoverlay.clone();
        let progressbar_clone2 = progressbar_clone1.clone();
        let c5_b_label = c3_b_label.clone();
        
        // أخذ نسخة من المرجع الذكي لاستخدامه داخل النطاق غير المتزامن
        let active_cancellable_async = active_cancellable_clone.clone();

        glib::MainContext::default().spawn_local(async move { 
            let (is_running, yes_or_no, dialog_header, dialog_label, custom_warning) = {
                let guard    = _plugin_clone.lock().unwrap();
                let running  = guard.get_install_is_running();
                let is_need  = guard.get_need_install();
                let metadata = guard.metadata();
                
                let (h, l) = if is_need {
                    (metadata.install_yes_or_no_header.to_string(), metadata.install_yes_or_no_label.to_string())
                } else {
                    (metadata.remove_yes_or_no_header.to_string(), metadata.remove_yes_or_no_label.to_string())
                };

                let warning = metadata.custom_cancel_warning_message.as_ref().map(|w| (w[0].to_string(), w[1].to_string()));
                
                (running, metadata.yes_or_no, h, l, warning)
            }; 

            if is_running {
                // استدعاء العملية الحالية بشكل صحيح من active_cancellable_async
                if let Some(current_cancellable) = active_cancellable_async.borrow().clone() {
                    let yes_or_no_dialog = if let Some(w) = custom_warning {
                        adw::AlertDialog::new(Some(w.0.as_str()), Some(w.1.as_str()))
                    } else {
                        adw::AlertDialog::new(Some("Warning: Potential System Corruption"), Some("Warning: Canceling this action while it is running poses a high risk to your system's integrity. It may result in broken components or data corruption.\nAre you sure you want to abort?"))
                    };

                    yes_or_no_dialog.add_response("No", "Keep Running");
                    yes_or_no_dialog.add_response("Yes", "Cancel Anyway");
                    yes_or_no_dialog.set_default_response(Some("No"));
                    yes_or_no_dialog.set_response_appearance("Yes",adw::ResponseAppearance::Destructive);
                    yes_or_no_dialog.set_body_use_markup(true);
                    yes_or_no_dialog.set_heading_use_markup(true);
                    
                    let response = yes_or_no_dialog.choose_future(Some(&_parent_clone)).await;

                    if response == "Yes" {
                        current_cancellable.cancel(); // الآن سيقوم بإلغاء العملية الأصلية
                        spinner_clone4.stop();
                    }
                }
                return; // الخروج مباشرة دون إنشاء Cancellable جديد
            }

            if yes_or_no {
                let yes_or_no_dialog = adw::AlertDialog::new(Some(dialog_header.as_str()), Some(dialog_label.as_str()));
                yes_or_no_dialog.add_response("No", "No");
                yes_or_no_dialog.add_response("Yes", "Yes");
                yes_or_no_dialog.set_default_response(Some("Yes"));
                yes_or_no_dialog.set_response_appearance("Yes",adw::ResponseAppearance::Suggested);
                yes_or_no_dialog.set_body_use_markup(true);
                yes_or_no_dialog.set_heading_use_markup(true);
                
                let response = yes_or_no_dialog.choose_future(Some(&_parent_clone)).await;

                if response == "No" {
                    _button_clone.set_sensitive(true);
                    return;
                }
            }

            // 🟢 التعديل الجوهري: إنشاء الـ Cancellable الجديد هنا فقط، بعد التأكد من عدم وجود عملية قيد التشغيل
            let new_cancellable = gio::Cancellable::new();
            *active_cancellable_async.borrow_mut() = Some(new_cancellable.clone());
            let clone_cancellable = new_cancellable.clone();

            let current_active_tasks = clone_click_handler_count.load(std::sync::atomic::Ordering::SeqCst);
            let is_queue_busy = current_active_tasks > 0;

            let button_clone  = _button_clone.clone();
            let plugin_clone3 = Arc::clone(&_plugin_clone);
            let plugin_clone2 = Arc::clone(&_plugin_clone);
            let run_lazy_fn_vec = Rc::clone(&clone_lazy_fn_vec);
            let run_click_handler_count = Arc::clone(&clone_click_handler_count);
            let clonetextview = _textview.clone();
            
            // تمرير الـ clone_cancellable لـ lazy_fn
            let clone2_cancellable = clone_cancellable.clone(); 
            
            let spinner_clone5 = spinner_clone4.clone();
            let clone3_toastoverlay = clone2_toastoverlay.clone();
            let progressbar_clone4 = progressbar_clone2.clone();
            let c6_b_label    = c5_b_label.clone();
            
            let lazy_fn = move || {
                spinner_clone5.start();
                let need_install: bool = plugin_clone3.lock().unwrap().get_need_install();
                if need_install {
                    c6_b_label.set_label(&plugin_clone3.lock().unwrap().metadata().button_install_running_label);
                } else {
                    c6_b_label.set_label(&plugin_clone3.lock().unwrap().metadata().button_remove_running_label);
                }

                use futures::StreamExt;
                let (tx, mut rx) = futures::channel::mpsc::unbounded::<baseplugin::base::OutMesseageType>();
                let clone_tx = tx.clone();
                let i_clone1_tx = tx.clone();
                let i_clone1_cancellable = clone2_cancellable.clone();
                let files_to_dowmload_count: usize = plugin_clone2.lock().unwrap().get_downlods_files_length();
                
                if plugin_clone2.lock().unwrap().get_need_install() {
                    if files_to_dowmload_count > 0 {
                        progressbar_clone4.set_visible(true);
                        plugin_clone2.lock().unwrap().download_files(clone_tx, clone2_cancellable);
                    }else {
                        plugin_clone2.lock().unwrap().install(clone_tx, clone2_cancellable,None);
                    }
                } else {
                    plugin_clone2.lock().unwrap().remove(clone_tx,clone2_cancellable);
                }

                let inner_lazy_fn_vec = Rc::clone(&run_lazy_fn_vec);
                let inner_click_handler_count = Arc::clone(&run_click_handler_count);
                let spinner_clone6 = spinner_clone5.clone();
                let clone4_toastoverlay = clone3_toastoverlay.clone();
                let progressbar_clone5 = progressbar_clone4.clone();
                let c7_b_label = c6_b_label.clone();
                
                glib::MainContext::default().spawn_local(async move {
                    let i_clone2_tx = i_clone1_tx.clone();
                    let i_clone2_cancellable = i_clone1_cancellable.clone();
                    let progressbar_clone6 = progressbar_clone5.clone();
                    let c8_b_label = c7_b_label.clone();
                    
                    while let Some(message) = rx.next().await { 
                        let mut new_need_install: bool = plugin_clone3.lock().unwrap().get_need_install();
                        let original_need_install: bool = plugin_clone3.lock().unwrap().get_need_install();
                        
                        // 🟢 دالة مساعدة لتفريغ الطابور بأمان ومنع انهيار الـ RefCell
                        let mut pending_task_fn = None;
                        let pop_queue = || -> Option<Box<dyn FnOnce()>> {
                            if inner_click_handler_count.load(std::sync::atomic::Ordering::SeqCst) > 0 {
                                let mut q = inner_lazy_fn_vec.borrow_mut();
                                if let Some(task) = q.pop() {
                                    if let Some(f) = task.lazy_fn.borrow_mut().take() {
                                        let plugin_type = task.plugin.lock().unwrap().metadata().type_;
                                        if task.plugin.lock().unwrap().get_need_install() {
                                            task.label.set_label(&task.plugin.lock().unwrap().metadata().button_install_running_label);
                                            task.button.set_css_classes(&["btn-state-install"]);
                                        } else {
                                            task.label.set_label(&task.plugin.lock().unwrap().metadata().button_remove_running_label);
                                            if plugin_type != baseplugin::base::PluginType::Oneshot{
                                                task.button.set_css_classes(&["btn-state-remove"]);
                                            }
                                        }
                                        task.plugin.lock().unwrap().set_install_is_running(true);
                                        return Some(f);
                                    }
                                }
                            }
                            None
                        };

                        match message {
                            baseplugin::base::OutMesseageType::Progress(downloadfractioninfo) => {
                                let percent = (downloadfractioninfo.fraction * 100.0) as i32;
                                c8_b_label.set_label(&format!("Download({}/{}) {}%",downloadfractioninfo.filenumber,downloadfractioninfo.countfiles, percent));
                                progressbar_clone6.set_fraction(downloadfractioninfo.fraction);
                            },
                            baseplugin::base::OutMesseageType::DownloadState(files_location) => {
                                match files_location {
                                    Some(f_location) =>  {  
                                        progressbar_clone6.set_visible(false);
                                        progressbar_clone6.set_fraction(0.0);
                                        append_text_with_smart_scroll(&clonetextview,"Download Done.\n");
                                        if original_need_install {
                                            c8_b_label.set_label(&plugin_clone3.lock().unwrap().metadata().button_install_running_label);
                                        } else {
                                            c8_b_label.set_label(&plugin_clone3.lock().unwrap().metadata().button_remove_running_label);
                                        }
                                        plugin_clone3.lock().unwrap().install(i_clone2_tx.clone(),i_clone2_cancellable.clone(),Some(f_location));
                                    },
                                    _ => {
                                        inner_click_handler_count.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
                                        append_text_with_smart_scroll(&clonetextview,"Download Failed\n");
                                        plugin_clone3.lock().unwrap().set_install_is_running(false);
                                        spinner_clone6.stop();
                                        button_clone.set_sensitive(true);
                                        if original_need_install {
                                            c8_b_label.set_label(&plugin_clone3.lock().unwrap().metadata().button_install_label);
                                        } else {
                                            c8_b_label.set_label(&plugin_clone3.lock().unwrap().metadata().button_remove_label);
                                        }
                                        pending_task_fn = pop_queue();
                                    }
                                }
                            },
                            baseplugin::base::OutMesseageType::DownloadCancelled => {
                                progressbar_clone6.set_fraction(0.0);
                                progressbar_clone6.set_visible(false);
                                plugin_clone3.lock().unwrap().set_install_is_running(false);
                                spinner_clone6.stop();
                                inner_click_handler_count.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
                                append_text_with_smart_scroll(&clonetextview,"Download Cancelled.\n");
                                if original_need_install {
                                    c8_b_label.set_label(&plugin_clone3.lock().unwrap().metadata().button_install_label);
                                    button_clone.set_css_classes(&["btn-state-install"]);
                                } else {
                                    c8_b_label.set_label(&plugin_clone3.lock().unwrap().metadata().button_remove_label);
                                    if plugin_clone3.lock().unwrap().metadata().type_ != baseplugin::base::PluginType::Oneshot{
                                        button_clone.set_css_classes(&["btn-state-remove"]);
                                    }
                                }
                                pending_task_fn = pop_queue();
                            },
                            baseplugin::base::OutMesseageType::DownloadError => {
                                progressbar_clone6.set_fraction(0.0);
                                progressbar_clone6.set_visible(false);
                                button_clone.set_sensitive(true);
                                spinner_clone6.stop();
                                plugin_clone3.lock().unwrap().set_install_is_running(false);
                                if original_need_install {
                                    c8_b_label.set_label(&plugin_clone3.lock().unwrap().metadata().button_install_label);
                                    button_clone.set_css_classes(&["btn-state-install"]);
                                } else {
                                    c8_b_label.set_label(&plugin_clone3.lock().unwrap().metadata().button_remove_label);
                                    if plugin_clone3.lock().unwrap().metadata().type_ != baseplugin::base::PluginType::Oneshot{
                                        button_clone.set_css_classes(&["btn-state-remove"]);
                                    }
                                }
                                append_text_with_smart_scroll(&clonetextview,"Download ERROR\n");
                                inner_click_handler_count.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
                                pending_task_fn = pop_queue();
                            },
                            baseplugin::base::OutMesseageType::Message(msg) => {
                                append_text_with_smart_scroll(&clonetextview,&format!("{}\n", msg));
                            },
                            baseplugin::base::OutMesseageType::Cancelled => {
                                plugin_clone3.lock().unwrap().set_install_is_running(false);
                                spinner_clone6.stop();
                                inner_click_handler_count.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
                                append_text_with_smart_scroll(&clonetextview,"Cancelled.\n");
                                if new_need_install {
                                    c8_b_label.set_label(&plugin_clone3.lock().unwrap().metadata().button_install_label);
                                    button_clone.set_css_classes(&["btn-state-install"]);
                                } else {
                                    c8_b_label.set_label(&plugin_clone3.lock().unwrap().metadata().button_remove_label);
                                    if plugin_clone3.lock().unwrap().metadata().type_ != baseplugin::base::PluginType::Oneshot{
                                        button_clone.set_css_classes(&["btn-state-remove"]);
                                    }
                                }
                                pending_task_fn = pop_queue();
                            },
                            baseplugin::base::OutMesseageType::State(state) => {
                                button_clone.set_sensitive(true);
                                spinner_clone6.stop();
                                if state == true {
                                    new_need_install = !new_need_install;
                                    append_text_with_smart_scroll(&clonetextview,"Done.\n");
                                    plugin_clone3.lock().unwrap().set_need_install(new_need_install);
                                    plugin_clone3.lock().unwrap().set_install_is_running(false);
                                    
                                    if new_need_install {
                                        c8_b_label.set_label(&plugin_clone3.lock().unwrap().metadata().button_install_label);
                                        button_clone.set_css_classes(&["btn-state-install"]);
                                        
                                        // عزل القراءة لمنع بقاء القفل نشطاً
                                        let msg_opt = plugin_clone3.lock().unwrap().metadata().after_success_remove_message.clone();
                                        if let Some(remove_success_msg)  = msg_opt {
                                            let toast = adw::Toast::new(&remove_success_msg);
                                            toast.set_use_markup(true);
                                            clone4_toastoverlay.add_toast(toast);
                                            append_markup_with_smart_scroll(&clonetextview,&format!("{}\n",remove_success_msg));
                                        }
                                    } else {
                                        c8_b_label.set_label(&plugin_clone3.lock().unwrap().metadata().button_remove_label);
                                        if plugin_clone3.lock().unwrap().metadata().type_ != baseplugin::base::PluginType::Oneshot{
                                            button_clone.set_css_classes(&["btn-state-remove"]);
                                        }
                                        
                                        let msg_opt = plugin_clone3.lock().unwrap().metadata().after_success_install_message.clone();
                                        if let Some(install_success_msg)  = msg_opt {
                                            let toast = adw::Toast::new(&install_success_msg);
                                            toast.set_use_markup(true);
                                            clone4_toastoverlay.add_toast(toast);
                                            append_markup_with_smart_scroll(&clonetextview,&format!("{}\n",install_success_msg));
                                        }
                                    }
                                } else {
                                    if new_need_install {
                                        c8_b_label.set_label(&plugin_clone3.lock().unwrap().metadata().button_install_label);
                                        button_clone.set_css_classes(&["btn-state-install"]);
                                    } else {
                                        c8_b_label.set_label(&plugin_clone3.lock().unwrap().metadata().button_remove_label);
                                        if plugin_clone3.lock().unwrap().metadata().type_ != baseplugin::base::PluginType::Oneshot{
                                            button_clone.set_css_classes(&["btn-state-remove"]);
                                        }
                                    }
                                    plugin_clone3.lock().unwrap().set_install_is_running(false);
                                    append_text_with_smart_scroll(&clonetextview,"Failed\n");
                                }

                                inner_click_handler_count.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
                                pending_task_fn = pop_queue();
                            },
                            _ => {
                                button_clone.set_sensitive(true);
                                spinner_clone6.stop();
                                plugin_clone3.lock().unwrap().set_install_is_running(false);
                                if new_need_install {
                                    c8_b_label.set_label(&plugin_clone3.lock().unwrap().metadata().button_install_label);
                                    button_clone.set_css_classes(&["btn-state-install"]);
                                } else {
                                    c8_b_label.set_label(&plugin_clone3.lock().unwrap().metadata().button_remove_label);
                                    if plugin_clone3.lock().unwrap().metadata().type_ != baseplugin::base::PluginType::Oneshot{
                                        button_clone.set_css_classes(&["btn-state-remove"]);
                                    }
                                }
                                append_text_with_smart_scroll(&clonetextview,"ERROR\n");

                                inner_click_handler_count.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
                                pending_task_fn = pop_queue();
                            }
                        }
                        
                        // 🟢 تنفيذ الدالة المعلقة هنا في النهاية فقط بعد التأكد من إغلاق (Drop) طابور RefMut تماماً
                        if let Some(f) = pending_task_fn {
                            f();
                        }
                    }
                });
            };

            if is_queue_busy {
                c5_b_label.set_label(&_plugin_clone.lock().unwrap().metadata().button_waiting_label);
                _button_clone.set_css_classes(&["btn-state-waiting"]);
                let c7_b_label    = c5_b_label.clone();
                let _plugin_clone2 = Arc::clone(&_plugin_clone);
                let task = Task { 
                    button: _button_clone.clone(),
                    lazy_fn: RefCell::new(Some(Box::new(lazy_fn))),
                    plugin: _plugin_clone2,
                    label : c7_b_label.clone(),
                };
                clone_lazy_fn_vec.borrow_mut().insert(0, Box::new(task));
                clone_click_handler_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            } else {
                clone_click_handler_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                spinner_clone4.start();
                _plugin_clone.lock().unwrap().set_install_is_running(true);
                lazy_fn();
            }
        });
    });

    btn
}
