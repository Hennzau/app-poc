install-ubuntu:
    sudo apt-get install mold clang
    sudo apt-get install git-lfs

rustup:
    rustup component add rustc-codegen-cranelift-preview --toolchain nightly
    rustup component add rust-analyzer

fix:
    cargo fix --allow-dirty --allow-staged

run:
    cargo run

release:
    cargo run --release

panes:
    cargo run --example panes

editor:
    cargo run --example editor

git message:
    git add "*" ".*"
    git commit -am "{{message}}"

push:
    git push

pull:
    git pull

rebase count:
    git rebase -i HEAD~{{count}}
    git push --force
