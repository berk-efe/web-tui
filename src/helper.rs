use ratatui::{self, text::Span};

pub const FIRST_BOOT_TEXT_LIST: [&str; 13] = [
    ":: running early hook [udev]",
    "Starting systemd-udevd version 257.7-1-arch",
    ":: running early hook [archiso_pxe_nbd]",
    ":: Triggering uevents...",
    ":: running early hook [memdisk]",
    ":: running early hook [archiso_loop_mnt]",
    ":: running early hook [archiso_pxe_common]",
    ":: running early hook [archiso_pxe_nbd]",
    ":: running early hook [archiso_pxe_http]",
    ":: running early hook [archiso_pxe_nfs]",
    ":: Mounting '/dev/sda1' to '/run/archiso/bootmnt'",
    ":: Device '/dev/sda1' mounted successfully",
    ":: Copying rootfs image to RAM...",

];

pub const SECOND_BOOT_TEXT_LIST: [&str; 4] = [
    ":: Mounting '/dev/loop0' to '/run/archiso/airootfs'",
    ":: Device '/dev/loop0' mounted successfully",
    ":: running late hook [archiso_pxe_common]",
    ":: running cleanup hook [udev]",
];


