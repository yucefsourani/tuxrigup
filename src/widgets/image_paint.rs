use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::gdk; 
use std::cell::RefCell;

mod imp {
    use super::*;

    #[derive(Default)]
    pub struct ImagePaint {
        pub width: RefCell<f64>,
        pub height: RefCell<f64>,
        // 1. تغيير النوع إلى Paintable لكي يستوعب Texture و IconPaintable معاً
        pub paintable: RefCell<Option<gdk::Paintable>>,
    }

    #[super::glib::object_subclass]
    impl ObjectSubclass for ImagePaint {
        const NAME: &'static str = "ImagePaint";
        type Type = super::ImagePaint;
        type ParentType = gtk::Widget;
    }

    impl ObjectImpl for ImagePaint {}

    impl BuildableImpl for ImagePaint {}

    impl WidgetImpl for ImagePaint {
        fn snapshot(&self, snapshot: &gtk::Snapshot) {
            // 2. تحديث دالة الرسم
            if let Some(ref paintable) = *self.paintable.borrow() {
                let w = *self.width.borrow();
                let h = *self.height.borrow();
                
                // دالة snapshot الخاصة بالـ Paintable ترسم نفسها مباشرة، 
                // مما يغنينا عن استخدام graphene::Rect و append_texture
                paintable.snapshot(snapshot, w, h);
            }
        }

        fn request_mode(&self) -> gtk::SizeRequestMode {
            gtk::SizeRequestMode::ConstantSize
        }

        fn measure(&self, orientation: gtk::Orientation, _for_size: i32) -> (i32, i32, i32, i32) {
            let w = *self.width.borrow() as i32;
            let h = *self.height.borrow() as i32;

            if orientation == gtk::Orientation::Horizontal {
                (w, w, -1, -1)
            } else {
                (h, h, -1, -1)
            }
        }
    }
}

glib::wrapper! {
    pub struct ImagePaint(ObjectSubclass<imp::ImagePaint>)
        @extends gtk::Widget, 
        @implements gtk::Buildable, gtk::Accessible, gtk::ConstraintTarget;
}

impl ImagePaint {
    pub fn new<P: AsRef<std::path::Path>>(image_location: P, width: f64, height: f64, icon_name: Option<String>) -> Self {
        let widget: Self = glib::Object::builder().build();
        let imp = widget.imp();

        *imp.width.borrow_mut() = width;
        *imp.height.borrow_mut() = height;

        if let Some(icon_n) = icon_name {
            // 3. الطريقة الصحيحة لجلب أيقونة من النظام
            if let Some(display) = gdk::Display::default() {
                // جلب سمة الأيقونات الافتراضية للنظام
                let icon_theme = gtk::IconTheme::for_display(&display);
                
                // البحث عن الأيقونة (ترجع IconPaintable)
                let icon_paintable = icon_theme.lookup_icon(
                    &icon_n,
                    &[],
                    width as i32, // الحجم المستهدف للأيقونة
                    1,            // المقياس (Scale)
                    gtk::TextDirection::None,
                    gtk::IconLookupFlags::NONE,
                );
                
                // تحويل IconPaintable إلى Paintable وتخزينه
                *imp.paintable.borrow_mut() = Some(icon_paintable.upcast::<gdk::Paintable>());
            } else {
                eprintln!("لا يمكن الوصول إلى شاشة العرض لجلب الأيقونات.");
            }
        } else {
            // 4. حالة وجود مسار ملف
            if let Ok(texture) = gdk::Texture::from_filename(&image_location) {
                // تحويل Texture إلى Paintable وتخزينه
                *imp.paintable.borrow_mut() = Some(texture.upcast::<gdk::Paintable>());
            } else {
                eprintln!("فشل تحميل الصورة من المسار: {}", image_location.as_ref().display());
            }
        }
        
        widget
    }
}
