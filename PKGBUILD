pkgname=swb021bk_driver
pkgver=0.1.0
pkgrel=2
arch=('x86_64')

options=(!debug !strip)

build() {
    cargo build --release --bin $pkgname
}

package() {
    echo $startdir
    echo $pkgdir
    install -Dm755 \
        "$startdir/target/release/$pkgname" \
        "$pkgdir/usr/bin/$pkgname"

    install -Dm644 \
        "$startdir/$pkgname.desktop" \
        "$pkgdir/usr/share/applications/$pkgname.desktop"

    install -Dm644 \
        "$startdir/assets/idle.png" \
        "$pkgdir/usr/share/icons/hicolor/32x32/apps/$pkgname.png"
}
