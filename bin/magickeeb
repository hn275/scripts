BUILTIN_SLAVE_ID=13
BUILTIN_MASTER_ID=3

function reset() {
    xset r rate 200 45
    echo "remapping key"
    xmodmap $HOME/.Xmodmap \
        && echo "[OK]"
}

function connect() {
    sudo systemctl start bluetooth.service \
        && bluetoothctl connect 70:F0:87:1E:38:A5 \
        && reset

    echo "detaching internal keyboard"
    xinput float ${BUILTIN_SLAVE_ID} && echo "[OK]"
}

function disconnect() {
    echo "reattaching internal keyboard"
    xinput reattach ${BUILTIN_SLAVE_ID} ${BUILTIN_MASTER_ID} \
        && echo "[OK]" \
        && echo "Disconnecting" \
        && bluetoothctl disconnect 70:F0:87:1E:38:A5 \
        && echo "[OK]" \
        && reset
}

case $1 in
    --connect|-c)
        connect
        ;;
    --disconnect|-d)
        disconnect
        ;;
    *)
        echo "usage: [--connect|-c, --disconnect|-d]"
        ;;
esac
