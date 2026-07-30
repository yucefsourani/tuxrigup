use adw::prelude::*;
use std::thread;
use std::time::Duration;
use futures::channel; // استخدام قناة الطلقة الواحدة
use futures::StreamExt;
mod plugins;
mod baseplugin;
mod utils;
use std::sync::{Arc,Mutex};
use std::sync::atomic::{AtomicI32, Ordering};
use gtk::prelude::*;
use std::cell::RefCell;

pub fn create_plugin_button(
    parent: &adw::ApplicationWindow, 
    plugin: Arc<Mutex<dyn baseplugin::base::PluginTools + 'static>>
) -> gtk::Button {
    
    let _weak_parent   = parent.downgrade();
    let btn = gtk::Button::builder()
        .label("Loading...")
        .margin_end(5)
        .sensitive(false)
        .build();
    
    let (tx, rx) = channel::oneshot::channel();
    let plugin_clone1 = Arc::clone(&plugin);
    let weak_button = btn.downgrade();
    glib::MainContext::default().spawn_local(async move  {
        if let Ok(_message) = rx.await {
            if let Some(button) = weak_button.upgrade(){
                let need_install: bool = plugin_clone1.lock().unwrap().get_need_install();
                drop(plugin_clone1);
                glib::idle_add_local_once(move || {
                    button.set_sensitive(true);
                    if need_install {
                        button.set_label("Install");
                    }else {
                        button.set_label("Remove");
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
    
    btn.connect_clicked(move |button| {
        
        let button_clone  = button.clone();
        button.set_sensitive(false);
        button.set_label("Running...");
        
        let plugin_clone3 = Arc::clone(&plugin);
        let need_install: bool = plugin_clone3.lock().unwrap().get_need_install();
        if need_install {
            button.set_label("Install Running...");
        }else{
            button.set_label("Remove Running...");
        }
        
        let (tx, mut rx) = futures::channel::mpsc::unbounded::<baseplugin::base::OutMesseageType>();
        let plugin_clone2 = Arc::clone(&plugin);
        let clone_tx = tx.clone();
        if plugin_clone2.lock().unwrap().get_need_install() {
            plugin_clone2.lock().unwrap().install(clone_tx);
        }else {
            plugin_clone2.lock().unwrap().remove(clone_tx);
        }
        
        glib::MainContext::default().spawn_local(async move {
            while let Some(message) = rx.next().await { 
                let mut new_need_install:bool  =  plugin_clone3.lock().unwrap().get_need_install();
                match message {
                        baseplugin::base::OutMesseageType::Message(msg) => {
                            println!("***{}***",msg);
                        },
                        baseplugin::base::OutMesseageType::State(state) => {
                            button_clone.set_sensitive(true);
                            if state == true {
                                new_need_install = !new_need_install;
                                plugin_clone3.lock().unwrap().set_need_install(new_need_install);
                                if new_need_install {
                                    button_clone.set_label("Install");
                                }else {
                                    button_clone.set_label("Remove");
                                }
                            }else {
                                button_clone.set_sensitive(true);
                                if new_need_install {
                                    button_clone.set_label("Install");
                                }else {
                                    button_clone.set_label("Remove");
                                }
                                println!("***Faild***");
                            }
                        },
                        _ => {
                            button_clone.set_sensitive(true);
                            if new_need_install {
                                button_clone.set_label("Install");
                            }else {
                                button_clone.set_label("Remove");
                            }
                            println!("***ERROR***",);
                        },
                    }    
                }
            });
                
        });

        
    btn
}

struct Task {
    lazy_fn: RefCell<Option<Box<dyn FnOnce()>>>,
    button: gtk::Button,
    plugin: Arc<Mutex<dyn baseplugin::base::PluginTools + 'static>>,
    }
    
pub fn fifo_create_plugin_button(
    parent: &adw::ApplicationWindow, 
    plugin: Arc<Mutex<dyn baseplugin::base::PluginTools + 'static>>,
    lazy_fn_vec: Arc<Mutex<Vec<Box<Task>>>>,
    click_handler_count: Arc<AtomicI32>,
) -> gtk::Button {
    
    let _weak_parent   = parent.downgrade();
    let btn = gtk::Button::builder()
        .label("Loading...")
        .margin_end(5)
        .sensitive(false)
        .build();
    
    let (tx, rx) = channel::oneshot::channel();
    let plugin_clone1 = Arc::clone(&plugin);
    let weak_button = btn.downgrade();
    glib::MainContext::default().spawn_local(async move  {
        if let Ok(_message) = rx.await {
            if let Some(button) = weak_button.upgrade(){
                let need_install: bool = plugin_clone1.lock().unwrap().get_need_install();
                drop(plugin_clone1);
                glib::idle_add_local_once(move || {
                    button.set_sensitive(true);
                    if need_install {
                        button.set_label("Install");
                    }else {
                        button.set_label("Remove");
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
    
    let clone_click_handler_count = Arc::clone(&click_handler_count);
    btn.connect_clicked(move |button| {
        let mut queue = lazy_fn_vec.lock().unwrap();
            
        if let Some(index) = queue.iter().position(|item| item.button == *button) {
            queue.remove(index);
            drop(queue); 
            
            clone_click_handler_count.fetch_sub(1, Ordering::SeqCst);
            
            // جلب حالة الـ plugin لتسمية الزر بشكل صحيح بعد التراجع
            let need_install: bool = plugin.lock().unwrap().get_need_install();
            if need_install {
                button.set_label("Install"); // عند التراجع نعيد الزر مستعداً للتثبيت
            } else {
                button.set_label("Remove");  // عند التراجع نعيد الزر مستعداً للإزالة
            }
            
            return;
        }
        
        drop(queue);
        
        let button_clone  = button.clone();
        let plugin_clone3 = Arc::clone(&plugin);
        let plugin_clone2 = Arc::clone(&plugin);
        let clone_lazy_fn_vec = Arc::clone(&lazy_fn_vec);
        let clone2_lazy_fn_vec = Arc::clone(&clone_lazy_fn_vec);
        let clone2_click_handler_count = Arc::clone(&clone_click_handler_count);

        

        
        let lazy_fn = move || {
            button_clone.set_sensitive(false);
            button_clone.set_label("Running...");
            
            let need_install: bool = plugin_clone3.lock().unwrap().get_need_install();
            if need_install {
                button_clone.set_label("Install Running...");
            }else{
                button_clone.set_label("Remove Running...");
            }
            
            let (tx, mut rx) = futures::channel::mpsc::unbounded::<baseplugin::base::OutMesseageType>();
            let clone_tx = tx.clone();
            if plugin_clone2.lock().unwrap().get_need_install() {
                plugin_clone2.lock().unwrap().install(clone_tx);
            }else {
                plugin_clone2.lock().unwrap().remove(clone_tx);
            }
            
            glib::MainContext::default().spawn_local(async move {
                while let Some(message) = rx.next().await { 
                    let mut new_need_install:bool  =  plugin_clone3.lock().unwrap().get_need_install();
                    match message {
                            baseplugin::base::OutMesseageType::Message(msg) => {
                                println!("***{}***",msg);
                            },
                            baseplugin::base::OutMesseageType::State(state) => {
                                button_clone.set_sensitive(true);
                                if state == true {
                                    new_need_install = !new_need_install;
                                    plugin_clone3.lock().unwrap().set_need_install(new_need_install);
                                    if new_need_install {
                                        button_clone.set_label("Install");
                                    }else {
                                        button_clone.set_label("Remove");
                                    }
                                }else {
                                    button_clone.set_sensitive(true);
                                    if new_need_install {
                                        button_clone.set_label("Install");
                                    }else {
                                        button_clone.set_label("Remove");
                                    }
                                    println!("***Faild***");
                                }
                                clone2_click_handler_count.fetch_sub(1, Ordering::SeqCst);
                                if clone2_click_handler_count.load(Ordering::SeqCst) > 0{
                                    if let Some(task)  = clone2_lazy_fn_vec.lock().unwrap().pop() {
                                        if let Some(f) = task.lazy_fn.borrow_mut().take() {
                                            
                                            if task.plugin.lock().unwrap().get_need_install() {
                                                task.button.set_label("Install Running");
                                            }else {
                                                 task.button.set_label("Remove Running");
                                            }
                                            task.button.set_sensitive(false);
                                            clone2_click_handler_count.fetch_add(1, Ordering::SeqCst);
                                            f();
                                        }
                                    }
                                }
                                
                            },
                            _ => {
                                button_clone.set_sensitive(true);
                                if new_need_install {
                                    button_clone.set_label("Install");
                                }else {
                                    button_clone.set_label("Remove");
                                }
                                println!("***ERROR***",);
                                
                                clone2_click_handler_count.fetch_sub(1, Ordering::SeqCst);
                                if clone2_click_handler_count.load(Ordering::SeqCst) > 0{
                                    if let Some(task)  = clone2_lazy_fn_vec.lock().unwrap().pop() {
                                        if let Some(f) = task.lazy_fn.borrow_mut().take() {
                                            
                                            if task.plugin.lock().unwrap().get_need_install() {
                                                task.button.set_label("Install Running");
                                            }else {
                                                 task.button.set_label("Remove Running");
                                            }
                                            task.button.set_sensitive(false);
                                            clone2_click_handler_count.fetch_add(1, Ordering::SeqCst);
                                            f();
                                        }
                                    }
                                }
                                
                            },
                        }    
                    }
                });
                    
            };
            
        if clone_click_handler_count.load(Ordering::SeqCst) > 0 {
            //button.set_sensitive(false);
            let _plugin_clone = Arc::clone(&plugin);
            button.set_label("Waiting...");
            let task = Task {
                button: button.clone(),
                lazy_fn: RefCell::new(Some(Box::new(lazy_fn))),
                plugin: _plugin_clone,
                };
            clone_lazy_fn_vec.lock().unwrap().insert(0,Box::new(task));
            clone_click_handler_count.fetch_add(1, Ordering::SeqCst);
            return ;
        }else {
            clone_click_handler_count.fetch_add(1, Ordering::SeqCst);
            lazy_fn();
            }
        });


        
    btn
}

pub fn get_all_plugins() -> Vec<Box<dyn baseplugin::base::PluginTools>> {
    vec![
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        Box::new(plugins::firefox::FirefoxPlugin::create()),
        // Box::new(plugins::chrome::ChromePlugin::create()), 
    ]
}

fn main() {
    let app = adw::Application::builder().application_id("com.github.yucefmsourani.gtk4_ex").build();

    
    app.connect_activate(|app| {
        let  all_plugin = get_all_plugins();
        let mainwindow = adw::ApplicationWindow::builder().application(app).title("Gtk4 Example").build();
        let mainvbox = gtk::Box::new(gtk::Orientation::Vertical, 10);
        mainwindow.set_content(Some(&mainvbox));

        let headerbar = gtk::HeaderBar::new();
        mainvbox.append(&headerbar);
        
        let sw = gtk::ScrolledWindow::new();
        mainvbox.append(&sw);
        
        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
        vbox.set_vexpand(true);
        sw.set_child(Some(&vbox));
        
        let run_button = gtk::Button::builder().label("Run Sync Task").build();
        mainvbox.append(&run_button);
        
        let mainwindow_clone = mainwindow.clone();
        let  lazy_fn_vec: Arc<Mutex<Vec<Box<Task >>>> = Arc::new(Mutex::new(Vec::new()));
        let  click_handler_count = Arc::new(AtomicI32::new(0));
        for plugin in all_plugin.into_iter() {
            let plugin_arc: Arc<Mutex<dyn baseplugin::base::PluginTools>> = Arc::new(Mutex::new(plugin));
            //let b = create_plugin_button(&mainwindow,plugin_arc);
            let  clone_click_handler_count = Arc::clone(&click_handler_count);
            let b = fifo_create_plugin_button(&mainwindow,plugin_arc,Arc::clone(&lazy_fn_vec),clone_click_handler_count);
            vbox.append(&b);
        }
        run_button.connect_clicked(move |button| {
            button.set_label("جاري المعالجة المعقدة...");
            button.set_sensitive(false);

            //let button_clone = button.clone();
            //let window_clone = mainwindow_clone.clone();
            let weak_button = button.downgrade();
            let weak_window = mainwindow_clone.downgrade();
            // 1. إنشاء قناة oneshot (مخصصة لإرسال قيمة واحدة فقط)
            let (tx, rx) = channel::oneshot::channel();

            // 2. إرسال المهمة الثقيلة (التي لا تدعم await) إلى خيط خلفي
            thread::spawn(move || {
                // محاكاة كود تزامني ثقيل (Blocking Code) كضغط ملفات أو معالجة داتا
                thread::sleep(Duration::from_secs(5));
                
                // بعد انتهاء العمل الشاق، نرسل النتيجة
                let _ = tx.send("تم إنجاز المهمة الثقيلة بنجاح!");
            });

            // 3. السحر هنا: نعود للخيط الرئيسي وننتظر النتيجة بشكل Async
            glib::MainContext::default().spawn_local(async move {
                        // الانتظار بشكل غير متزامن
                        //glib::timeout_future(Duration::from_secs(10)).await;
                        // محاولة ترقية المراجع (Upgrade) للتأكد من أن المستخدم لم يغلق التطبيق أثناء الانتظار
                        if let Ok(result_message) = rx.await {
                            if let (Some(b), Some(w)) = (weak_button.upgrade(), weak_window.upgrade()) {
                                b.set_label("Done");
                                b.set_sensitive(true);
                                w.set_title(Some(result_message));
                            } else {
                                println!("تم إلغاء التحديث لأن النافذة أُغلقت أثناء فترة الانتظار.");
                            }
                    }
                });
        });
        
        let quit_button = gtk::Button::builder().label("Quit").build();
        mainvbox.append(&quit_button);

        let mainwindow_weak = mainwindow.downgrade();
        quit_button.connect_clicked(move |_| {
            if let Some(w) = mainwindow_weak.upgrade() {
                w.close();
                }
            });

        mainwindow.present();
    });
    app.run();
}
