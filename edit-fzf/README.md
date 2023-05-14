# Edit-fzf

term cli to help me hop around my files

## Installation

1. Make sure [fuzzy finder](https://github.com/junegunn/fzf) is installed.

2. Compile the code

```sh
cargo build --release
```

The binary is compiled at `./target/release/edit-fzf`, this location is needed
in step 4.

3. create a `locations.txt` at `$XDG_CONFIG_HOME/edit` (this is likely going
   to be at `$HOME/.config/`). Specify inside this file all the directory you
   want.

   - For example, if the full path is `$HOME/scripts` then an entry
     for this location would be `scripts`.

4. create a function in `$HOME/.zshrc`:

```sh
edit() {
  FILES=$("${HOME}/path/to/edit-fzf")
  CHOICE=$(echo "$FILES" | fzf)

  if [[ "$CHOICE" ]];then
    cd "$HOME/$CHOICE"
  fi
}
```
