# Maintainer: dddevid <your-email@example.com>
pkgname=rfetch
pkgver=1.0.0
pkgrel=1
pkgdesc="A fast, customizable system information tool written in Rust with full iOS support"
arch=('x86_64' 'aarch64')
url="https://github.com/dddevid/rFetch"
license=('MIT')
depends=()
makedepends=('rust' 'cargo')
source=("$pkgname-$pkgver.tar.gz::https://github.com/dddevid/rFetch/archive/v$pkgver.tar.gz")
sha256sums=('SKIP')

build() {
    cd "$srcdir/rFetch-$pkgver"
    cargo build --release --locked
}

check() {
    cd "$srcdir/rFetch-$pkgver"
    cargo test --release --locked
}

package() {
    cd "$srcdir/rFetch-$pkgver"
    
    # Install binary
    install -Dm755 "target/release/rfetch" "$pkgdir/usr/bin/rfetch"
    
    # Install license
    install -Dm644 "LICENSE" "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
    
    # Install documentation
    install -Dm644 "README.md" "$pkgdir/usr/share/doc/$pkgname/README.md"
    
    # Install example config
    install -Dm644 "config.example.toml" "$pkgdir/usr/share/doc/$pkgname/config.example.toml"
    
    # Install theme creator
    install -dm755 "$pkgdir/usr/share/$pkgname/theme-creator"
    cp -r theme-creator/* "$pkgdir/usr/share/$pkgname/theme-creator/"
}