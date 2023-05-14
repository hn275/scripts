BUILTIN_SLAVE_ID=13
BUILTIN_MASTER_ID=3

function connect() {
    sudo systemctl start bluetooth.service \
        && bluetoothctl connect 70:F0:87:1E:38:A5 

    echo "detaching internal keyboard"
    xinput float ${BUILTIN_SLAVE_ID} && echo "[OK]"
}

function disconnect() {
    echo "reattaching internal keyboard"
    xinput reattach ${BUILTIN_SLAVE_ID} ${BUILTIN_MASTER_ID} \
        && echo "[OK]" \
        && echo "Disconnecting" \
        && bluetoothctl disconnect 70:F0:87:1E:38:A5 \
        && echo "[OK]" 
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
        exit 1
        ;;
esac
