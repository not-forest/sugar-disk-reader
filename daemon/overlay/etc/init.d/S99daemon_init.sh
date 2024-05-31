#!/bin/sh
# DAEMON post-initialization script.

MOUNT_DIR="/mnt/disks"

# Starting the ADB server for on-target debugging.
adb start-server
adb devices -l

mkdir -p $MOUNT_DIR

# Gets the list of all disks, excluding the root filesystem.
disks=$(lsblk -dn -o NAME | grep -v "$(lsblk -dno NAME /)")

# Counter for mount points
counter=0

for disk in $disks; do
    # Get all partitions for the current disk
    partitions=$(lsblk -ln -o NAME /dev/$disk | grep -v "^$disk$")
    
    for partition in $partitions; do
        mount_point="$MOUNT_DIR/$partition"
        mkdir -p $mount_point
        
        mount /dev/$partition $mount_point
        
        if mountpoint -q $mount_point; then
            echo "Mounted /dev/$partition at $mount_point"
        else
            echo "Failed to mount /dev/$partition"
        fi
        
        counter=$((counter + 1))
    done
done

if [ $counter -eq 0 ]; then
    echo "No partitions found to mount."
else
    echo "$counter partitions mounted."
fi

# Daemon program.
/bin/daemon
