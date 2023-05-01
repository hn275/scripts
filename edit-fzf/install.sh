echo "Building bin"
cargo build --release && \
    printf "\nSymlink to bin path\n" && \
    sudo ln -Pf $HOME/scripts/edit-fzf/target/release/edit-fzf $HOME/scripts/bin && \
    printf "\tOK\n"
