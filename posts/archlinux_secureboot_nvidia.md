---
title: "Archlinux with secureboot on a nvidia machine"
date: 2022-03-26T19:12:17+05:30
tags: ['arch', 'linux','secureboot', 'nvidia']
draft: false
---

Many moons ago, when I first started using archlinux with secure boot, I always dreamed of running it on my desktop, a machine with an Nvidia graphics card. While the desktop has been leagues faster than my laptop, secure boot never worked with it. Enabling secure boot would result in a black screen with no output forcing me every time to reset the CMOS.

For a while, I tried solving the problem, asking on Reddit and even reaching out to my motherboard and graphics card manufacturers, who pointed the blame at each other. But today, I made a breakthrough. I figured out why it happened and am happy to say I am running Archlinux with secure boot using EFISTUBS with an Nvidia graphics card.

To start with, we need to shame me a little. I misinterpreted some stuff hard and goofed up. So Nvidia being the annoying people they are, require your signature DB to contain Microsoft's secure boot keys, (these can be grabed [here](https://www.microsoft.com/pkiops/certs/MicWinProPCA2011_2011-10-19.crt) & [here](https://www.microsoft.com/pkiops/certs/MicCorUEFCA2011_2011-06-27.crt)). The arch wiki has a [section](https://wiki.archlinux.org/title/Unified_Extensible_Firmware_Interface/Secure_Boot#Microsoft_Windows) on how to use them. 

By keeping these around and adding your key, everything works. Some graphics cards allow one to modify the bios not to depend on Microsoft's keys. I would not recommend this due to the dangers of BIOS modding. Overall it's pretty much a bog-standard secure boot setup. In tandem with this, I use EFISTUBs which allows firmware to load the kernel like any other EFI stub. I use systemd instead of direct to the firmware since I prefer having the ability to choose Kernels without dropping them into the firmware.

The only difference when using EFISTUBS I have found is that a custom Pacman hook is required. I based my hook on one used to automatically sign kernels with MOKUTIL.

```bash
[Trigger]
Operation = Install
Operation = Upgrade
Type = Package
Target = linux
Target = linux-hardened

[Action]
Description = Signing kernel with Db Key for Secure Boot
When = PostTransaction
Exec = /usr/bin/find /boot/EFI/Linux -maxdepth 1 -name 'archlinux-*' -exec /usr/bin/sh -c 'if ! /usr/bin/sbverify --list {} 2>/dev/null | /usr/bin/grep -q "signature certificates"; then /usr/bin/sbsign --key /etc/secureboot/DB.key --cert /etc/secureboot/DB.crt --output {} {}; fi' ;
Depends = sbsigntools
Depends = findutils
Depends = grep
```
You can feel free to modify and use this hook to your needs. As for generating EFISTUBs. This [blog post](https://linderud.dev/blog/mkinitcpio-v31-and-uefi-stubs/) by Morten Linderud aka foxboron explains how to get started pretty well. I hope this post helps others in a similar situation to me.

— This is nullrquest signing off