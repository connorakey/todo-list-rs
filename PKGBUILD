# Maintainer: Connor Akey <connor@proton.me>
pkgname=todo-list-rs
pkgver=1.0.0
pkgrel=1
pkgdesc="A simple CLI todo list written in Rust"
arch=('x86_64')
url="https://github.com/connorakey/todo-list-rs"
license=('GPL')
depends=('rust')
makedepends=('cargo')
source=("https://github.com/connorakey/todo-list-rs/archive/refs/tags/v${pkgver}.zip")
sha256sums=('8d2a8b34cb2bed43f011cdf76bbbbb1957b9ae61ae8702db359e2ae80dc30d0a')

build() {
    cd "${srcdir}/todo-list-rs-${pkgver}"
    cargo build --release
}

package() {
    cd "${srcdir}/todo-list-rs-${pkgver}"
    install -Dm755 "target/release/todo_list" "${pkgdir}/usr/bin/todo"
}
