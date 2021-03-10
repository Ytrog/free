use std::path::Path;
use sysinfo::*;

/// format the size to bytes, KB, MB or GB
fn format_size(size: u64) -> String {
    if size > 1000_000_000 {
        // GB
        format!("{} GB", size / 1000_000_000)
    } else if size > 1000_000 {
        // MB
        format!("{} MB", size / 1000_000)
    } else if size > 1000 {
        // KB
        format!("{} KB", size / 1000)
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
