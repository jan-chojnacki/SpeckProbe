#!/usr/bin/env bash
# shellcheck disable=SC2034

iso_name="archbtw"
iso_label="ARCHBTW"
iso_publisher="archbench <https://example.org>"
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
  ["/root/.zlogin"]="0:0:644"
  ["/root/tune.sh"]="0:0:755"
  ["/root/activate-rust.sh"]="0:0:755"
)