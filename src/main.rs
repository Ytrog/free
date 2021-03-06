//! # Free
//! A simple tool to see how much disk space is left on your system

use std::path::Path;
use sysinfo::{DiskExt, System, SystemExt};

/// format the size to bytes, KB, MB or GB
///
/// # Examples
/// ```
/// // 4.2 GB
/// let x = 4_200_000_000;
/// let actual = format_size(x);
/// let expected = String::from("4.2 GB");
///
/// assert_eq!(expected, actual);
/// ```
fn format_size(size: u64) -> String {
    if size > 1_000_000_000 {
        // GB
        format!("{}.{} GB", size / 1_000_000_000, size / 100_000_000 % 10)
    } else if size > 1_000_000 {
        // MB
        format!("{}.{} MB", size / 1_000_000, size / 100_000 % 10)
    } else if size > 1000 {
        // KB
        format!("{}.{} KB", size / 1000, size / 100 % 10)
    } else {
        format!("{} B", size.to_string())
    }
}

/// print the disk info to output
fn print_disk(mount: &Path, total: u64, free: u64) {
    println!(
        "{} total:{} free: {}",
        mount.display(),
        format_size(total),
        format_size(free)
    );
}

/// main function
fn main() {
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    let mut s = System::new();
    s.refresh_disks_list();
    let disks = s.get_disks();
    println!("dfree v{}", VERSION);
    for d in disks {
        let mount = d.get_mount_point();
        let total = d.get_total_space();
        let free = d.get_available_space();

        print_disk(mount, total, free);
    }
}

#[cfg(test)]
mod tests {

    use super::format_size;

    #[test]
    fn gb_format_correct() {
        // 4.2 GB
        let x = 4_200_000_000;
        let actual = format_size(x);
        let expected = String::from("4.2 GB");

        assert_eq!(expected, actual);
    }

    #[test]
    fn mb_format_correct() {
        let x = 4_200_000;
        let actual = format_size(x);
        let expected = String::from("4.2 MB");

        assert_eq!(expected, actual);
    }

    #[test]
    fn kb_format_correct() {
        let x = 4_200;
        let actual = format_size(x);
        let expected = String::from("4.2 KB");

        assert_eq!(expected, actual);
    }

    #[test]
    fn b_format_correct() {
        let x = 42;
        let actual = format_size(x);
        let expected = String::from("42 B");

        assert_eq!(expected, actual);
    }

    #[test]
    fn zero_correct() {
        let x = 0;
        let actual = format_size(x);
        let expected = String::from("0 B");

        assert_eq!(expected, actual);
    }

    #[test]
    fn max_correct() {
        let x = u64::MAX;
        let actual = format_size(x);

        assert!(actual.contains("GB"));
    }
}
