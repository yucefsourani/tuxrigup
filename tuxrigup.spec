Name:           tuxrigup
Version:        1.50
Release:        1%{?dist}
Summary:        The Essential Post-Setup & Workstation Tuning Suite for Linux

License:        GPL-3.0-or-later
URL:            https://github.com/yucefsourani/tuxrigup
Source0:        %{url}/archive/refs/tags/v%{version}.tar.gz

BuildRequires:  meson
BuildRequires:  gcc
BuildRequires:  cargo
BuildRequires:  rust
BuildRequires:  gettext
BuildRequires:  pkgconfig(openssl)
BuildRequires:  pkgconfig(gtk4)
BuildRequires:  pkgconfig(libadwaita-1)
BuildRequires:  pkgconfig(glib-2.0)
BuildRequires:  pkgconfig(gio-2.0)
BuildRequires:  desktop-file-utils
BuildRequires:  libappstream-glib

Requires:       hicolor-icon-theme
Requires:       gtk4
Requires:       libadwaita

%description
The Essential Post-Setup & Workstation Tuning Suite for Linux.
TuxRigUp provides a comprehensive suite of utilities for configuring
and optimizing your Linux workstation.

%prep
%autosetup -n %{name}-%{version}

%build
%meson
%meson_build

%install
%meson_install

%check
desktop-file-validate %{buildroot}%{_datadir}/applications/com.github.yucefsourani.tuxrigup.desktop
appstream-util validate-relax --nonet %{buildroot}%{_datadir}/metainfo/com.github.yucefsourani.tuxrigup.appdata.xml || :

%files
%license LICENSE
%doc README.md
%{_bindir}/tuxrigup
%{_datadir}/applications/com.github.yucefsourani.tuxrigup.desktop
%{_datadir}/metainfo/com.github.yucefsourani.tuxrigup.appdata.xml
%{_datadir}/glib-2.0/schemas/com.github.yucefsourani.tuxrigup.gschema.xml
%{_datadir}/icons/hicolor/*/apps/com.github.yucefsourani.tuxrigup*.{png,svg}
%{_datadir}/pixmaps/com.github.yucefsourani.tuxrigup.png
%{_datadir}/tuxrigup/

%changelog
* Thu Jul 30 2026 Yucef Sourani <yucef.m.sourani@gmail.com> - 1.50-1
- Initial release.
