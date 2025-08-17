# Maintainer: Connor Akey <connor@example.com>
pkgname=todo-list-rs
pkgver=1.0.0
pkgrel=1
pkgdesc="A simple CLI todo list written in Rust"
arch=('x86_64')
url="https://github.com/connorakey/todo-list-rs"
license=('MIT')
depends=('rust')
makedepends=('cargo')
source=("https://github.com/connorakey/todo-list-rs/archive/refs/tags/v${pkgver}.zip")
sha256sums=('SKIP')  # Replace with actual sha256sum if you want integrity check

build() {
    cd "${srcdir}/todo-list-rs-${pkgver}"
    cargo build --release
}

package() {
    cd "${srcdir}/todo-list-rs-${pkgver}"
    install -Dm755 "target/release/todo_list" "${pkgdir}/usr/bin/todo_list"
}
