use std::process::Command;

fn main() {
    let mut cmd = Command::new("cargo");
    cmd.arg("build")
        .arg("--release")
        .arg("--target=x86_64-unknown-uefi");

    let exit_status = cmd.status().expect("failed to execute cargo build");
    if !exit_status.success() {
        panic!("cargo build failed");
    }

    let out_dir = format!("target/{}/release", "x86_64-unknown-uefi");

    // Use the bootloader crate to create the bootable image
    let mut cmd = Command::new("x86_64-unknown-uefi-bootloader");
    cmd.arg("--kernel")
        .arg(format!("{}/{}", out_dir, "kernel_with_bootloader.efi"))
        .arg("--bootloader")
        .arg("bootloader.efi")
        .arg("--output")
        .arg("os.img");

    let exit_status = cmd.status().expect("failed to execute bootloader command");
    if !exit_status.success() {
        panic!("bootloader command failed");
    }

    println!("Bootable image created: os.img");
}