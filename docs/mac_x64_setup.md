# Mac x64 Setup

Installing MacOS on non-apple hardware is an **explicit violation of the EULA**. I highly recommend you purchase apple hardware. If you insist on doing it anyways, do at your own risk.

Quickemu + Quickgui + MacOS Monterey.

[YouTube Tutorial](https://www.youtube.com/watch?v=Qa6y_CiyAMA).

Note that if you run macOS Installer and it completes but boots back, rerunning it and going to recovery, and then pressing the apple icon on the top left and selecting "restart", you should be able to re-run macOS Installer, or your specified disk will appear on it's own.

```sh
xcode-select --install

# Install homebrew https://brew.sh/
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

sudo trimforce enable
```

In Settings > Sharing, enable Remote Login and File Sharing.

```sh
sudo modprobe nbd max_part=8
sudo qemu-nbd --connect=/dev/nbd0 ~/Downloads/macos-monterey/OpenCore.qcow2
sudo mkdir -p /mnt/opencore
sudo mount /dev/nbd0p1 /mnt/opencore

sudo nano /mnt/opencore/EFI/OC/config.plist
# Find: <key>Timeout</key><integer>5</integer>
# Change: <key>Timeout</key><integer>0</integer>
# Find: <key>ShowPicker</key><true/>
# Change: <key>ShowPicker</key><false/>

# Unmount and disconnect
sudo umount /mnt/opencore
sudo qemu-nbd --disconnect /dev/nbd0
```

```sh
qemu-system-x86_64 \
-machine q35,accel=kvm \
-device isa-applesmc,osk=ourhardworkbythesewordsguardedpleasedontsteal\(c\)AppleComputerInc \
-cpu Haswell-v2,vendor=GenuineIntel,vmware-cpuid-freq=on \
-smp 4 \
-m 16G \
-drive if=pflash,format=raw,unit=0,file=$HOME/Downloads/macos-monterey/OVMF_CODE.fd,readonly=on \
-drive if=pflash,format=raw,unit=1,file=$HOME/Downloads/macos-monterey/OVMF_VARS-1920x1080.fd \
-device ahci,id=ahci \
-device ide-hd,bus=ahci.0,drive=BootLoader,bootindex=0 \
-drive id=BootLoader,if=none,format=qcow2,file=$HOME/Downloads/macos-monterey/OpenCore.qcow2 \
-device virtio-blk-pci,drive=SystemDisk \
-drive id=SystemDisk,if=none,format=qcow2,file=$HOME/Downloads/macos-monterey/disk.qcow2 \
-netdev user,id=net0,hostfwd=tcp::2224-:22 \
-device virtio-net,netdev=net0 \
-device qemu-xhci,id=input \
-device usb-kbd,bus=input.0 \
-device usb-tablet,bus=input.0 \
-k en-us \
-vga virtio \
-display gtk \
-serial stdio
```
