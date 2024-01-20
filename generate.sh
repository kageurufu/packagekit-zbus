#!/usr/bin/env bash

set -ex

pushd packagekit-gir-files
rm -fv *.gir
cp -v /usr/share/gir-1.0/PackageKitGlib-1.0.gir .
popd

# Packages to generate, listed in dependency order
sys_pkgs=(
    package-kit-glib-sys
)

for pkg in "${sys_pkgs[@]}"
do
    # Uncomment to rebuild all automatically generated files
    #rm -rfv "${pkg}"
    if [ ! -d "${pkg}" ]
    then
        cargo run --release --manifest-path gir/Cargo.toml -- \
            --config "${pkg}.toml" \
            --girs-directories packagekit-gir-files \
            --girs-directories gir-files
        if [ -f "${pkg}.patch" ]
        then
            pushd "${pkg}"
            patch -p1 < "../${pkg}.patch"
            popd
        fi
    fi
    cargo build --release --manifest-path "${pkg}/Cargo.toml" --all-features
done

rust_pkgs=(
    packagekit
)

for pkg in "${rust_pkgs[@]}"
do
    rm -rfv "${pkg}/comments.md" "${pkg}/src/auto"
    cargo run --release --manifest-path gir/Cargo.toml -- \
        --config "${pkg}/Gir.toml" \
        --girs-directories packagekit-gir-files \
        --girs-directories gir-files
    cargo run --release --manifest-path gir/Cargo.toml -- \
        --config "${pkg}/Gir.toml" \
        --girs-directories packagekit-gir-files \
        --girs-directories gir-files \
        --mode doc \
        --doc-target-path "comments.md"
    rustdoc-stripper --regenerate --comment-file "${pkg}/comments.md" --dir "${pkg}/src" --ignore-doc-commented
    if [ "${pkg}" != "gdesktop_enums" ]
    then
        cargo run --release --manifest-path gir/Cargo.toml -- \
            --config "${pkg}/Gir.toml" \
            --girs-directories packagekit-gir-files \
            --girs-directories gir-files \
            --mode not_bound
    fi
    cargo build --release --manifest-path "${pkg}/Cargo.toml" --all-features
    cargo doc --manifest-path "${pkg}/Cargo.toml" --all-features
done
