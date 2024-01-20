#!/usr/bin/env bash

set -ex

rm -rfv src
mkdir -p src
pushd src

cargo run --manifest-path ../zbus/zbus_xmlgen/Cargo.toml --release -- \
    ../PackageKit/src/org.freedesktop.PackageKit.xml
cargo run --manifest-path ../zbus/zbus_xmlgen/Cargo.toml --release -- \
    ../PackageKit/src/org.freedesktop.PackageKit.Transaction.xml

for file in *.rs
do
    name="$(basename $file .rs)"
    echo "pub mod ${name};" >> lib.rs
done
echo "pub use zbus;" >> lib.rs

popd
