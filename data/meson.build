
install_data(
  'com.github.yucefsourani.tuxrigup.desktop',
  install_dir: datadir / 'applications'
)

install_data(
  'com.github.yucefsourani.tuxrigup.appdata.xml',
  install_dir: datadir / 'metainfo'
)

install_data(
  'com.github.yucefsourani.tuxrigup.gschema.xml',
  install_dir: datadir / 'glib-2.0' / 'schemas'
)


icon_sizes = ['8', '16', '20', '22', '24', '32', '36', '40', '42', '48', '64', '72', '96', '128', '192', '256', '512']
foreach i : icon_sizes
  install_data(
    'icons/hicolor' / i + 'x' + i / 'apps/com.github.yucefsourani.tuxrigup.png',
    install_dir: datadir / 'icons/hicolor' / i + 'x' + i / 'apps'
  )
endforeach

install_data(
  'icons/hicolor/com.github.yucefsourani.tuxrigup.png',
  install_dir: datadir / 'pixmaps'
)
