project(
  'tuxrigup',
  'rust',
  version: '0.1.0',
  license: 'GPL-3.0-or-later',
  meson_version: '>= 0.59.0'
)

gnome = import('gnome')
i18n = import('i18n')

prefix = get_option('prefix')
bindir = join_paths(prefix, get_option('bindir'))
datadir = join_paths(prefix, get_option('datadir'))

# تفعيل بناء المجلدات الفرعية
subdir('src')
subdir('data')

# نقل مجلد images إلى prefix/share/tuxrigup
install_subdir(
  'images',
  install_dir: datadir / 'tuxrigup'
)

# تحديث قواعد البيانات بعد التثبيت
gnome.post_install(
  glib_compile_schemas: true,
  gtk_update_icon_cache: true,
  update_desktop_database: true
)
