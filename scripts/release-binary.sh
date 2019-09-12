#!/bin/sh
SAY_PREFIX="$(basename $0): "
lightred() { echo -e "\033[1;31m$*\033[0m"; }
blue() { echo -e "\033[1;34m$*\033[0m"; }
dump() { echo "$SAY_PREFIX$*" >&2; }
say() { echo "$SAY_PREFIX$(blue "$*")" >&2; }
yell() { echo "$SAY_PREFIX$(lightred "$*")" >&2; }
die() {
	yell "$*"
	exit 111
}
try() { "$@" || die "cannot $*"; }
asuser() { sudo su - "$1" -c "${*:2}"; }
need_var() { test -n "${!1}" || die "$1 must be defined"; }
need_vars() { for var in "$@"; do need_var $var; done; }
has_bin() { which "$1" 2>&1 >/dev/null; }
need_exe() { has_bin "$1" || die "'$1' not found in PATH"; }
need_bin() { need_exe "$1"; }
strictmode() { set -eo pipefail; }
nostrictmode() { set +eo pipefail; }
say_var() { say "$1 = ${!1}"; }
say_vars() { for var in "$@"; do say_var $var; done; }
yell_var() { yell "$1 = ${!1}"; }
yell_vars() { for var in "$@"; do yell_var $var; done; }

export SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

VERSION=$(cat Cargo.toml | reser -f json | jp -u package.version)

need_var VERSION
say_var VERSION

yell "Aight?"
read RESPONSE
need_var RESPONSE
test "$(echo "$RESPONSE" | tr A-Z a-z)" = tight || break

linux_musl() {
	LINUX_MUSL=twitch-scraper-x86_64-linux-musl
	docker run --rm -it -v $PWD:/home/rust/src ekidd/rust-musl-builder:stable cargo build --release &&
		cp target/x86_64-unknown-linux-musl/release/twitch-scraper $LINUX_MUSL &&
		strip $LINUX_MUSL &&
		hub release create -e -m "Version v$VERSION" -m "$(sha256sum $LINUX_MUSL)" -a $LINUX_MUSL v$VERSION

}

linux_musl
