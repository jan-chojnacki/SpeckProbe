#!/bin/bash
set -euo pipefail

ISO="${1:-}"
DEV="${2:-}"

[[ -f "$ISO" ]] || { echo "ISO not found: $ISO"; exit 1; }
[[ -b "$DEV" ]] || { echo "not a block device: $DEV"; exit 1; }

REMOVABLE="$(cat /sys/block/$(basename "$DEV")/removable 2>/dev/null || echo 0)"
if [[ "$REMOVABLE" != "1" ]]; then
  echo "WARNING: $DEV is NOT removable (probably internal disk)."
  read -p "Type the device path again to confirm wipe ($DEV): " CONFIRM
  [[ "$CONFIRM" == "$DEV" ]] || { echo "aborted"; exit 1; }
fi

echo ">>> unmounting any partitions on $DEV"
sudo umount "${DEV}"* 2>/dev/null || true

echo ">>> writing ISO to $DEV"
sudo dd if="$ISO" of="$DEV" bs=4M conv=fsync status=progress oflag=direct
sudo sync

echo ">>> fixing GPT"
sudo sgdisk -e "$DEV"

echo ">>> creating DATA partition in free space"
NEXT=$(sudo sgdisk -p "$DEV" | awk '/^ +[0-9]+ / {n=$1} END {print n+1}')
sudo sgdisk -n "${NEXT}:0:0" -t "${NEXT}:8300" -c "${NEXT}:DATA" "$DEV"

sudo partprobe "$DEV"
sleep 1

if [[ "$DEV" == *nvme* || "$DEV" == *mmcblk* ]]; then
  PART="${DEV}p${NEXT}"
else
  PART="${DEV}${NEXT}"
fi

echo ">>> formatting $PART as ext4 (label=DATA)"
sudo mkfs.ext4 -L DATA -F "$PART"

sudo eject "$DEV" || true
echo ">>> done"