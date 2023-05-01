echo "Building bin"
cargo build --release && \
    printf "\nSymlink to bin path\n" && \
    sudo ln -Pf $HOME/scripts/edit/target/release/edit $HOME/scripts/bin && \
    printf "\tOK\n"
