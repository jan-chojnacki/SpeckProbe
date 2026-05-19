#!/usr/bin/env bash

iso_name="archbtw"
iso_label="ARCHBTW"
iso_application="Arch Benchmark Tooling Workspace"
iso_version="1.0.0"
install_dir="arch"
buildmodes=('iso')
bootmodes=('uefi.systemd-boot')
arch="x86_64"
pacman_conf="pacman.conf"
airootfs_image_type="squashfs"
airootfs_image_tool_options=('-comp' 'zstd' '-Xcompression-level' '19' '-b' '1M')
bootstrap_tarball_compression=('zstd' '-c' '-T0' '--auto-threads=logical' '--long' '-19')
file_permissions=(
  ["/etc/shadow"]="0:0:400"
  ["/root"]="0:0:750"
  ["/root/.bash_profile"]="0:0:644"
  ["/root/rust.sh"]="0:0:755"
  ["/root/tune.sh"]="0:0:755"
  ["/root/run.sh"]="0:0:755"
)