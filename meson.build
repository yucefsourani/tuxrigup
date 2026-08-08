project(
  'tuxrigup',
  'rust',
  version: '1.50.1',
  license: 'GPL-3.0-or-later',
  meson_version: '>= 0.59.0'
)

gnome = import('gnome')
i18n = import('i18n')

prefix = get_option('prefix')
bindir = join_paths(prefix, get_option('bindir'))
datadir = join_paths(prefix, get_option('datadir'))


subdir('src')
subdir('data')


install_subdir(
  'images',
  install_dir: datadir / 'tuxrigup'
)


gnome.post_install(
  glib_compile_schemas: true,
  gtk_update_icon_cache: true,
  update_desktop_database: true
)
