# Ask for SMB server, username, password
read -p "Enter SMB server IP (EX: 192.168.X.X): " SERVER
read -p "Enter SMB username (EX: root): " USERNAME
read -s -p "Enter SMB password (EX: root_pwd): " PASSWORD
echo ""

# Ask for mount directory
read -p "Enter mount directory (default: /mnt, EX: /mnt/ServerName): " BASE_MOUNT
# Use default if no input
if [ -z "$BASE_MOUNT" ]; then
    BASE_MOUNT="/mnt"
fi

# Ask for folders to mount
read -p "Enter folders to mount (space separated, EX: movies backups music): " -a FOLDERS


# Loop through each folder and mount
for FOLDER in "${FOLDERS[@]}"; do
    MOUNT_POINT="${BASE_MOUNT}/${FOLDER}"
    SMB_PATH="//${SERVER}/${FOLDER}"

    # Create mount point directory if it doesn't exist
    if [ ! -d "$MOUNT_POINT" ]; then
        sudo mkdir -p "$MOUNT_POINT"
    fi

    # Mount the SMB share
    echo "Mounting ${SMB_PATH} to ${MOUNT_POINT}..."
    sudo mount -t cifs "$SMB_PATH" "$MOUNT_POINT" -o username="$USERNAME",password="$PASSWORD",rw,uid=$(id -u),gid=$(id -g)
done

echo "All folders mounted successfully."
