#!/usr/bin/env bash
#
# Builds known downstream projects against local nexissource
#

set -e
cd "$(dirname "$0")"/..
source ci/_
source scripts/read-cargo-variable.sh

nexis_ver=$(readCargoVariable version sdk/Cargo.toml)
nexis_dir=$PWD
cargo="$nexis_dir"/cargo
cargo_build_bpf="$nexis_dir"/cargo-build-bpf
cargo_test_bpf="$nexis_dir"/cargo-test-bpf

mkdir -p target/downstream-projects
cd target/downstream-projects

update_nexis_dependencies() {
  declare tomls=()
  while IFS='' read -r line; do tomls+=("$line"); done < <(find "$1" -name Cargo.toml)

  sed -i -e "s#\(nexis-program = \"\)[^\"]*\(\"\)#\1=$nexis_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(nexis-program-test = \"\)[^\"]*\(\"\)#\1=$nexis_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(nexis-sdk = \"\).*\(\"\)#\1=$nexis_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(nexis-sdk = { version = \"\)[^\"]*\(\"\)#\1=$nexis_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(nexis-client = \"\)[^\"]*\(\"\)#\1=$nexis_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(nexis-client = { version = \"\)[^\"]*\(\"\)#\1=$nexis_ver\2#g" "${tomls[@]}" || return $?
}

patch_crates_io() {
  cat >> "$1" <<EOF
[patch.crates-io]
nexis-client = { path = "$nexis_dir/client" }
nexis-program = { path = "$nexis_dir/sdk/program" }
nexis-program-test = { path = "$nexis_dir/program-test" }
nexis-sdk = { path = "$nexis_dir/sdk" }
EOF
}

example_helloworld() {
  (
    set -x
    rm -rf example-helloworld
    git clone https://github.com/nexis-labs/example-helloworld.git
    cd example-helloworld

    update_nexis_dependencies src/program-rust
    patch_crates_io src/program-rust/Cargo.toml
    echo "[workspace]" >> src/program-rust/Cargo.toml

    $cargo_build_bpf \
      --manifest-path src/program-rust/Cargo.toml

    # TODO: Build src/program-c/...
  )
}

spl() {
  (
    set -x
    rm -rf spl
    git clone https://github.com/nexis-labs/nexis-program-library.git spl
    cd spl

    ./patch.crates-io.sh "$nexis_dir"

    $cargo build
    $cargo test
    $cargo_build_bpf
    $cargo_test_bpf
  )
}

serum_dex() {
  (
    set -x
    rm -rf serum-dex
    git clone https://github.com/project-serum/serum-dex.git
    cd serum-dex

    update_nexis_dependencies .
    patch_crates_io Cargo.toml
    patch_crates_io dex/Cargo.toml
    cat >> dex/Cargo.toml <<EOF
[workspace]
exclude = [
    "crank",
    "permissioned",
]
EOF
    $cargo build

    $cargo_build_bpf \
      --manifest-path dex/Cargo.toml --no-default-features --features program

    $cargo test \
      --manifest-path dex/Cargo.toml --no-default-features --features program
  )
}


_ example_helloworld
_ serum_dex
