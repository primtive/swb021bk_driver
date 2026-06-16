pkgname=swb021bk-driver-git
pkgver=r2.60f6dda
pkgrel=1
source=("$pkgname::git+https://github.com/primtive/swb021bk_driver.git")
sha256sums=('SKIP')
makedepends=('cargo' 'git')
arch=('x86_64')
pkgdesc="small rust driver for swb021bk tablet"
url="https://github.com/primtive/swb021bk_driver"
license=('MIT')
options=(!debug !strip)
bin_name=swb021bk_driver

pkgver() {
    cd "$pkgname"
    printf "r%s.%s" "$(git rev-list --count HEAD)" "$(git rev-parse --short HEAD)"
}

prepare() {
    cd "$pkgname"
    cargo fetch --locked --target "$(rustc -vV | sed -n 's/host: //p')"
}

build() {
    cd "$pkgname"
    export RUSTUP_TOOLCHAIN=stable
    cargo build --frozen --release --bin $bin_name
}

package() {
    cd "$pkgname"
    install -Dm0755 "target/release/$bin_name" "$pkgdir/usr/bin/$bin_name"
    install -Dm644 "$bin_name.desktop" "$pkgdir/usr/share/applications/$bin_name.desktop"
    install -Dm644 "assets/idle.png" "$pkgdir/usr/share/icons/hicolor/32x32/apps/$bin_name.png"
}
