#!/bin/bash
set -fexuo pipefail

if [ "${1:-}" ]; then
    day=$1
    mkdir -p "${day}/src"
    sed -i -re "s/^]/    \"${day}\",\n]/g" Cargo.toml
    echo -e 'fn main()\n{\n}'  > "${day}/src/main.rs"
    echo -e '\n#[cfg(test)]\nmod tests {\n    use super::*;\n\n    #[test]\n    fn test() {\n    }\n}' >> "${day}"/src/main.rs

    cat > "${day}/Cargo.toml" <<EOF
[package]
name = "${day}"
version = "0.1.0"
authors = ["Jason Woodward <woodwardj@jaos.org>"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
EOF
    touch "${day}/input"
    touch "${day}/test"
    git add Cargo.toml "${day}"/{Cargo.toml,input,test,src/main.rs}

fi
