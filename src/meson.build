cargo = find_program('cargo', required: true)

cargo_build_dir = meson.project_build_root() / 'src'

# بناء المشروع عبر Cargo وتغيير اسم المخرج النهائي إلى tuxrigup
custom_target(
  'cargo-build',
  build_by_default: true,
  build_always_stale: true,
  output: 'tuxrigup',
  console: true,
  install: true,
  install_dir: bindir,
  command: [
    'sh', '-c',
    'cargo build --release --manifest-path @0@ --target-dir @1@ && cp @1@/release/gtk4_ex @2@'.format(
      meson.project_source_root() / 'Cargo.toml',
      cargo_build_dir,
      '@OUTPUT@'
    )
  ]
)
