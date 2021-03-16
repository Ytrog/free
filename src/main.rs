//! # Free
//! A simple tool to see how much disk space is left on your system

use std::path::Path;
use sysinfo::*;

/// format the size to bytes, KB, MB or GB
/// 
/// # Examples
/// ```
/// // 4.2 GB
/// let x = 4_200_000_000;
/// let actual = format_size(x);
/// let expected = String::new("4.2 GB");
/// 
/// assert_eq!(expected, actual);
/// ```
fn format_size(size: u64) -> String {
    if size > 1000_000_000 {
        // GB
        format!("{}.{} GB", size / 1000_000_000, size / 100_000_000 % 10)
    } else if size > 1000_000 {
        // MB
        format!("{}.{} MB", size / 1000_000, size / 100_000 % 10)
    } else if size > 1000 {
        // KB
        format!("{}.{} KB", size / 1000, size / 100 % 10)
    } else {
        size.to_string()
    }
}

/// print the disk info to output
fn print_disk(mount: &Path, total: u64, free: u64) -> () {
    println!(
        "{} total:{} free: {}",
        mount.display(),
        format_size(total),
        format_size(free)
    );
}

/// main function
fn main() {
    let mut s = System::new();
    s.refresh_disks_list();
    let disks = s.get_disks();
    for d in disks {
        let mount = d.get_mount_point();
        let total = d.get_total_space();
        let free = d.get_available_space();

        print_disk(mount, total, free);
    }
}
